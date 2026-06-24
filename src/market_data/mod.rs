use crate::{
    domain::{CalendarDate, MarketSnapshot, OptionContract},
    errors::AppError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketDataRequest {
    pub contract: OptionContract,
    pub as_of: CalendarDate,
}

pub trait MarketDataProvider {
    fn snapshot(&self, request: MarketDataRequest) -> Result<MarketSnapshot, AppError>;
}

#[derive(Debug, Clone)]
pub struct FixtureMarketDataProvider {
    snapshot: MarketSnapshot,
}

impl FixtureMarketDataProvider {
    pub fn new(snapshot: MarketSnapshot) -> Self {
        Self { snapshot }
    }
}

impl MarketDataProvider for FixtureMarketDataProvider {
    fn snapshot(&self, _request: MarketDataRequest) -> Result<MarketSnapshot, AppError> {
        Ok(self.snapshot.clone())
    }
}

pub const LAYER: &str = "market_data";
