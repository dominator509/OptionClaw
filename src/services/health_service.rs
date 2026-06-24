use std::path::{Path, PathBuf};

use crate::{
    config::AppConfig,
    errors::AppError,
    observability::{
        init_logging, record_metric, record_structured_log, LogLevel, MetricEvent, StructuredField,
        StructuredLogEvent,
    },
    persistence::verify_data_dir,
    services::{derive_data_dir, state_service::verify_state},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthReport {
    pub config_path: PathBuf,
    pub data_dir: PathBuf,
    pub trading_mode: crate::domain::TradingMode,
    pub config_ready: bool,
    pub data_ready: bool,
    pub audit_ready: bool,
    pub secrets_store_ready: bool,
    pub providers_ready: bool,
    pub kill_switch_active: bool,
}

pub fn health(config_path: impl AsRef<Path>) -> Result<HealthReport, AppError> {
    let config_path = config_path.as_ref().to_path_buf();
    init_logging(LogLevel::Info);
    let config = AppConfig::load_from_path(&config_path)?;
    let data_dir = derive_data_dir(&config_path);

    let mut data_ready = false;
    let mut audit_ready = false;
    let config_ready = true;
    let secrets_store_ready = matches!(config.trading_mode, crate::domain::TradingMode::Paper);
    let providers_ready = matches!(config.trading_mode, crate::domain::TradingMode::Paper);

    if data_dir.exists() {
        if verify_state(&data_dir).is_ok() {
            data_ready = true;
        }
        audit_ready = verify_data_dir(&data_dir).is_ok();
    }

    let report = HealthReport {
        config_path,
        data_dir,
        trading_mode: config.trading_mode,
        config_ready,
        data_ready,
        audit_ready,
        secrets_store_ready,
        providers_ready,
        kill_switch_active: false,
    };

    record_metric(MetricEvent::health_status(
        report.config_ready,
        report.data_ready,
        report.audit_ready,
        report.secrets_store_ready,
        report.providers_ready,
        report.kill_switch_active,
    ));
    record_structured_log(
        StructuredLogEvent::new(LogLevel::Info, "health", "check", "success")
            .with_mode(report.trading_mode.to_string())
            .with_field(StructuredField::plain(
                "config_path",
                report.config_path.display().to_string(),
            ))
            .with_field(StructuredField::plain(
                "data_dir",
                report.data_dir.display().to_string(),
            ))
            .with_field(StructuredField::plain(
                "config_ready",
                report.config_ready.to_string(),
            ))
            .with_field(StructuredField::plain(
                "data_ready",
                report.data_ready.to_string(),
            ))
            .with_field(StructuredField::plain(
                "audit_ready",
                report.audit_ready.to_string(),
            ))
            .with_field(StructuredField::plain(
                "secrets_store_ready",
                report.secrets_store_ready.to_string(),
            ))
            .with_field(StructuredField::plain(
                "providers_ready",
                report.providers_ready.to_string(),
            ))
            .with_field(StructuredField::plain(
                "kill_switch_active",
                report.kill_switch_active.to_string(),
            )),
    );

    Ok(report)
}
