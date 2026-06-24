use crate::{
    domain::{MarketSnapshot, PercentBps, Signal},
    errors::AppError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvisoryContext {
    pub market: MarketSnapshot,
    pub signal: Signal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvisoryResult {
    pub score: i16,
    pub confidence: PercentBps,
    pub explanation: String,
}

pub trait LlmAdvisor {
    fn advise(&self, context: AdvisoryContext) -> Result<AdvisoryResult, AppError>;
}

#[derive(Debug, Clone)]
pub struct FixtureLlmAdvisor {
    advisory: AdvisoryResult,
}

impl FixtureLlmAdvisor {
    pub fn new(advisory: AdvisoryResult) -> Self {
        Self { advisory }
    }
}

impl LlmAdvisor for FixtureLlmAdvisor {
    fn advise(&self, _context: AdvisoryContext) -> Result<AdvisoryResult, AppError> {
        Ok(self.advisory.clone())
    }
}

pub const LAYER: &str = "llm";
