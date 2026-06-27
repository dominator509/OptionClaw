use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

use crate::{
    config::{stable_hex_hash, AppConfig},
    errors::{AppError, InputError, PersistenceError},
    services::{derive_data_dir, load_json_file},
};

pub const MIN_BACKTEST_NET_ROI_BPS: i32 = 2_500;
pub const MIN_FORWARD_PAPER_ROI_BPS: i32 = 800;
pub const MIN_PROFIT_FACTOR_BPS: u32 = 13_500;
pub const MAX_DRAWDOWN_BPS: u32 = 2_000;
pub const MIN_BACKTEST_TRADES: u32 = 200;
pub const MIN_FORWARD_PAPER_TRADES: u32 = 30;
pub const APPROVAL_TTL_SECONDS: u64 = 7 * 24 * 60 * 60;

#[derive(Debug, Clone, Deserialize)]
pub struct BacktestFixture {
    pub annualized_net_roi_bps: i32,
    pub forward_paper_roi_bps: i32,
    pub profit_factor_bps: u32,
    pub max_drawdown_bps: u32,
    pub backtest_trades: u32,
    pub forward_paper_trades: u32,
    pub risk_gate_bypasses: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResearchEvidence {
    pub strategy_id: String,
    pub risk_profile_id: String,
    pub config_hash: String,
    pub annualized_net_roi_bps: i32,
    pub forward_paper_roi_bps: i32,
    pub profit_factor_bps: u32,
    pub max_drawdown_bps: u32,
    pub backtest_trades: u32,
    pub forward_paper_trades: u32,
    pub risk_gate_bypasses: u32,
    pub generated_at_unix_seconds: u64,
    pub approved: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BacktestReport {
    pub report_path: PathBuf,
    pub evidence: ResearchEvidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LiveApprovalArtifact {
    pub approval_version: String,
    pub strategy_id: String,
    pub risk_profile_id: String,
    pub config_hash: String,
    pub generated_at_unix_seconds: u64,
    pub expires_at_unix_seconds: u64,
    pub approved: bool,
    pub signature: String,
    pub evidence: ResearchEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalReport {
    pub approval_path: PathBuf,
    pub artifact: LiveApprovalArtifact,
}

pub fn run_backtest(
    config_path: impl AsRef<Path>,
    fixtures: impl AsRef<Path>,
) -> Result<BacktestReport, AppError> {
    let config_path = config_path.as_ref().to_path_buf();
    let config = AppConfig::load_from_path(&config_path)?;
    let fixture_path = resolve_fixture_path(fixtures.as_ref());
    let fixture: BacktestFixture = load_json_file(&fixture_path)?;
    let generated_at_unix_seconds = current_unix_seconds()?;
    let evidence = ResearchEvidence {
        strategy_id: config.strategy_id.clone(),
        risk_profile_id: config.risk_profile_id.clone(),
        config_hash: config.approval_config_hash(),
        annualized_net_roi_bps: fixture.annualized_net_roi_bps,
        forward_paper_roi_bps: fixture.forward_paper_roi_bps,
        profit_factor_bps: fixture.profit_factor_bps,
        max_drawdown_bps: fixture.max_drawdown_bps,
        backtest_trades: fixture.backtest_trades,
        forward_paper_trades: fixture.forward_paper_trades,
        risk_gate_bypasses: fixture.risk_gate_bypasses,
        generated_at_unix_seconds,
        approved: false,
    };
    validate_aggressive_growth(&evidence, &fixture_path)?;

    let mut approved_evidence = evidence;
    approved_evidence.approved = true;
    let report_path = derive_data_dir(&config_path)
        .join("research")
        .join("backtest-report.json");
    write_json(&report_path, &approved_evidence)?;

    Ok(BacktestReport {
        report_path,
        evidence: approved_evidence,
    })
}

pub fn approve_research(
    config_path: impl AsRef<Path>,
    report_path: impl AsRef<Path>,
) -> Result<ApprovalReport, AppError> {
    let config_path = config_path.as_ref().to_path_buf();
    let report_path = report_path.as_ref().to_path_buf();
    let config = AppConfig::load_from_path(&config_path)?;
    let evidence: ResearchEvidence = load_json_file(&report_path)?;
    validate_aggressive_growth(&evidence, &report_path)?;
    if evidence.config_hash != config.approval_config_hash() {
        return Err(AppError::from(InputError::Invalid {
            path: report_path,
            detail: "ROI evidence config hash does not match current live config".to_string(),
        }));
    }

    let generated_at_unix_seconds = current_unix_seconds()?;
    let expires_at_unix_seconds = generated_at_unix_seconds + APPROVAL_TTL_SECONDS;
    let artifact = LiveApprovalArtifact {
        approval_version: "optionclaw-live-approval-v1".to_string(),
        strategy_id: config.strategy_id.clone(),
        risk_profile_id: config.risk_profile_id.clone(),
        config_hash: evidence.config_hash.clone(),
        generated_at_unix_seconds,
        expires_at_unix_seconds,
        approved: true,
        signature: approval_signature(
            &config.strategy_id,
            &config.risk_profile_id,
            &evidence.config_hash,
            generated_at_unix_seconds,
            expires_at_unix_seconds,
        ),
        evidence,
    };

    let approval_path = approval_path(&config, &config_path);
    write_json(&approval_path, &artifact)?;
    Ok(ApprovalReport {
        approval_path,
        artifact,
    })
}

pub fn load_approval_artifact(path: impl AsRef<Path>) -> Result<LiveApprovalArtifact, AppError> {
    load_json_file(path)
}

pub fn validate_approval_artifact(
    config: &AppConfig,
    artifact: &LiveApprovalArtifact,
    now_unix_seconds: u64,
    path: impl AsRef<Path>,
) -> Result<(), AppError> {
    let path = path.as_ref().to_path_buf();
    if !artifact.approved {
        return Err(invalid(path, "live approval artifact is not approved"));
    }
    if artifact.config_hash != config.approval_config_hash() {
        return Err(invalid(
            path,
            "live approval artifact config hash does not match current config",
        ));
    }
    if artifact.strategy_id != config.strategy_id
        || artifact.risk_profile_id != config.risk_profile_id
    {
        return Err(invalid(
            path,
            "live approval artifact strategy or risk profile does not match current config",
        ));
    }
    if artifact.expires_at_unix_seconds < now_unix_seconds {
        return Err(invalid(path, "live approval artifact is stale"));
    }
    if artifact.evidence.strategy_id != artifact.strategy_id
        || artifact.evidence.risk_profile_id != artifact.risk_profile_id
        || artifact.evidence.config_hash != artifact.config_hash
    {
        return Err(invalid(
            path,
            "live approval artifact embedded evidence does not match artifact identity",
        ));
    }
    let expected_signature = approval_signature(
        &artifact.strategy_id,
        &artifact.risk_profile_id,
        &artifact.config_hash,
        artifact.generated_at_unix_seconds,
        artifact.expires_at_unix_seconds,
    );
    if artifact.signature != expected_signature {
        return Err(invalid(path, "live approval artifact signature is invalid"));
    }
    validate_aggressive_growth(&artifact.evidence, path)?;
    Ok(())
}

pub fn approval_path(config: &AppConfig, config_path: impl AsRef<Path>) -> PathBuf {
    config.approval_artifact.clone().unwrap_or_else(|| {
        derive_data_dir(config_path)
            .join("live")
            .join("live-approval.json")
    })
}

pub fn current_unix_seconds() -> Result<u64, AppError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|source| {
            AppError::from(InputError::Invalid {
                path: PathBuf::from("system-time"),
                detail: source.to_string(),
            })
        })
}

pub fn validate_aggressive_growth(
    evidence: &ResearchEvidence,
    path: impl AsRef<Path>,
) -> Result<(), AppError> {
    let path = path.as_ref().to_path_buf();
    if evidence.annualized_net_roi_bps < MIN_BACKTEST_NET_ROI_BPS {
        return Err(invalid(
            path,
            "annualized net ROI is below aggressive-growth gate",
        ));
    }
    if evidence.forward_paper_roi_bps < MIN_FORWARD_PAPER_ROI_BPS {
        return Err(invalid(
            path,
            "forward paper ROI is below aggressive-growth gate",
        ));
    }
    if evidence.profit_factor_bps < MIN_PROFIT_FACTOR_BPS {
        return Err(invalid(
            path,
            "profit factor is below aggressive-growth gate",
        ));
    }
    if evidence.max_drawdown_bps > MAX_DRAWDOWN_BPS {
        return Err(invalid(path, "max drawdown exceeds aggressive-growth gate"));
    }
    if evidence.backtest_trades < MIN_BACKTEST_TRADES {
        return Err(invalid(path, "not enough backtest trades for approval"));
    }
    if evidence.forward_paper_trades < MIN_FORWARD_PAPER_TRADES {
        return Err(invalid(
            path,
            "not enough forward-paper trades for approval",
        ));
    }
    if evidence.risk_gate_bypasses != 0 {
        return Err(invalid(path, "risk gate bypasses must be zero"));
    }
    Ok(())
}

fn resolve_fixture_path(fixtures: &Path) -> PathBuf {
    if fixtures.is_dir() {
        fixtures.join("aggressive_growth.json")
    } else {
        fixtures.to_path_buf()
    }
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), AppError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| {
            AppError::from(PersistenceError::Unavailable {
                path: parent.to_path_buf(),
                source: Box::new(source),
            })
        })?;
    }
    let contents = serde_json::to_string_pretty(value).map_err(|source| {
        AppError::from(InputError::Invalid {
            path: path.to_path_buf(),
            detail: source.to_string(),
        })
    })?;
    fs::write(path, contents).map_err(|source| {
        AppError::from(PersistenceError::Unavailable {
            path: path.to_path_buf(),
            source: Box::new(source),
        })
    })
}

