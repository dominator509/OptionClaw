pub mod logging;
pub mod metrics;

pub use logging::{
    current_logging_level, drain_structured_logs_for_test, error_code_from_display, init_logging,
    record_structured_log, LogLevel, LoggingConfig, StructuredField, StructuredLogEvent,
};
pub use metrics::{
    record_metric, reset_metrics_for_test, snapshot_metrics, MetricEvent, MetricSnapshot,
};

pub const LAYER: &str = "observability";
