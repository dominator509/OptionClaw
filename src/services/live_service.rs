use std::path::{Path, PathBuf};

use crate::{
    alpaca::{AlpacaAccountStatus, AlpacaClient, AlpacaCredentials},
    config::{AppConfig, Provider, ProviderEnvironment},
    domain::{AccountSnapshot, OrderIntent, OrderSide, TradingMode},
    errors::{AppError, InputError, SecurityError},
    observability::{record_metric, MetricEvent},
    persistence::{append_audit, init_data_dir, AuditEvent, AuditEventType},
    risk::evaluate_order_intent,
    services::{
        derive_data_dir, load_json_file,
        research_service::{
            approval_path, current_unix_seconds, load_approval_artifact, validate_approval_artifact,
        },
        risk_service::OrderIntentFixture,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveCheckReport {
    pub config_path: PathBuf,
    pub provider: Provider,
    pub provider_environment: ProviderEnvironment,
    pub account_status: String,
    pub options_approved_level: u8,
    pub options_trading_level: u8,
    pub approval_fresh: bool,
    pub kill_switch_active: bool,
    pub approved: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveSubmitReport {
    pub config_path: PathBuf,
    pub order_intent_path: PathBuf,
    pub order_intent_id: String,
    pub provider_order_id: String,
    pub provider_status: String,
    pub submitted: bool,
}

pub fn live_check(config_path: impl AsRef<Path>) -> Result<LiveCheckReport, AppError> {
    let config_path = config_path.as_ref().to_path_buf();
    let config = AppConfig::load_from_path(&config_path)?;
    require_live_config(&config, &config_path)?;
    ensure_kill_switch_inactive(&config)?;
    let approval = load_and_validate_approval(&config, &config_path)?;
    let client = alpaca_client(&config)?;
    let status = client.account_status()?;
    ensure_options_capability(&status, &config_path)?;
    record_metric(MetricEvent::adapter_result("alpaca", "live_check", true));

    Ok(LiveCheckReport {
        config_path,
        provider: config.provider,
        provider_environment: config.provider_environment,
        account_status: status.status,
        options_approved_level: status.options_approved_level,
        options_trading_level: status.options_trading_level,
        approval_fresh: approval.approved,
        kill_switch_active: false,
        approved: true,
    })
}

pub fn live_submit(
    config_path: impl AsRef<Path>,
    order_intent_path: impl AsRef<Path>,
    confirm_live: bool,
) -> Result<LiveSubmitReport, AppError> {
    let config_path = config_path.as_ref().to_path_buf();
    let order_intent_path = order_intent_path.as_ref().to_path_buf();
    if !confirm_live {
        return Err(AppError::from(InputError::Invalid {
            path: order_intent_path,
            detail: "live submit requires --confirm-live".to_string(),
        }));
    }

    let config = AppConfig::load_from_path(&config_path)?;
    require_live_config(&config, &config_path)?;
    ensure_kill_switch_inactive(&config)?;
    let _approval = load_and_validate_approval(&config, &config_path)?;
    let client = alpaca_client(&config)?;
    let status = client.account_status()?;
    ensure_options_capability(&status, &config_path)?;
    let account = account_snapshot_from_status(&status)?;

    let fixture: OrderIntentFixture = load_json_file(&order_intent_path)?;
    let intent = fixture.to_domain()?;
    ensure_supported_live_intent(&intent, &order_intent_path)?;
    let limits = config.live_risk_limits()?;
    let decision = evaluate_order_intent(&intent, Some(&limits), &account, false);
    record_metric(MetricEvent::risk_decision(decision.is_accepted()));
    if !decision.is_accepted() {
        return Err(AppError::from(InputError::Invalid {
            path: order_intent_path,
            detail: format!("live risk gate rejected order: {decision}"),
        }));
    }

    let provider_report = client.submit_order(&intent)?;
    record_metric(MetricEvent::adapter_result("alpaca", "submit_order", true));
    append_live_audit(&config_path, &config, &intent, &provider_report.order_id)?;

    Ok(LiveSubmitReport {
        config_path,
        order_intent_path,
        order_intent_id: intent.id,
        provider_order_id: provider_report.order_id,
        provider_status: provider_report.status,
        submitted: true,
    })
}

fn require_live_config(config: &AppConfig, config_path: &Path) -> Result<(), AppError> {
    if config.trading_mode != TradingMode::Live {
        return Err(AppError::from(InputError::Invalid {
            path: config_path.to_path_buf(),
            detail: "live commands require trading_mode = \"live\"".to_string(),
        }));
    }
    config.validate_live_contract()
}

fn alpaca_client(config: &AppConfig) -> Result<AlpacaClient, AppError> {
    let base_url =
        config
            .alpaca_base_url
            .clone()
            .unwrap_or_else(|| match config.provider_environment {
                ProviderEnvironment::Live => "https://api.alpaca.markets".to_string(),
                ProviderEnvironment::Paper | ProviderEnvironment::Sandbox => {
                    "https://paper-api.alpaca.markets".to_string()
                }
            });
    Ok(AlpacaClient::new(base_url, AlpacaCredentials::from_env()?))
}

fn load_and_validate_approval(
    config: &AppConfig,
    config_path: &Path,
) -> Result<crate::services::research_service::LiveApprovalArtifact, AppError> {
    let path = approval_path(config, config_path);
    let artifact = load_approval_artifact(&path)?;
    validate_approval_artifact(config, &artifact, current_unix_seconds()?, &path)?;
    Ok(artifact)
}

fn ensure_kill_switch_inactive(config: &AppConfig) -> Result<(), AppError> {
    if let Some(path) = &config.kill_switch_file {
        if path.exists() {
            return Err(AppError::from(SecurityError::KillSwitchActive));
        }
    }
    Ok(())
}

fn ensure_options_capability(status: &AlpacaAccountStatus, path: &Path) -> Result<(), AppError> {
    if status.trading_blocked || !status.status.eq_ignore_ascii_case("active") {
        return Err(AppError::from(InputError::Invalid {
            path: path.to_path_buf(),
            detail: "Alpaca account is not active for trading".to_string(),
        }));
    }
    if status.options_approved_level < 2 || status.options_trading_level < 2 {
        return Err(AppError::from(InputError::Invalid {
            path: path.to_path_buf(),
            detail: "Alpaca account must have options approved/trading level 2".to_string(),
        }));
    }
    Ok(())
}

fn ensure_supported_live_intent(intent: &OrderIntent, path: &Path) -> Result<(), AppError> {
    if intent.mode != TradingMode::Live {
        return Err(AppError::from(InputError::Invalid {
            path: path.to_path_buf(),
            detail: "live submit requires an order intent with mode = live".to_string(),
        }));
    }
    if intent.side != OrderSide::Buy {
        return Err(AppError::from(InputError::Invalid {
            path: path.to_path_buf(),
            detail: "first live release permits only long calls and long puts".to_string(),
        }));
    }
    Ok(())
}

fn account_snapshot_from_status(status: &AlpacaAccountStatus) -> Result<AccountSnapshot, AppError> {
    Ok(AccountSnapshot::new(
        status.account_id.clone(),
        status.equity,
        status.buying_power,
        0,
        Vec::new(),
    )?)
}

fn append_live_audit(
    config_path: &Path,
    config: &AppConfig,
    intent: &OrderIntent,
    provider_order_id: &str,
) -> Result<(), AppError> {
    let data_dir = derive_data_dir(config_path);
    init_data_dir(&data_dir)?;
    let audit = AuditEvent::new(
        AuditEventType::StateWrite,
        config.trading_mode,
        Some(intent.id.clone()),
        Some("LIVE_SUBMITTED".to_string()),
        format!("provider_order_id={provider_order_id}"),
    )?;
    append_audit(data_dir.join("audit").join("events.jsonl"), &audit)?;
    Ok(())
}
