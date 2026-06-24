use std::path::{Path, PathBuf};

use crate::observability::{
    error_code_from_display, init_logging, record_metric, record_structured_log, LogLevel,
    MetricEvent, StructuredField, StructuredLogEvent,
};
use crate::{config::AppConfig, domain::TradingMode, errors::AppError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigReport {
    pub config_path: PathBuf,
    pub trading_mode: TradingMode,
}

pub fn check_config(path: impl AsRef<Path>) -> Result<ConfigReport, AppError> {
    let path = path.as_ref().to_path_buf();
    init_logging(LogLevel::Info);
    let config = AppConfig::load_from_path(&path)?;
    if let Err(err) = config.validate_security() {
        record_metric(MetricEvent::config_validation(
            false,
            config.trading_mode.to_string(),
        ));
        record_structured_log(
            StructuredLogEvent::new(
                LogLevel::Warn,
                "check-config",
                "validate_security",
                "failure",
            )
            .with_mode(config.trading_mode.to_string())
            .with_field(StructuredField::plain(
                "config_path",
                path.display().to_string(),
            ))
            .with_error_code(error_code_from_display(&err)),
        );
        return Err(err);
    }
    record_metric(MetricEvent::config_validation(
        true,
        config.trading_mode.to_string(),
    ));
    record_structured_log(
        StructuredLogEvent::new(
            LogLevel::Info,
            "check-config",
            "validate_security",
            "success",
        )
        .with_mode(config.trading_mode.to_string())
        .with_field(StructuredField::plain(
            "config_path",
            path.display().to_string(),
        )),
    );
    Ok(ConfigReport {
        config_path: path,
        trading_mode: config.trading_mode,
    })
}
