use std::{fs, path::Path};

use serde::Deserialize;

pub use crate::domain::TradingMode;
use crate::{
    errors::ConfigError,
    risk::{authorize_execution, ExecutionGates},
};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct AppConfig {
    pub trading_mode: TradingMode,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            trading_mode: TradingMode::Paper,
        }
    }
}

impl AppConfig {
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path).map_err(|source| ConfigError::Read {
            path: path.to_path_buf(),
            source: Box::new(source),
        })?;

        toml::from_str(&contents).map_err(|source| ConfigError::Parse {
            path: path.to_path_buf(),
            source: Box::new(source),
        })
    }

    pub fn validate_security(&self) -> Result<(), crate::errors::AppError> {
        authorize_execution(self.trading_mode, ExecutionGates::default())
    }
}

#[cfg(test)]
mod tests {
    use super::{AppConfig, TradingMode};

    #[test]
    fn defaults_to_paper_mode() {
        let config: AppConfig = toml::from_str("").expect("empty config should use defaults");
        assert_eq!(config.trading_mode, TradingMode::Paper);
    }

    #[test]
    fn parses_explicit_paper_mode() {
        let config: AppConfig = toml::from_str("trading_mode = 'paper'").expect("valid config");
        assert_eq!(config.trading_mode, TradingMode::Paper);
    }
}
