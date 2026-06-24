use crate::errors::DomainError;

use super::{AccountSnapshot, OrderIntent, PercentBps, TradingMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskReasonCode {
    MissingLimits,
    LiveTradingDisabled,
    QuantityOverLimit,
    EstimatedAccountRiskOverCap,
    DailyLossOverCap,
    KillSwitchActive,
}

impl RiskReasonCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingLimits => "RISK_MISSING_LIMITS",
            Self::LiveTradingDisabled => "RISK_LIVE_DISABLED",
            Self::QuantityOverLimit => "RISK_QUANTITY_OVER_LIMIT",
            Self::EstimatedAccountRiskOverCap => "RISK_ACCOUNT_RISK_OVER_CAP",
            Self::DailyLossOverCap => "RISK_DAILY_LOSS_OVER_CAP",
            Self::KillSwitchActive => "KILL_SWITCH_ACTIVE",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RiskLimits {
    pub max_account_risk_bps: PercentBps,
    pub max_daily_loss_bps: PercentBps,
    pub max_contracts_per_order: u32,
    pub allow_live: bool,
}

impl RiskLimits {
    pub fn new(
        max_account_risk_bps: PercentBps,
        max_daily_loss_bps: PercentBps,
        max_contracts_per_order: u32,
        allow_live: bool,
    ) -> Result<Self, DomainError> {
        if max_contracts_per_order == 0 {
            return Err(DomainError::InvalidQuantity {
                value: max_contracts_per_order,
            });
        }

        Ok(Self {
            max_account_risk_bps,
            max_daily_loss_bps,
            max_contracts_per_order,
            allow_live,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RiskDecision {
    Accepted,
    Rejected {
        code: RiskReasonCode,
        message: String,
    },
}

impl RiskDecision {
    pub fn accepted() -> Self {
        Self::Accepted
    }

    pub fn rejected(code: RiskReasonCode, message: impl Into<String>) -> Self {
        Self::Rejected {
            code,
            message: message.into(),
        }
    }

    pub fn is_accepted(&self) -> bool {
        matches!(self, Self::Accepted)
    }

    pub fn code(&self) -> Option<RiskReasonCode> {
        match self {
            Self::Accepted => None,
            Self::Rejected { code, .. } => Some(*code),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RiskContext {
    pub kill_switch_active: bool,
}

pub fn evaluate_order_intent(
    intent: &OrderIntent,
    limits: Option<&RiskLimits>,
    account: &AccountSnapshot,
    kill_switch_active: bool,
) -> RiskDecision {
    let Some(limits) = limits else {
        return RiskDecision::rejected(
            RiskReasonCode::MissingLimits,
            "risk limits must be configured before execution",
        );
    };

    if kill_switch_active {
        return RiskDecision::rejected(
            RiskReasonCode::KillSwitchActive,
            "kill switch is active; execution is blocked",
        );
    }

    if intent.mode == TradingMode::Live && !limits.allow_live {
        return RiskDecision::rejected(
            RiskReasonCode::LiveTradingDisabled,
            "live trading is disabled by risk limits",
        );
    }

    if intent.quantity.get() > limits.max_contracts_per_order {
        return RiskDecision::rejected(
            RiskReasonCode::QuantityOverLimit,
            "order quantity exceeds configured max contracts per order",
        );
    }

    let estimated_account_risk_bps = intent.estimated_max_loss.risk_bps_of(account.equity);
    if estimated_account_risk_bps > limits.max_account_risk_bps.bps() {
        return RiskDecision::rejected(
            RiskReasonCode::EstimatedAccountRiskOverCap,
            "estimated account risk exceeds configured limit",
        );
    }

    if account.daily_loss_bps > limits.max_daily_loss_bps.bps() {
        return RiskDecision::rejected(
            RiskReasonCode::DailyLossOverCap,
            "daily loss exceeds configured limit",
        );
    }

    RiskDecision::accepted()
}

impl std::fmt::Display for RiskDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Accepted => f.write_str("RISK_ACCEPTED"),
            Self::Rejected { code, message } => write!(f, "{}: {}", code.as_str(), message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        AccountSnapshot, CalendarDate, ContractQuantity, OptionContract, OptionKind, OrderSide,
        OrderType, PercentBps, Position, Price,
    };
    use std::time::{Duration, UNIX_EPOCH};

    fn contract() -> OptionContract {
        OptionContract::new(
            "AAPL260201C00100000",
            "AAPL",
            CalendarDate::new(2026, 2, 1).expect("valid date"),
            Price::from_micros(1_000_000).expect("valid price"),
            OptionKind::Call,
            None::<String>,
            CalendarDate::new(2026, 1, 1).expect("valid date"),
        )
        .expect("valid contract")
    }

    fn account(equity_micros: i64, daily_loss_bps: u32) -> AccountSnapshot {
        AccountSnapshot::new(
            "acct-1",
            Price::from_micros(equity_micros).expect("valid price"),
            None,
            daily_loss_bps,
            vec![Position::new(
                contract(),
                ContractQuantity::new(1).expect("valid quantity"),
                Price::from_micros(500_000).expect("valid price"),
            )],
        )
        .expect("valid account snapshot")
    }

    fn order_intent(
        mode: TradingMode,
        quantity: u32,
        max_loss_micros: i64,
    ) -> super::super::order::OrderIntent {
        super::super::order::OrderIntent::new(super::super::order::OrderIntentSpec {
            id: "intent-1".to_string(),
            mode,
            contract: contract(),
            side: OrderSide::Buy,
            quantity: ContractQuantity::new(quantity).expect("valid quantity"),
            order_type: OrderType::Limit,
            limit_price: Some(Price::from_micros(500_000).expect("valid price")),
            estimated_max_loss: Price::from_micros(max_loss_micros).expect("valid price"),
            strategy_id: "strategy-1".to_string(),
            risk_context_id: "risk-1".to_string(),
            created_at: UNIX_EPOCH + Duration::from_secs(1),
        })
        .expect("valid order intent")
    }

    #[test]
    fn rejects_missing_limits() {
        let decision = evaluate_order_intent(
            &order_intent(TradingMode::Paper, 1, 100_000),
            None,
            &account(1_000_000, 0),
            false,
        );
        assert_eq!(
            decision.code(),
            Some(RiskReasonCode::MissingLimits),
            "missing limits should fail closed"
        );
    }

    #[test]
    fn rejects_live_when_disabled() {
        let limits = RiskLimits::new(
            PercentBps::from_bps(100).expect("valid percent"),
            PercentBps::from_bps(100).expect("valid percent"),
            1,
            false,
        )
        .expect("valid limits");
        let decision = evaluate_order_intent(
            &order_intent(TradingMode::Live, 1, 100_000),
            Some(&limits),
            &account(1_000_000, 0),
            false,
        );
        assert_eq!(
            decision.code(),
            Some(RiskReasonCode::LiveTradingDisabled),
            "live trading should be blocked"
        );
    }

    #[test]
    fn rejects_quantity_over_limit() {
        let limits = RiskLimits::new(
            PercentBps::from_bps(100).expect("valid percent"),
            PercentBps::from_bps(100).expect("valid percent"),
            1,
            true,
        )
        .expect("valid limits");
        let decision = evaluate_order_intent(
            &order_intent(TradingMode::Paper, 2, 100_000),
            Some(&limits),
            &account(1_000_000, 0),
            false,
        );
        assert_eq!(
            decision.code(),
            Some(RiskReasonCode::QuantityOverLimit),
            "quantity cap should fail closed"
        );
    }

    #[test]
    fn rejects_risk_over_cap() {
        let limits = RiskLimits::new(
            PercentBps::from_bps(100).expect("valid percent"),
            PercentBps::from_bps(100).expect("valid percent"),
            10,
            true,
        )
        .expect("valid limits");
        let decision = evaluate_order_intent(
            &order_intent(TradingMode::Paper, 1, 500_000),
            Some(&limits),
            &account(1_000_000, 0),
            false,
        );
        assert_eq!(
            decision.code(),
            Some(RiskReasonCode::EstimatedAccountRiskOverCap),
            "risk cap should fail closed"
        );
    }

    #[test]
    fn rejects_daily_loss_over_cap() {
        let limits = RiskLimits::new(
            PercentBps::from_bps(100).expect("valid percent"),
            PercentBps::from_bps(50).expect("valid percent"),
            10,
            true,
        )
        .expect("valid limits");
        let decision = evaluate_order_intent(
            &order_intent(TradingMode::Paper, 1, 10_000),
            Some(&limits),
            &account(1_000_000, 60),
            false,
        );
        assert_eq!(
            decision.code(),
            Some(RiskReasonCode::DailyLossOverCap),
            "daily loss cap should fail closed"
        );
    }

    #[test]
    fn rejects_kill_switch() {
        let limits = RiskLimits::new(
            PercentBps::from_bps(100).expect("valid percent"),
            PercentBps::from_bps(100).expect("valid percent"),
            10,
            true,
        )
        .expect("valid limits");
        let decision = evaluate_order_intent(
            &order_intent(TradingMode::Paper, 1, 10_000),
            Some(&limits),
            &account(1_000_000, 0),
            true,
        );
        assert_eq!(
            decision.code(),
            Some(RiskReasonCode::KillSwitchActive),
            "kill switch should block execution"
        );
        assert_eq!(
            RiskReasonCode::KillSwitchActive.as_str(),
            "KILL_SWITCH_ACTIVE"
        );
        assert!(format!("{decision}").contains("KILL_SWITCH_ACTIVE"));
    }
}
