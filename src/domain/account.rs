use crate::errors::DomainError;

use super::{ContractQuantity, OptionContract, Price};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub contract: OptionContract,
    pub quantity: ContractQuantity,
    pub average_entry_price: Price,
}

impl Position {
    pub fn new(
        contract: OptionContract,
        quantity: ContractQuantity,
        average_entry_price: Price,
    ) -> Self {
        Self {
            contract,
            quantity,
            average_entry_price,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountSnapshot {
    pub account_id: String,
    pub equity: Price,
    pub available_cash: Option<Price>,
    pub daily_loss_bps: u32,
    pub positions: Vec<Position>,
}

impl AccountSnapshot {
    pub fn new(
        account_id: impl Into<String>,
        equity: Price,
        available_cash: Option<Price>,
        daily_loss_bps: u32,
        positions: Vec<Position>,
    ) -> Result<Self, DomainError> {
        let account_id = normalize_text(account_id.into(), "account_id")?;
        if let Some(cash) = available_cash {
            if cash.micros() <= 0 {
                return Err(DomainError::InvalidPrice {
                    value_micros: cash.micros(),
                });
            }
        }

        Ok(Self {
            account_id,
            equity,
            available_cash,
            daily_loss_bps,
            positions,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{CalendarDate, OptionKind};

    fn position() -> Position {
        let contract = OptionContract::new(
            "AAPL260201C00100000",
            "AAPL",
            CalendarDate::new(2026, 2, 1).expect("valid date"),
            Price::from_micros(1_000_000).expect("valid price"),
            OptionKind::Call,
            None::<String>,
            CalendarDate::new(2026, 1, 1).expect("valid date"),
        )
        .expect("valid contract");
        Position::new(
            contract,
            ContractQuantity::new(1).expect("valid quantity"),
            Price::from_micros(500_000).expect("valid price"),
        )
    }

    #[test]
    fn rejects_empty_account_id() {
        let err = AccountSnapshot::new(
            "  ",
            Price::from_micros(1_000_000).expect("valid price"),
            None,
            0,
            vec![position()],
        )
        .expect_err("empty account id invalid");
        assert_eq!(err.code().as_str(), "DOMAIN_EMPTY_FIELD");
    }

    #[test]
    fn accepts_positive_cash() {
        let account = AccountSnapshot::new(
            "acct-1",
            Price::from_micros(1_000_000).expect("valid price"),
            Some(Price::from_micros(1).expect("valid price")),
            0,
            vec![position()],
        )
        .expect("positive cash should be allowed");
        assert_eq!(account.account_id, "acct-1");
    }
}
