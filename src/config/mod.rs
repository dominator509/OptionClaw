use std::{fs, path::PathBuf};

use serde::Deserialize;

pub use crate::domain::TradingMode;
use crate::{
    domain::{PercentBps, RiskLimits},
    errors::{AppError, ConfigError, InputError, SecurityError},
    risk::{authorize_execution, ExecutionGates},
};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct AppConfig {
    pub trading_mode: TradingMode,
    pub provider: Provider,
    pub provider_environment: ProviderEnvironment,
    pub strategy_id: String,
    pub risk_profile_id: String,
    pub max_account_risk_bps: Option<u32>,
    pub max_daily_loss_bps: Option<u32>,
    pub max_contracts_per_order: Option<u32>,
    pub kill_switch_file: Option<PathBuf>,
    pub approval_artifact: Option<PathBuf>,
    pub alpaca_base_url: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            trading_mode: TradingMode::Paper,
            provider: Provider::Fixture,
            provider_environment: ProviderEnvironment::Paper,
            strategy_id: "aggressive-growth-v1".to_string(),
            risk_profile_id: "aggressive-growth-risk-v1".to_string(),
            max_account_risk_bps: None,
            max_daily_loss_bps: None,
            max_contracts_per_order: None,
            kill_switch_file: None,
            approval_artifact: None,
            alpaca_base_url: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    #[default]
    Fixture,
    Alpaca,
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Fixture => "fixture",
            Self::Alpaca => "alpaca",
        })
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ProviderEnvironment {
    #[default]
    Paper,
    Sandbox,
    Live,
}

impl std::fmt::Display for ProviderEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Paper => "paper",
            Self::Sandbox => "sandbox",
            Self::Live => "live",
        })
    }
}

impl AppConfig {
    pub fn load_from_path(path: impl AsRef<std::path::Path>) -> Result<Self, ConfigError> {
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
        if self.trading_mode != TradingMode::Live {
            return authorize_execution(self.trading_mode, ExecutionGates::default());
        }

        self.validate_live_contract()
    }

    pub fn validate_live_contract(&self) -> Result<(), AppError> {
        if std::env::var("OPTIONCLAW_ENABLE_LIVE_TRADING").as_deref() != Ok("true") {
            return Err(AppError::from(SecurityError::LiveTradingDisabled {
                mode: self.trading_mode.to_string(),
            }));
        }

        if self.provider != Provider::Alpaca {
            return Err(AppError::from(InputError::Invalid {
                path: PathBuf::from("config"),
                detail: "live mode requires provider = \"alpaca\"".to_string(),
            }));
        }

        for name in ["OPTIONCLAW_ALPACA_API_KEY", "OPTIONCLAW_ALPACA_API_SECRET"] {
            if std::env::var(name).unwrap_or_default().trim().is_empty() {
                return Err(AppError::from(SecurityError::SecretMissing {
                    name: name.to_string(),
                }));
            }
        }

        let _ = self.live_risk_limits()?;
        if self.approval_artifact.is_none() {
            return Err(AppError::from(InputError::Invalid {
                path: PathBuf::from("config"),
                detail: "live mode requires approval_artifact".to_string(),
            }));
        }

        Ok(())
    }

    pub fn live_risk_limits(&self) -> Result<RiskLimits, AppError> {
        let max_account_risk_bps =
            self.required_bps("max_account_risk_bps", self.max_account_risk_bps)?;
        let max_daily_loss_bps =
            self.required_bps("max_daily_loss_bps", self.max_daily_loss_bps)?;
        let max_contracts_per_order = self.max_contracts_per_order.ok_or_else(|| {
            AppError::from(InputError::Invalid {
                path: PathBuf::from("config"),
                detail: "live mode requires max_contracts_per_order".to_string(),
            })
        })?;

        Ok(RiskLimits::new(
            PercentBps::from_bps(max_account_risk_bps)?,
            PercentBps::from_bps(max_daily_loss_bps)?,
            max_contracts_per_order,
            true,
        )?)
    }

    pub fn approval_config_hash(&self) -> String {
        let input = format!(
            "provider={};provider_environment={};strategy_id={};risk_profile_id={};max_account_risk_bps={:?};max_daily_loss_bps={:?};max_contracts_per_order={:?}",
            self.provider,
            self.provider_environment,
            self.strategy_id,
            self.risk_profile_id,
            self.max_account_risk_bps,
            self.max_daily_loss_bps,
            self.max_contracts_per_order
        );
        stable_hex_hash(&input)
    }

    fn required_bps(&self, name: &'static str, value: Option<u32>) -> Result<u32, AppError> {
        value.ok_or_else(|| {
            AppError::from(InputError::Invalid {
                path: PathBuf::from("config"),
                detail: format!("live mode requires {name}"),
            })
        })
    }
}

pub fn stable_hex_hash(input: &str) -> String {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x00000100000001B3;
    let mut hash = FNV_OFFSET;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::{stable_hex_hash, AppConfig, Provider, TradingMode};

    #[test]
    fn defaults_to_paper_mode() {
        let config: AppConfig = toml::from_str("").expect("empty config should use defaults");
        assert_eq!(config.trading_mode, TradingMode::Paper);
        assert_eq!(config.provider, Provider::Fixture);
    }

    #[test]
    fn parses_explicit_paper_mode() {
        let config: AppConfig = toml::from_str("trading_mode = 'paper'").expect("valid config");
        assert_eq!(config.trading_mode, TradingMode::Paper);
    }

    #[test]
    fn live_mode_fails_closed_at_validation_boundary() {
        let config = AppConfig {
            trading_mode: TradingMode::Live,
            provider: Provider::Alpaca,
            approval_artifact: Some("var/dev/live/live-approval.json".into()),
            max_account_risk_bps: Some(100),
            max_daily_loss_bps: Some(300),
            max_contracts_per_order: Some(1),
            ..AppConfig::default()
        };
        let err = config
            .validate_security()
            .expect_err("live mode should fail closed");
        assert!(format!("{err}").contains("LIVE_TRADING_DISABLED"));
    }

    #[test]
    fn config_hash_changes_when_risk_changes() {
        let first = AppConfig {
            provider: Provider::Alpaca,
            max_account_risk_bps: Some(100),
            max_daily_loss_bps: Some(300),
            max_contracts_per_order: Some(1),
            ..AppConfig::default()
        };
        let second = AppConfig {
            max_account_risk_bps: Some(200),
            ..first.clone()
        };
        assert_ne!(first.approval_config_hash(), second.approval_config_hash());
        assert_eq!(stable_hex_hash("abc"), stable_hex_hash("abc"));
    }
}
