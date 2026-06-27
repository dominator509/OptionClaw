use std::{
    path::{Path, PathBuf},
    time::{Duration, UNIX_EPOCH},
};

use serde::Deserialize;

use crate::{
    domain::{
        AccountSnapshot, CalendarDate, ContractQuantity, OptionContract, OptionKind, OrderIntent,
        OrderIntentSpec, OrderSide, OrderType, PercentBps, Price, RiskDecision, RiskLimits,
        TradingMode,
    },
    errors::AppError,
    observability::{
        record_metric, record_structured_log, LogLevel, MetricEvent, StructuredField,
        StructuredLogEvent,
    },
    risk::evaluate_order_intent,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiskReport {
    pub order_intent_path: PathBuf,
    pub order_intent_id: String,
    pub decision: RiskDecision,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RiskExplainFixture {
    pub order_intent: OrderIntentFixture,
    pub account: AccountFixture,
    pub limits: RiskLimitsFixture,
    pub kill_switch_active: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrderIntentFixture {
    pub id: String,
    pub mode: TradingMode,
    pub contract: OptionContractFixture,
    pub side: OrderSide,
    pub quantity: u32,
    pub order_type: OrderType,
    pub limit_price_micros: Option<i64>,
    pub estimated_max_loss_micros: i64,
    pub strategy_id: String,
    pub risk_context_id: String,
    pub created_at_unix_seconds: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OptionContractFixture {
    pub symbol: String,
    pub underlying: String,
    pub expiration: CalendarDateFixture,
    pub strike_micros: i64,
    pub kind: OptionKind,
    pub venue_id: Option<String>,
    pub as_of: CalendarDateFixture,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CalendarDateFixture {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccountFixture {
    pub account_id: String,
    pub equity_micros: i64,
    pub available_cash_micros: Option<i64>,
    pub daily_loss_bps: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RiskLimitsFixture {
    pub max_account_risk_bps: u32,
    pub max_daily_loss_bps: u32,
    pub max_contracts_per_order: u32,
    pub allow_live: bool,
}

pub fn explain_risk(path: impl AsRef<Path>) -> Result<RiskReport, AppError> {
    let path = path.as_ref().to_path_buf();
    let fixture: RiskExplainFixture = crate::services::load_json_file(&path)?;

    let intent = fixture.order_intent.to_domain()?;
    let account = fixture.account.to_domain()?;
    let limits = fixture.limits.to_domain()?;
    let decision =
        evaluate_order_intent(&intent, Some(&limits), &account, fixture.kill_switch_active);
    record_metric(MetricEvent::risk_decision(decision.is_accepted()));
    record_structured_log(
        StructuredLogEvent::new(
            if decision.is_accepted() {
                LogLevel::Info
            } else {
                LogLevel::Warn
            },
            "risk",
            "evaluate_order_intent",
            if decision.is_accepted() {
                "accepted"
            } else {
                "rejected"
            },
        )
        .with_mode(intent.mode.to_string())
        .with_order_intent_id(intent.id.clone())
        .with_strategy_id(intent.strategy_id.clone())
        .with_risk_profile_id(intent.risk_context_id.clone())
        .with_field(StructuredField::plain(
            "order_intent_path",
            path.display().to_string(),
        ))
        .with_field(StructuredField::plain("decision", decision.to_string())),
    );

    Ok(RiskReport {
        order_intent_path: path,
        order_intent_id: intent.id.clone(),
        decision,
    })
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

impl AccountFixture {
    fn to_domain(&self) -> Result<AccountSnapshot, AppError> {
        Ok(AccountSnapshot::new(
            self.account_id.clone(),
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

impl OrderIntentFixture {
    pub(crate) fn to_domain(&self) -> Result<OrderIntent, AppError> {
        Ok(OrderIntent::new(OrderIntentSpec {
            id: self.id.clone(),
            mode: self.mode,
            contract: self.contract.to_domain()?,
            side: self.side,
            quantity: ContractQuantity::new(self.quantity)?,
            order_type: self.order_type,
            limit_price: self
                .limit_price_micros
                .map(Price::from_micros)
                .transpose()?,
            estimated_max_loss: Price::from_micros(self.estimated_max_loss_micros)?,
            strategy_id: self.strategy_id.clone(),
            risk_context_id: self.risk_context_id.clone(),
            created_at: UNIX_EPOCH + Duration::from_secs(self.created_at_unix_seconds),
        })?)
    }
}
