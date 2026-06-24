use std::time::{SystemTime, UNIX_EPOCH};

use crate::errors::DomainError;

use super::PercentBps;

pub type AdvisoryScore = PercentBps;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalSource {
    Technical,
    News,
    Fundamental,
    Model,
    Manual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signal {
    pub source: SignalSource,
    pub score: i16,
    pub confidence: PercentBps,
    pub timestamp: SystemTime,
    pub explanation: String,
}

impl Signal {
    pub fn new(
        source: SignalSource,
        score: i16,
        confidence: PercentBps,
        timestamp: SystemTime,
        explanation: impl Into<String>,
    ) -> Result<Self, DomainError> {
        if !(-100..=100).contains(&score) {
            return Err(DomainError::InvalidScore { value: score });
        }
        validate_timestamp(timestamp)?;

        let explanation = normalize_text(explanation.into(), "explanation")?;
        Ok(Self {
            source,
            score,
            confidence,
            timestamp,
            explanation,
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

    #[test]
    fn rejects_invalid_score() {
        let err = Signal::new(
            SignalSource::Model,
            101,
            PercentBps::from_bps(10).expect("valid confidence"),
            UNIX_EPOCH + std::time::Duration::from_secs(1),
            "signal",
        )
        .expect_err("out of range score invalid");
        assert_eq!(err.code().as_str(), "DOMAIN_INVALID_SCORE");
    }
}
