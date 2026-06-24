use std::fmt;

use serde::Deserialize;

use crate::errors::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TradingMode {
    #[default]
    Paper,
    Sandbox,
    Live,
}

impl fmt::Display for TradingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Paper => "paper",
            Self::Sandbox => "sandbox",
            Self::Live => "live",
        })
    }
}

impl TradingMode {
    pub fn parse(value: &str) -> Result<Self, DomainError> {
        match value.trim().to_ascii_lowercase().as_str() {
            "paper" => Ok(Self::Paper),
            "sandbox" => Ok(Self::Sandbox),
            "live" => Ok(Self::Live),
            other => Err(DomainError::InvalidTradingMode {
                value: other.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Price {
    micros: i64,
}

impl Price {
    pub const SCALE: i64 = 1_000_000;

    pub fn from_micros(value: i64) -> Result<Self, DomainError> {
        if value <= 0 {
            return Err(DomainError::InvalidPrice {
                value_micros: value,
            });
        }
        Ok(Self { micros: value })
    }

    pub fn micros(self) -> i64 {
        self.micros
    }

    pub fn risk_bps_of(self, equity: Self) -> u32 {
        ((self.micros as u128) * 10_000 / equity.micros as u128) as u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PercentBps {
    bps: u32,
}

impl PercentBps {
    pub const MAX_BPS: u32 = 10_000;

    pub fn from_bps(value: u32) -> Result<Self, DomainError> {
        if value > Self::MAX_BPS {
            return Err(DomainError::InvalidPercent {
                field: "bps",
                value,
            });
        }
        Ok(Self { bps: value })
    }

    pub fn bps(self) -> u32 {
        self.bps
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContractQuantity {
    contracts: u32,
}

impl ContractQuantity {
    pub fn new(value: u32) -> Result<Self, DomainError> {
        if value == 0 {
            return Err(DomainError::InvalidQuantity { value });
        }
        Ok(Self { contracts: value })
    }

    pub fn get(self) -> u32 {
        self.contracts
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CalendarDate {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl CalendarDate {
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, DomainError> {
        if !is_valid_date(year, month, day) {
            return Err(DomainError::InvalidDate { year, month, day });
        }
        Ok(Self { year, month, day })
    }

    pub fn is_on_or_before(self, other: Self) -> bool {
        (self.year, self.month, self.day) <= (other.year, other.month, other.day)
    }
}

impl fmt::Display for CalendarDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum OptionKind {
    Call,
    Put,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionContract {
    pub symbol: String,
    pub underlying: String,
    pub expiration: CalendarDate,
    pub strike: Price,
    pub kind: OptionKind,
    pub venue_id: Option<String>,
}

impl OptionContract {
    pub fn new(
        symbol: impl Into<String>,
        underlying: impl Into<String>,
        expiration: CalendarDate,
        strike: Price,
        kind: OptionKind,
        venue_id: Option<impl Into<String>>,
        as_of: CalendarDate,
    ) -> Result<Self, DomainError> {
        let symbol = normalize_text(symbol.into(), "symbol")?;
        let underlying = normalize_text(underlying.into(), "underlying")?;
        if expiration.is_on_or_before(as_of) {
            return Err(DomainError::ExpiredContract {
                expiration: expiration.to_string(),
                as_of: as_of.to_string(),
            });
        }
        let venue_id = match venue_id {
            Some(value) => Some(normalize_text(value.into(), "venue_id")?),
            None => None,
        };

        Ok(Self {
            symbol,
            underlying,
            expiration,
            strike,
            kind,
            venue_id,
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

fn is_valid_date(year: u16, month: u8, day: u8) -> bool {
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => return false,
    };

    day >= 1 && day <= days_in_month
}

fn is_leap_year(year: u16) -> bool {
    let year = year as u32;
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_of() -> CalendarDate {
        CalendarDate::new(2026, 1, 1).expect("valid date")
    }

    #[test]
    fn rejects_invalid_date() {
        let err = CalendarDate::new(2026, 2, 30).expect_err("invalid date");
        assert_eq!(err.code().as_str(), "DOMAIN_INVALID_DATE");
    }

    #[test]
    fn rejects_empty_symbol() {
        let expiration = CalendarDate::new(2026, 2, 1).expect("valid date");
        let strike = Price::from_micros(1_000_000).expect("valid price");
        let err = OptionContract::new(
            "  ",
            "AAPL",
            expiration,
            strike,
            OptionKind::Call,
            None::<String>,
            as_of(),
        )
        .expect_err("empty symbol should fail");
        assert_eq!(err.code().as_str(), "DOMAIN_EMPTY_FIELD");
    }

    #[test]
    fn rejects_invalid_strike() {
        let expiration = CalendarDate::new(2026, 2, 1).expect("valid date");
        let err = Price::from_micros(0).expect_err("zero price invalid");
        assert_eq!(err.code().as_str(), "DOMAIN_INVALID_PRICE");
        let strike = Price::from_micros(1).expect("valid price");
        let contract = OptionContract::new(
            "AAPL260201C00100000",
            "AAPL",
            expiration,
            strike,
            OptionKind::Call,
            None::<String>,
            as_of(),
        )
        .expect("valid contract");
        assert_eq!(contract.strike.micros(), 1);
    }

    #[test]
    fn rejects_expired_contract() {
        let expiration = CalendarDate::new(2025, 12, 31).expect("valid date");
        let strike = Price::from_micros(1_000_000).expect("valid price");
        let err = OptionContract::new(
            "AAPL260101C00100000",
            "AAPL",
            expiration,
            strike,
            OptionKind::Call,
            None::<String>,
            as_of(),
        )
        .expect_err("expired contract should fail");
        assert_eq!(err.code().as_str(), "DOMAIN_EXPIRED_CONTRACT");
    }

    #[test]
    fn rejects_zero_quantity() {
        let err = ContractQuantity::new(0).expect_err("zero quantity invalid");
        assert_eq!(err.code().as_str(), "DOMAIN_INVALID_QUANTITY");
    }

    #[test]
    fn rejects_invalid_trading_mode() {
        let err = TradingMode::parse("demo").expect_err("invalid mode should fail");
        assert_eq!(err.code().as_str(), "DOMAIN_INVALID_MODE");
    }
}
