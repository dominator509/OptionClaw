pub const LAYER: &str = "strategy";

use crate::domain::{AdvisoryScore, OrderIntent, Signal};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrategyCandidate {
    pub candidate_id: String,
    pub signal: Signal,
    pub order_intent: OrderIntent,
    pub advisory_score: Option<AdvisoryScore>,
}

impl StrategyCandidate {
    pub fn new(
        candidate_id: impl Into<String>,
        signal: Signal,
        order_intent: OrderIntent,
        advisory_score: Option<AdvisoryScore>,
    ) -> Result<Self, crate::errors::DomainError> {
        let candidate_id = normalize_text(candidate_id.into(), "candidate_id")?;
        Ok(Self {
            candidate_id,
            signal,
            order_intent,
            advisory_score,
        })
    }
}

fn normalize_text(
    value: String,
    field: &'static str,
) -> Result<String, crate::errors::DomainError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(crate::errors::DomainError::EmptyField { field });
    }
    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        CalendarDate, ContractQuantity, OptionContract, OptionKind, OrderIntentSpec, OrderSide,
        OrderType, PercentBps, Price, SignalSource, TradingMode,
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

    fn signal() -> Signal {
        Signal::new(
            SignalSource::Model,
            90,
            PercentBps::from_bps(9_000).expect("valid confidence"),
            UNIX_EPOCH + Duration::from_secs(1),
            "high confidence",
        )
        .expect("valid signal")
    }

    fn order_intent() -> OrderIntent {
        OrderIntent::new(OrderIntentSpec {
            id: "intent-1".to_string(),
            mode: TradingMode::Paper,
            contract: contract(),
            side: OrderSide::Buy,
            quantity: ContractQuantity::new(1).expect("valid quantity"),
            order_type: OrderType::Limit,
            limit_price: Some(Price::from_micros(500_000).expect("valid price")),
            estimated_max_loss: Price::from_micros(100_000).expect("valid price"),
            strategy_id: "strategy-1".to_string(),
            risk_context_id: "risk-1".to_string(),
            created_at: UNIX_EPOCH + Duration::from_secs(1),
        })
        .expect("valid order intent")
    }

    #[test]
    fn retains_high_advisory_score_without_changing_order() {
        let candidate = StrategyCandidate::new(
            "candidate-1",
            signal(),
            order_intent(),
            Some(PercentBps::from_bps(9_900).expect("valid advisory score")),
        )
        .expect("valid candidate");

        assert_eq!(candidate.advisory_score.unwrap().bps(), 9_900);
        assert_eq!(candidate.order_intent.quantity.get(), 1);
    }

    #[test]
    fn rejects_empty_candidate_id() {
        let err = StrategyCandidate::new("  ", signal(), order_intent(), None)
            .expect_err("empty candidate id invalid");
        assert_eq!(err.code().as_str(), "DOMAIN_EMPTY_FIELD");
    }
}
