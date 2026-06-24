use std::{
    fmt,
    sync::{Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Serialize;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoggingConfig {
    pub level: LogLevel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StructuredField {
    pub key: String,
    pub value: String,
    pub redacted: bool,
}

impl StructuredField {
    pub fn plain(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            redacted: false,
        }
    }

    pub fn redacted(key: impl Into<String>, _value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: "<redacted>".to_string(),
            redacted: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StructuredLogEvent {
    pub timestamp_unix_seconds: u64,
    pub level: LogLevel,
    pub command: String,
    pub mode: Option<String>,
    pub strategy_id: Option<String>,
    pub risk_profile_id: Option<String>,
    pub order_intent_id: Option<String>,
    pub provider: Option<String>,
    pub operation: String,
    pub latency_ms: Option<u64>,
    pub result: String,
    pub error_code: Option<String>,
    pub retryable: Option<bool>,
    pub correlation_id: Option<String>,
    pub fields: Vec<StructuredField>,
}

impl StructuredLogEvent {
    pub fn new(
        level: LogLevel,
        command: impl Into<String>,
        operation: impl Into<String>,
        result: impl Into<String>,
    ) -> Self {
        Self {
            timestamp_unix_seconds: now_unix_seconds(),
            level,
            command: command.into(),
            mode: None,
            strategy_id: None,
            risk_profile_id: None,
            order_intent_id: None,
            provider: None,
            operation: operation.into(),
            latency_ms: None,
            result: result.into(),
            error_code: None,
            retryable: None,
            correlation_id: None,
            fields: Vec::new(),
        }
    }

    pub fn with_mode(mut self, mode: impl Into<String>) -> Self {
        self.mode = Some(mode.into());
        self
    }

    pub fn with_strategy_id(mut self, strategy_id: impl Into<String>) -> Self {
        self.strategy_id = Some(strategy_id.into());
        self
    }

    pub fn with_risk_profile_id(mut self, risk_profile_id: impl Into<String>) -> Self {
        self.risk_profile_id = Some(risk_profile_id.into());
        self
    }

    pub fn with_order_intent_id(mut self, order_intent_id: impl Into<String>) -> Self {
        self.order_intent_id = Some(order_intent_id.into());
        self
    }

    pub fn with_provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }

    pub fn with_latency_ms(mut self, latency_ms: u64) -> Self {
        self.latency_ms = Some(latency_ms);
        self
    }

    pub fn with_error_code(mut self, error_code: impl Into<String>) -> Self {
        self.error_code = Some(error_code.into());
        self
    }

    pub fn with_retryable(mut self, retryable: bool) -> Self {
        self.retryable = Some(retryable);
        self
    }

    pub fn with_correlation_id(mut self, correlation_id: impl Into<String>) -> Self {
        self.correlation_id = Some(correlation_id.into());
        self
    }

    pub fn with_field(mut self, field: StructuredField) -> Self {
        self.fields.push(field);
        self
    }
}

static LOGGING_CONFIG: OnceLock<LoggingConfig> = OnceLock::new();
static STRUCTURED_LOGS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

pub fn init_logging(level: LogLevel) -> &'static LoggingConfig {
    LOGGING_CONFIG.get_or_init(|| LoggingConfig { level })
}

pub fn current_logging_level() -> Option<LogLevel> {
    LOGGING_CONFIG.get().map(|config| config.level)
}

pub fn record_structured_log(event: StructuredLogEvent) -> String {
    let line = serde_json::to_string(&event).expect("structured log should serialize");
    structured_log_sink()
        .lock()
        .expect("structured log sink should be usable")
        .push(line.clone());
    line
}

pub fn drain_structured_logs_for_test() -> Vec<String> {
    structured_log_sink()
        .lock()
        .expect("structured log sink should be usable")
        .drain(..)
        .collect()
}

pub fn error_code_from_display(error: impl fmt::Display) -> String {
    error
        .to_string()
        .split_once(':')
        .map(|(code, _)| code.trim().to_string())
        .unwrap_or_else(|| error.to_string())
}

fn structured_log_sink() -> &'static Mutex<Vec<String>> {
    STRUCTURED_LOGS.get_or_init(|| Mutex::new(Vec::new()))
}

fn now_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structured_log_redacts_secret_fields() {
        drain_structured_logs_for_test();
        init_logging(LogLevel::Info);

        let line = record_structured_log(
            StructuredLogEvent::new(LogLevel::Info, "check-config", "load", "success")
                .with_field(StructuredField::plain("config_path", "config/example.toml"))
                .with_field(StructuredField::redacted("api_key", "super-secret")),
        );

        assert!(line.contains("\"redacted\":true"));
        assert!(!line.contains("super-secret"));
    }

    #[test]
    fn logging_level_initializes_once() {
        let config = init_logging(LogLevel::Debug);
        assert_eq!(current_logging_level(), Some(config.level));
    }
}