fn approval_signature(
    strategy_id: &str,
    risk_profile_id: &str,
    config_hash: &str,
    generated_at_unix_seconds: u64,
    expires_at_unix_seconds: u64,
) -> String {
    stable_hex_hash(&format!(
        "{strategy_id}:{risk_profile_id}:{config_hash}:{generated_at_unix_seconds}:{expires_at_unix_seconds}"
    ))
}

fn invalid(path: PathBuf, detail: impl Into<String>) -> AppError {
    AppError::from(InputError::Invalid {
        path,
        detail: detail.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evidence() -> ResearchEvidence {
        ResearchEvidence {
            strategy_id: "aggressive-growth-v1".to_string(),
            risk_profile_id: "aggressive-growth-risk-v1".to_string(),
            config_hash: "hash".to_string(),
            annualized_net_roi_bps: MIN_BACKTEST_NET_ROI_BPS,
            forward_paper_roi_bps: MIN_FORWARD_PAPER_ROI_BPS,
            profit_factor_bps: MIN_PROFIT_FACTOR_BPS,
            max_drawdown_bps: MAX_DRAWDOWN_BPS,
            backtest_trades: MIN_BACKTEST_TRADES,
            forward_paper_trades: MIN_FORWARD_PAPER_TRADES,
            risk_gate_bypasses: 0,
            generated_at_unix_seconds: 1,
            approved: true,
        }
    }

    #[test]
    fn aggressive_growth_gate_accepts_boundary_values() {
        validate_aggressive_growth(&evidence(), "report.json").expect("boundary should pass");
    }

    #[test]
    fn aggressive_growth_gate_rejects_drawdown() {
        let mut evidence = evidence();
        evidence.max_drawdown_bps = MAX_DRAWDOWN_BPS + 1;
        let err =
            validate_aggressive_growth(&evidence, "report.json").expect_err("drawdown should fail");
        assert!(format!("{err}").contains("max drawdown"));
    }

    #[test]
    fn stale_approval_is_rejected() {
        let config = AppConfig {
            approval_artifact: Some("approval.json".into()),
            max_account_risk_bps: Some(100),
            max_daily_loss_bps: Some(300),
            max_contracts_per_order: Some(1),
            ..AppConfig::default()
        };
        let mut artifact = LiveApprovalArtifact {
            approval_version: "optionclaw-live-approval-v1".to_string(),
            strategy_id: config.strategy_id.clone(),
            risk_profile_id: config.risk_profile_id.clone(),
            config_hash: config.approval_config_hash(),
            generated_at_unix_seconds: 1,
            expires_at_unix_seconds: 2,
            approved: true,
            signature: "sig".to_string(),
            evidence: evidence(),
        };
        artifact.evidence.config_hash = artifact.config_hash.clone();
        let err = validate_approval_artifact(&config, &artifact, 3, "approval.json")
            .expect_err("stale approval should fail");
        assert!(format!("{err}").contains("stale"));
    }
}
