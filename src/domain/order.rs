use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;

use crate::errors::DomainError;

use super::{ContractQuantity, OptionContract, Price, TradingMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderIntentSpec {
    pub id: String,
    pub mode: TradingMode,
    pub contract: OptionContract,
    pub side: OrderSide,
    pub quantity: ContractQuantity,
    pub order_type: OrderType,
    pub limit_price: Option<Price>,
    pub estimated_max_loss: Price,
    pub strategy_id: String,
    pub risk_context_id: String,
    pub created_at: SystemTime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderIntent {
    pub id: String,
    pub mode: TradingMode,
    pub contract: OptionContract,
    pub side: OrderSide,
    pub quantity: ContractQuantity,
    pub order_type: OrderType,
    pub limit_price: Option<Price>,
    pub estimated_max_loss: Price,
    pub strategy_id: String,
    pub risk_context_id: String,
    pub created_at: SystemTime,
}

impl OrderIntent {
    pub fn new(spec: OrderIntentSpec) -> Result<Self, DomainError> {
        let id = normalize_text(spec.id, "id")?;
        let strategy_id = normalize_text(spec.strategy_id, "strategy_id")?;
        let risk_context_id = normalize_text(spec.risk_context_id, "risk_context_id")?;
        validate_timestamp(spec.created_at)?;

        match spec.order_type {
            OrderType::Limit if spec.limit_price.is_none() => {
                return Err(DomainError::InvalidOrderType {
                    detail: "limit orders require a limit price",
                });
            }
            OrderType::Market if spec.limit_price.is_some() => {
                return Err(DomainError::InvalidOrderType {
                    detail: "market orders must not include a limit price",
                });
            }
            _ => {}
        }

        Ok(Self {
            id,
            mode: spec.mode,
            contract: spec.contract,
            side: spec.side,
            quantity: spec.quantity,
            order_type: spec.order_type,
            limit_price: spec.limit_price,
            estimated_max_loss: spec.estimated_max_loss,
            strategy_id,
            risk_context_id,
            created_at: spec.created_at,
        })
    }
}

fn normalize_text(value: String, field: &'static str) -> Result<String, DomainError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(DomainError::EmptyField { field });
    }
    Ok(trimmed.to_string())
}

fn validate_timestamp(timestamp: SystemTime) -> Result<(), DomainError> {
    if timestamp.duration_since(UNIX_EPOCH).is_err() {
        return Err(DomainError::InvalidTimestamp);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{CalendarDate, OptionKind};

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

    #[test]
    fn rejects_zero_quantity() {
        let err = ContractQuantity::new(0).expect_err("zero invalid");
        assert_eq!(err.code().as_str(), "DOMAIN_INVALID_QUANTITY");
    }

    #[test]
    fn rejects_missing_limit_price_for_limit_order() {
        let err = OrderIntent::new(OrderIntentSpec {
            id: "intent-1".to_string(),
            mode: TradingMode::Paper,
            contract: contract(),
            side: OrderSide::Buy,
            quantity: ContractQuantity::new(1).expect("valid quantity"),
            order_type: OrderType::Limit,
            limit_price: None,
            estimated_max_loss: Price::from_micros(100_000).expect("valid price"),
            strategy_id: "strategy-1".to_string(),
            risk_context_id: "risk-1".to_string(),
            created_at: UNIX_EPOCH + std::time::Duration::from_secs(1),
        })
        .expect_err("limit order without limit price invalid");
        assert_eq!(err.code().as_str(), "DOMAIN_INVALID_ORDER_TYPE");
    }
}
