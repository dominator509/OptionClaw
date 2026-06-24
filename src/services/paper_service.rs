use std::{
    path::{Path, PathBuf},
    time::{Duration, UNIX_EPOCH},
};

use serde::Deserialize;

use crate::{
    domain::{
        AccountSnapshot, CalendarDate, ContractQuantity, MarketSnapshot, OptionContract,
        OptionKind, OrderIntent, OrderIntentSpec, OrderSide, OrderType, PercentBps, Price,
        RiskDecision, RiskLimits, Signal, SignalSource, TradingMode,
    },
    errors::{AppError, InputError},
    execution::{
        DisabledExecutionProvider, ExecutionProvider, FixturePaperExecutor, PaperExecutor,
    },
    llm::{AdvisoryContext, AdvisoryResult, FixtureLlmAdvisor, LlmAdvisor},
    market_data::{FixtureMarketDataProvider, MarketDataProvider, MarketDataRequest},
    observability::{
        error_code_from_display, init_logging, record_metric, record_structured_log, LogLevel,
        MetricEvent, StructuredField, StructuredLogEvent,
    },
    persistence::{
        append_audit, init_data_dir, read_state, write_state_atomic, AuditEvent, AuditEventType,
        StoredPosition,
    },
    risk::evaluate_order_intent,
    services::{config_service::check_config, derive_data_dir, load_json_file},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaperExecutionStatus {
    Executed,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaperRunReport {
    pub config_path: PathBuf,
    pub fixture_root: PathBuf,
    pub trading_mode: TradingMode,
    pub order_intent_id: String,
    pub risk_decision: RiskDecision,
    pub execution_status: PaperExecutionStatus,
    pub audit_appended: bool,
    pub state_updated: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct PaperRunFixture {
    market_snapshot: MarketSnapshotFixture,
    account: AccountFixture,
    risk_limits: RiskLimitsFixture,
    order: OrderFixture,
}

#[derive(Debug, Clone, Deserialize)]
struct MarketSnapshotFixture {
    contract: OptionContractFixture,
    timestamp_unix_seconds: u64,
    underlying_price_micros: i64,
    option_bid_micros: Option<i64>,
    option_ask_micros: Option<i64>,
    option_last_micros: Option<i64>,
    implied_volatility_bps: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
struct OptionContractFixture {
    symbol: String,
    underlying: String,
    expiration: CalendarDateFixture,
    strike_micros: i64,
    kind: OptionKind,
    venue_id: Option<String>,
    as_of: CalendarDateFixture,
}

#[derive(Debug, Clone, Deserialize)]
struct CalendarDateFixture {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct AccountFixture {
    equity_micros: i64,
    available_cash_micros: Option<i64>,
    daily_loss_bps: u32,
}

#[derive(Debug, Clone, Deserialize)]
struct RiskLimitsFixture {
    max_account_risk_bps: u32,
    max_daily_loss_bps: u32,
    max_contracts_per_order: u32,
    allow_live: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct OrderFixture {
    id: String,
    strategy_id: String,
    risk_context_id: String,
    quantity: u32,
    side: OrderSide,
    order_type: OrderType,
}

#[derive(Debug, Clone, Deserialize)]
struct AdvisoryFixture {
    score: i16,
    confidence_bps: u32,
    explanation: String,
}

pub fn run_paper_once(
    config_path: impl AsRef<Path>,
    fixture_root: impl AsRef<Path>,
) -> Result<PaperRunReport, AppError> {
    let config_path = config_path.as_ref().to_path_buf();
    let fixture_root = fixture_root.as_ref().to_path_buf();
    init_logging(LogLevel::Info);
    let config = check_config(&config_path)?;
    if matches!(config.trading_mode, TradingMode::Live) {
        return Err(AppError::from(InputError::Invalid {
            path: config_path,
            detail: "paper run-once requires paper or sandbox mode".to_string(),
        }));
    }
    let data_dir = derive_data_dir(&config_path);
    init_data_dir(&data_dir)?;

    let paper_fixture: PaperRunFixture =
        load_json_file(fixture_root.join("market").join("sample_snapshot.json"))?;
    let advisory_fixture: AdvisoryFixture =
        load_json_file(fixture_root.join("llm").join("sample_advisory.json"))?;

    let market_snapshot = paper_fixture.market_snapshot.to_domain()?;
    let advisory = AdvisoryResult::new(
        advisory_fixture.score,
        PercentBps::from_bps(advisory_fixture.confidence_bps)?,
        advisory_fixture.explanation,
    )?;
    let market_provider = FixtureMarketDataProvider::new(market_snapshot.clone());
    let llm_advisor = FixtureLlmAdvisor::new(advisory.clone());
    let execution_provider = FixturePaperExecutor::new();
    let live_provider = DisabledExecutionProvider::new();

    let resolved_market = match market_provider.snapshot(MarketDataRequest {
        contract: market_snapshot.contract.clone(),
        as_of: market_snapshot.contract.expiration,
    }) {
        Ok(snapshot) => {
            record_metric(MetricEvent::adapter_result("market_data", "snapshot", true));
            snapshot
        }
        Err(err) => {
            record_metric(MetricEvent::adapter_result(
                "market_data",
                "snapshot",
                false,
            ));
            record_structured_log(
                StructuredLogEvent::new(LogLevel::Warn, "paper", "market_snapshot", "failure")
                    .with_mode(config.trading_mode.to_string())
                    .with_provider("market_data")
                    .with_error_code(error_code_from_display(&err)),
            );
            return Err(err);
        }
    };
    let resolved_advisory = match llm_advisor.advise(AdvisoryContext {
        market: resolved_market.clone(),
        signal: Signal::new(
            SignalSource::Model,
            advisory.score,
            advisory.confidence,
            UNIX_EPOCH + Duration::from_secs(paper_fixture.market_snapshot.timestamp_unix_seconds),
            advisory.explanation.clone(),
        )?,
    }) {
        Ok(advisory) => {
            record_metric(MetricEvent::adapter_result("llm", "advise", true));
            advisory
        }
        Err(err) => {
            record_metric(MetricEvent::adapter_result("llm", "advise", false));
            record_structured_log(
                StructuredLogEvent::new(LogLevel::Warn, "paper", "llm_advice", "failure")
                    .with_mode(config.trading_mode.to_string())
                    .with_provider("llm")
                    .with_error_code(error_code_from_display(&err)),
            );
            return Err(err);
        }
    };

    let intent = build_order_intent(&paper_fixture, &resolved_market)?;
    let account = paper_fixture.account.to_domain()?;
    let limits = paper_fixture.risk_limits.to_domain()?;
    let decision = evaluate_order_intent(&intent, Some(&limits), &account, false);
    record_metric(MetricEvent::risk_decision(decision.is_accepted()));
    record_structured_log(
        StructuredLogEvent::new(
            if decision.is_accepted() {
                LogLevel::Info
            } else {
                LogLevel::Warn
            },
            "paper",
            "evaluate_order_intent",
            if decision.is_accepted() {
                "accepted"
            } else {
                "rejected"
            },
        )
        .with_mode(config.trading_mode.to_string())
        .with_order_intent_id(intent.id.clone())
        .with_strategy_id(intent.strategy_id.clone())
        .with_risk_profile_id(intent.risk_context_id.clone())
        .with_field(StructuredField::plain("decision", decision.to_string())),
    );

    let audit_path = data_dir.join("audit").join("events.jsonl");
    let audit_event = AuditEvent::new(
        AuditEventType::RiskDecision,
        config.trading_mode,
        Some(intent.id.clone()),
        Some(decision.to_string()),
        format!("paper run-once decision: {}", resolved_advisory.explanation),
    )?;
    if let Err(err) = append_audit(&audit_path, &audit_event) {
        record_metric(MetricEvent::audit_append(false));
        record_structured_log(
            StructuredLogEvent::new(LogLevel::Warn, "paper", "append_audit", "failure")
                .with_mode(config.trading_mode.to_string())
                .with_order_intent_id(intent.id.clone())
                .with_field(StructuredField::plain(
                    "audit_path",
                    audit_path.display().to_string(),
                ))
                .with_error_code(error_code_from_display(&err)),
        );
        return Err(err.into());
    }
    record_metric(MetricEvent::audit_append(true));

    let mut state = read_state(data_dir.join("paper").join("state.json"))?;

    let mut execution_status = PaperExecutionStatus::Rejected;
    let mut state_updated = false;

    if decision.is_accepted() {
        let _paper_report = match execution_provider.execute(&intent) {
            Ok(report) => {
                record_metric(MetricEvent::adapter_result(
                    "paper_executor",
                    "execute",
                    true,
                ));
                report
            }
            Err(err) => {
                record_metric(MetricEvent::paper_execution(false));
                record_metric(MetricEvent::adapter_result(
                    "paper_executor",
                    "execute",
                    false,
                ));
                record_structured_log(
                    StructuredLogEvent::new(LogLevel::Warn, "paper", "execute", "failure")
                        .with_mode(config.trading_mode.to_string())
                        .with_order_intent_id(intent.id.clone())
                        .with_provider("paper_executor")
                        .with_error_code(error_code_from_display(&err)),
                );
                return Err(err);
            }
        };
        let _live_report = match live_provider.execute_live(&intent) {
            Ok(report) => {
                record_metric(MetricEvent::adapter_result(
                    "live_executor",
                    "execute",
                    true,
                ));
                report
            }
            Err(err) => {
                record_metric(MetricEvent::adapter_result(
                    "live_executor",
                    "execute",
                    false,
                ));
                record_structured_log(
                    StructuredLogEvent::new(LogLevel::Warn, "paper", "execute_live", "failure")
                        .with_mode(config.trading_mode.to_string())
                        .with_order_intent_id(intent.id.clone())
                        .with_provider("live_executor")
                        .with_error_code(error_code_from_display(&err)),
                );
                return Err(err);
            }
        };
        execution_status = PaperExecutionStatus::Executed;
        state.positions.push(StoredPosition {
            symbol: intent.contract.symbol.clone(),
            underlying: intent.contract.underlying.clone(),
            expiration: intent.contract.expiration.to_string(),
            strike_micros: intent.contract.strike.micros(),
            kind: match intent.contract.kind {
                OptionKind::Call => "call".to_string(),
                OptionKind::Put => "put".to_string(),
            },
            quantity: intent.quantity.get(),
            average_entry_price_micros: intent
                .limit_price
                .unwrap_or(intent.estimated_max_loss)
                .micros(),
        });
        state.last_updated_unix_seconds =
            Some(paper_fixture.market_snapshot.timestamp_unix_seconds);
        write_state_atomic(data_dir.join("paper").join("state.json"), &state)?;
        state_updated = true;

        let execution_audit = AuditEvent::new(
            AuditEventType::StateWrite,
            config.trading_mode,
            Some(intent.id.clone()),
            Some("PAPER_EXECUTED".to_string()),
            "paper execution applied".to_string(),
        )?;
        if let Err(err) = append_audit(&audit_path, &execution_audit) {
            record_metric(MetricEvent::paper_execution(false));
            record_metric(MetricEvent::audit_append(false));
            record_structured_log(
                StructuredLogEvent::new(LogLevel::Warn, "paper", "append_audit", "failure")
                    .with_mode(config.trading_mode.to_string())
                    .with_order_intent_id(intent.id.clone())
                    .with_field(StructuredField::plain(
                        "audit_path",
                        audit_path.display().to_string(),
                    ))
                    .with_error_code(error_code_from_display(&err)),
            );
            return Err(err.into());
        }
        record_metric(MetricEvent::audit_append(true));
    }

    record_metric(MetricEvent::paper_execution(matches!(
        execution_status,
        PaperExecutionStatus::Executed
    )));
    record_structured_log(
        StructuredLogEvent::new(LogLevel::Info, "paper", "run_once", "success")
            .with_mode(config.trading_mode.to_string())
            .with_order_intent_id(intent.id.clone())
            .with_strategy_id(intent.strategy_id.clone())
            .with_field(StructuredField::plain(
                "risk_decision",
                decision.to_string(),
            ))
            .with_field(StructuredField::plain(
                "execution_status",
                format!("{execution_status:?}"),
            ))
            .with_field(StructuredField::plain("audit_appended", true.to_string()))
            .with_field(StructuredField::plain(
                "state_updated",
                state_updated.to_string(),
            )),
    );

    Ok(PaperRunReport {
        config_path,
        fixture_root,
        trading_mode: config.trading_mode,
        order_intent_id: intent.id,
        risk_decision: decision,
        execution_status,
        audit_appended: true,
        state_updated,
    })
}

fn build_order_intent(
    fixture: &PaperRunFixture,
    market: &MarketSnapshot,
) -> Result<OrderIntent, AppError> {
    let quantity = ContractQuantity::new(fixture.order.quantity)?;
    let limit_price = market
        .option_ask
        .or(market.option_last)
        .or(market.option_bid);
    let estimated_max_loss = limit_price.unwrap_or(market.underlying_price);
    let side = fixture.order.side;

    Ok(OrderIntent::new(OrderIntentSpec {
        id: fixture.order.id.clone(),
        mode: TradingMode::Paper,
        contract: market.contract.clone(),
        side,
        quantity,
        order_type: fixture.order.order_type,
        limit_price,
        estimated_max_loss,
        strategy_id: fixture.order.strategy_id.clone(),
        risk_context_id: fixture.order.risk_context_id.clone(),
        created_at: market.timestamp,
    })?)
}

impl CalendarDateFixture {
    fn to_domain(&self) -> Result<CalendarDate, AppError> {
        Ok(CalendarDate::new(self.year, self.month, self.day)?)
    }
}

impl OptionContractFixture {
    fn to_domain(&self) -> Result<OptionContract, AppError> {
        Ok(OptionContract::new(
            self.symbol.clone(),
            self.underlying.clone(),
            self.expiration.to_domain()?,
            Price::from_micros(self.strike_micros)?,
            self.kind,
            self.venue_id.clone(),
            self.as_of.to_domain()?,
        )?)
    }
}

impl MarketSnapshotFixture {
    fn to_domain(&self) -> Result<MarketSnapshot, AppError> {
        Ok(MarketSnapshot::new(
            self.contract.to_domain()?,
            UNIX_EPOCH + Duration::from_secs(self.timestamp_unix_seconds),
            Price::from_micros(self.underlying_price_micros)?,
            self.option_bid_micros.map(Price::from_micros).transpose()?,
            self.option_ask_micros.map(Price::from_micros).transpose()?,
            self.option_last_micros
                .map(Price::from_micros)
                .transpose()?,
            self.implied_volatility_bps,
        )?)
    }
}

impl AccountFixture {
    fn to_domain(&self) -> Result<AccountSnapshot, AppError> {
        Ok(AccountSnapshot::new(
            "paper",
            Price::from_micros(self.equity_micros)?,
            self.available_cash_micros
                .map(Price::from_micros)
                .transpose()?,
            self.daily_loss_bps,
            Vec::new(),
        )?)
    }
}

impl RiskLimitsFixture {
    fn to_domain(&self) -> Result<RiskLimits, AppError> {
        Ok(RiskLimits::new(
            PercentBps::from_bps(self.max_account_risk_bps)?,
            PercentBps::from_bps(self.max_daily_loss_bps)?,
            self.max_contracts_per_order,
            self.allow_live,
        )?)
    }
}
