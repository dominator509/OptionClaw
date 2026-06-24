use std::path::{Path, PathBuf};

use crate::{config::AppConfig, domain::TradingMode, errors::AppError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigReport {
    pub config_path: PathBuf,
    pub trading_mode: TradingMode,
}

pub fn check_config(path: impl AsRef<Path>) -> Result<ConfigReport, AppError> {
    let path = path.as_ref().to_path_buf();
    let config = AppConfig::load_from_path(&path)?;
    Ok(ConfigReport {
        config_path: path,
        trading_mode: config.trading_mode,
    })
}
