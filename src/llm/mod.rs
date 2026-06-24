use crate::{
    domain::{MarketSnapshot, PercentBps, Signal},
    errors::{AppError, DomainError},
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

impl AdvisoryResult {
    pub fn new(
        score: i16,
        confidence: PercentBps,
        explanation: impl Into<String>,
    ) -> Result<Self, DomainError> {
        if !(-100..=100).contains(&score) {
            return Err(DomainError::InvalidScore { value: score });
        }

        let explanation = explanation.into();
        if explanation.trim().is_empty() {
            return Err(DomainError::EmptyField {
                field: "explanation",
            });
        }

        Ok(Self {
            score,
            confidence,
            explanation: explanation.trim().to_string(),
        })
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{CalendarDate, OptionContract, OptionKind, Price};
    use std::time::{Duration, UNIX_EPOCH};

    fn market_snapshot() -> MarketSnapshot {
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

        MarketSnapshot::new(
            contract,
            UNIX_EPOCH + Duration::from_secs(1704067201),
            Price::from_micros(1_900_000).expect("valid price"),
            Some(Price::from_micros(1_800_000).expect("valid price")),
            Some(Price::from_micros(2_000_000).expect("valid price")),
            Some(Price::from_micros(1_900_000).expect("valid price")),
            Some(2_500),
        )
        .expect("valid market snapshot")
    }

    fn signal() -> Signal {
        Signal::new(
            crate::domain::SignalSource::Model,
            75,
            PercentBps::from_bps(8_500).expect("valid confidence"),
            UNIX_EPOCH + Duration::from_secs(1704067201),
            "fixture advisory",
        )
        .expect("valid signal")
    }

    #[test]
    fn advisory_result_rejects_invalid_score() {
        let err = AdvisoryResult::new(
            101,
            PercentBps::from_bps(8_500).expect("valid confidence"),
            "fixture advisory",
        )
        .expect_err("score outside range should fail");
        assert_eq!(err.code().as_str(), "DOMAIN_INVALID_SCORE");
    }

    #[test]
    fn advisory_result_trims_explanation_and_accepts_valid_output() {
        let advisory = AdvisoryResult::new(
            75,
            PercentBps::from_bps(8_500).expect("valid confidence"),
            " fixture advisory ",
        )
        .expect("valid advisory");
        assert_eq!(advisory.explanation, "fixture advisory");
    }

    #[test]
    fn fixture_advisor_returns_expected_result() {
        let advisor = FixtureLlmAdvisor::new(
            AdvisoryResult::new(
                75,
                PercentBps::from_bps(8_500).expect("valid confidence"),
                "fixture advisory",
            )
            .expect("valid advisory"),
        );
        let advisory = advisor
            .advise(AdvisoryContext {
                market: market_snapshot(),
                signal: signal(),
            })
            .expect("advisory should resolve");
        assert_eq!(advisory.score, 75);
        assert_eq!(advisory.confidence.bps(), 8_500);
    }
}
