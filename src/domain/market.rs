use std::time::{SystemTime, UNIX_EPOCH};

use crate::errors::DomainError;

use super::{OptionContract, Price};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketSnapshot {
    pub contract: OptionContract,
    pub timestamp: SystemTime,
    pub underlying_price: Price,
    pub option_bid: Option<Price>,
    pub option_ask: Option<Price>,
    pub option_last: Option<Price>,
    pub implied_volatility_bps: Option<u32>,
}

impl MarketSnapshot {
    pub fn new(
        contract: OptionContract,
        timestamp: SystemTime,
        underlying_price: Price,
        option_bid: Option<Price>,
        option_ask: Option<Price>,
        option_last: Option<Price>,
        implied_volatility_bps: Option<u32>,
    ) -> Result<Self, DomainError> {
        validate_timestamp(timestamp)?;
        if let (Some(bid), Some(ask)) = (option_bid, option_ask) {
            if ask.micros() < bid.micros() {
                return Err(DomainError::InvalidPercent {
                    field: "ask_bid_spread",
                    value: 0,
                });
            }
        }
        if let Some(vol) = implied_volatility_bps {
            if vol == 0 {
                return Err(DomainError::InvalidPercent {
                    field: "implied_volatility_bps",
                    value: vol,
                });
            }
        }

        Ok(Self {
            contract,
            timestamp,
            underlying_price,
            option_bid,
            option_ask,
            option_last,
            implied_volatility_bps,
        })
    }
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
    fn rejects_invalid_timestamp() {
        let err = MarketSnapshot::new(
            contract(),
            UNIX_EPOCH - std::time::Duration::from_secs(1),
            Price::from_micros(1_000_000).expect("valid price"),
            None,
            None,
            None,
            None,
        )
        .expect_err("pre-epoch timestamp invalid");
        assert_eq!(err.code().as_str(), "DOMAIN_INVALID_TIMESTAMP");
    }

    #[test]
    fn rejects_inverted_spread() {
        let err = MarketSnapshot::new(
            contract(),
            UNIX_EPOCH + std::time::Duration::from_secs(1),
            Price::from_micros(1_000_000).expect("valid price"),
            Some(Price::from_micros(2_000_000).expect("valid price")),
            Some(Price::from_micros(1_500_000).expect("valid price")),
            None,
            None,
        )
        .expect_err("ask below bid invalid");
        assert_eq!(err.code().as_str(), "DOMAIN_INVALID_PERCENT");
    }
}
