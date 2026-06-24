use crate::{
    domain::TradingMode,
    errors::{AppError, SecurityError},
};

pub use crate::domain::risk::{
    evaluate_order_intent, RiskContext, RiskDecision, RiskLimits, RiskReasonCode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ExecutionGates {
    pub live_trading_enabled: bool,
    pub provider_configured: bool,
    pub secrets_present: bool,
    pub production_ready: bool,
    pub kill_switch_active: bool,
}

pub fn authorize_execution(mode: TradingMode, gates: ExecutionGates) -> Result<(), AppError> {
    if gates.kill_switch_active {
        return Err(SecurityError::KillSwitchActive.into());
    }

    if matches!(mode, TradingMode::Paper) {
        return Ok(());
    }

    if !gates.live_trading_enabled
        || !gates.provider_configured
        || !gates.secrets_present
        || !gates.production_ready
    {
        return Err(SecurityError::LiveTradingDisabled {
            mode: mode.to_string(),
        }
        .into());
    }

    Ok(())
}

pub const LAYER: &str = "risk";
