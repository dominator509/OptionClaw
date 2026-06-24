use std::{fmt, path::PathBuf};

#[derive(Debug)]
pub enum AppError {
    Config(ConfigError),
    Domain(DomainError),
    Security(SecurityError),
    Input(InputError),
    Persistence(PersistenceError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CliErrorCode {
    UnknownCommand,
    UnknownSubcommand,
    MissingArgument,
    UnexpectedArgument,
}

impl CliErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnknownCommand => "CLI_UNKNOWN_COMMAND",
            Self::UnknownSubcommand => "CLI_UNKNOWN_SUBCOMMAND",
            Self::MissingArgument => "CLI_MISSING_ARGUMENT",
            Self::UnexpectedArgument => "CLI_UNEXPECTED_ARGUMENT",
        }
    }
}

impl fmt::Display for CliErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Read {
        path: PathBuf,
        source: Box<std::io::Error>,
    },
    Parse {
        path: PathBuf,
        source: Box<toml::de::Error>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainErrorCode {
    EmptyField,
    InvalidPrice,
    InvalidQuantity,
    InvalidTradingMode,
    InvalidDate,
    ExpiredContract,
    InvalidScore,
    InvalidConfidence,
    InvalidPercent,
    InvalidOrderType,
    InvalidTimestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputErrorCode {
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityErrorCode {
    SecretStorageDisabled,
    SecretMissing,
    SecretPlaintextRejected,
    LiveTradingDisabled,
    KillSwitchActive,
    InsecureFilePermissions,
}

impl SecurityErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SecretStorageDisabled => "SECRET_STORAGE_DISABLED",
            Self::SecretMissing => "SECRET_MISSING",
            Self::SecretPlaintextRejected => "SECRET_PLAINTEXT_REJECTED",
            Self::LiveTradingDisabled => "LIVE_TRADING_DISABLED",
            Self::KillSwitchActive => "KILL_SWITCH_ACTIVE",
            Self::InsecureFilePermissions => "INSECURE_FILE_PERMISSIONS",
        }
    }
}

impl fmt::Display for SecurityErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl InputErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Invalid => "INPUT_INVALID",
        }
    }
}

impl fmt::Display for InputErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistenceErrorCode {
    Missing,
    Corrupt,
    UnsupportedSchema,
    Unavailable,
    AuditAppendFailed,
    BackupExists,
    BackupFailed,
    MigrationDryRunFailed,
}

impl PersistenceErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "PERSISTENCE_MISSING",
            Self::Corrupt => "PERSISTENCE_CORRUPT",
            Self::UnsupportedSchema => "SCHEMA_UNSUPPORTED",
            Self::Unavailable => "PERSISTENCE_UNAVAILABLE",
            Self::AuditAppendFailed => "PERSISTENCE_AUDIT_APPEND_FAILED",
            Self::BackupExists => "PERSISTENCE_BACKUP_EXISTS",
            Self::BackupFailed => "PERSISTENCE_BACKUP_FAILED",
            Self::MigrationDryRunFailed => "PERSISTENCE_MIGRATION_DRY_RUN_FAILED",
        }
    }
}

impl fmt::Display for PersistenceErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl DomainErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyField => "DOMAIN_EMPTY_FIELD",
            Self::InvalidPrice => "DOMAIN_INVALID_PRICE",
            Self::InvalidQuantity => "DOMAIN_INVALID_QUANTITY",
            Self::InvalidTradingMode => "DOMAIN_INVALID_MODE",
            Self::InvalidDate => "DOMAIN_INVALID_DATE",
            Self::ExpiredContract => "DOMAIN_EXPIRED_CONTRACT",
            Self::InvalidScore => "DOMAIN_INVALID_SCORE",
            Self::InvalidConfidence => "DOMAIN_INVALID_CONFIDENCE",
            Self::InvalidPercent => "DOMAIN_INVALID_PERCENT",
            Self::InvalidOrderType => "DOMAIN_INVALID_ORDER_TYPE",
            Self::InvalidTimestamp => "DOMAIN_INVALID_TIMESTAMP",
        }
    }
}

impl fmt::Display for DomainErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug)]
pub enum DomainError {
    EmptyField { field: &'static str },
    InvalidPrice { value_micros: i64 },
    InvalidQuantity { value: u32 },
    InvalidTradingMode { value: String },
    InvalidDate { year: u16, month: u8, day: u8 },
    ExpiredContract { expiration: String, as_of: String },
    InvalidScore { value: i16 },
    InvalidConfidence { value: u32 },
    InvalidPercent { field: &'static str, value: u32 },
    InvalidOrderType { detail: &'static str },
    InvalidTimestamp,
}

#[derive(Debug)]
pub enum InputError {
    Invalid { path: PathBuf, detail: String },
}

#[derive(Debug)]
pub enum SecurityError {
    SecretStorageDisabled { path: Option<PathBuf> },
    SecretMissing { name: String },
    SecretPlaintextRejected { path: PathBuf },
    LiveTradingDisabled { mode: String },
    KillSwitchActive,
    InsecureFilePermissions { path: PathBuf },
}

impl SecurityError {
    pub const fn code(&self) -> SecurityErrorCode {
        match self {
            Self::SecretStorageDisabled { .. } => SecurityErrorCode::SecretStorageDisabled,
            Self::SecretMissing { .. } => SecurityErrorCode::SecretMissing,
            Self::SecretPlaintextRejected { .. } => SecurityErrorCode::SecretPlaintextRejected,
            Self::LiveTradingDisabled { .. } => SecurityErrorCode::LiveTradingDisabled,
            Self::KillSwitchActive => SecurityErrorCode::KillSwitchActive,
            Self::InsecureFilePermissions { .. } => SecurityErrorCode::InsecureFilePermissions,
        }
    }
}

impl fmt::Display for SecurityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SecretStorageDisabled { path } => {
                if let Some(path) = path {
                    write!(
                        f,
                        "{}: secret storage at {} is disabled. Hint: use paper mode or configure an approved secret store.",
                        self.code(),
                        path.display()
                    )
                } else {
                    write!(
                        f,
                        "{}: secret storage is disabled. Hint: use paper mode or configure an approved secret store.",
                        self.code()
                    )
                }
            }
            Self::SecretMissing { name } => write!(
                f,
                "{}: required secret `{}` is missing. Hint: configure the secret through the approved local store.",
                self.code(),
                name
            ),
            Self::SecretPlaintextRejected { path } => write!(
                f,
                "{}: plaintext secret file {} is not allowed. Hint: use an encrypted or approved secret store.",
                self.code(),
                path.display()
            ),
            Self::LiveTradingDisabled { mode } => write!(
                f,
                "{}: live trading is disabled for `{}`. Hint: stay in paper mode until production approval is complete.",
                self.code(),
                mode
            ),
            Self::KillSwitchActive => write!(
                f,
                "{}: kill switch is active. Hint: clear the kill switch only after the operator confirms execution may resume.",
                self.code()
            ),
            Self::InsecureFilePermissions { path } => write!(
                f,
                "{}: insecure file permissions at {}. Hint: restrict access before storing secrets.",
                self.code(),
                path.display()
            ),
        }
    }
}

#[derive(Debug)]
pub enum CliError {
    UnknownCommand {
        command: String,
    },
    UnknownSubcommand {
        command: &'static str,
        subcommand: String,
    },
    MissingArgument {
        command: &'static str,
        argument: &'static str,
        hint: &'static str,
    },
    UnexpectedArgument {
        command: &'static str,
        argument: String,
        hint: &'static str,
    },
}

impl CliError {
    pub const fn code(&self) -> CliErrorCode {
        match self {
            Self::UnknownCommand { .. } => CliErrorCode::UnknownCommand,
            Self::UnknownSubcommand { .. } => CliErrorCode::UnknownSubcommand,
            Self::MissingArgument { .. } => CliErrorCode::MissingArgument,
            Self::UnexpectedArgument { .. } => CliErrorCode::UnexpectedArgument,
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownCommand { command } => write!(
                f,
                "{}: unknown command `{}`. Hint: run `optionclaw --help` to list commands.",
                self.code(),
                command
            ),
            Self::UnknownSubcommand {
                command,
                subcommand,
            } => write!(
                f,
                "{}: unknown subcommand `{}` for command `{}`. Hint: run `optionclaw {} --help`.",
                self.code(),
                subcommand,
                command,
                command
            ),
            Self::MissingArgument {
                command,
                argument,
                hint,
            } => write!(
                f,
                "{}: missing required argument `{}` for command `{}`. Hint: {}",
                self.code(),
                argument,
                command,
                hint
            ),
            Self::UnexpectedArgument {
                command,
                argument,
                hint,
            } => write!(
                f,
                "{}: unexpected argument `{}` for command `{}`. Hint: {}",
                self.code(),
                argument,
                command,
                hint
            ),
        }
    }
}

impl std::error::Error for CliError {}

impl InputError {
    pub const fn code(&self) -> InputErrorCode {
        match self {
            Self::Invalid { .. } => InputErrorCode::Invalid,
        }
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid { path, detail } => write!(
                f,
                "{}: invalid input at {}: {}. Hint: verify the fixture or request file format.",
                self.code(),
                path.display(),
                detail
            ),
        }
    }
}

impl std::error::Error for InputError {}

impl std::error::Error for SecurityError {}

#[derive(Debug)]
pub enum PersistenceError {
    Missing {
        path: PathBuf,
    },
    Corrupt {
        path: PathBuf,
        detail: String,
    },
    UnsupportedSchema {
        path: PathBuf,
        found: u32,
        expected: u32,
    },
    Unavailable {
        path: PathBuf,
        source: Box<std::io::Error>,
    },
    AuditAppendFailed {
        path: PathBuf,
        source: Box<std::io::Error>,
    },
    BackupExists {
        path: PathBuf,
    },
    BackupFailed {
        path: PathBuf,
        source: Box<std::io::Error>,
    },
    MigrationDryRunFailed {
        path: PathBuf,
        detail: String,
    },
}

impl PersistenceError {
    pub const fn code(&self) -> PersistenceErrorCode {
        match self {
            Self::Missing { .. } => PersistenceErrorCode::Missing,
            Self::Corrupt { .. } => PersistenceErrorCode::Corrupt,
            Self::UnsupportedSchema { .. } => PersistenceErrorCode::UnsupportedSchema,
            Self::Unavailable { .. } => PersistenceErrorCode::Unavailable,
            Self::AuditAppendFailed { .. } => PersistenceErrorCode::AuditAppendFailed,
            Self::BackupExists { .. } => PersistenceErrorCode::BackupExists,
            Self::BackupFailed { .. } => PersistenceErrorCode::BackupFailed,
            Self::MigrationDryRunFailed { .. } => PersistenceErrorCode::MigrationDryRunFailed,
        }
    }
}

impl fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing { path } => write!(
                f,
                "{}: required persistence path {} is missing. Hint: run state init.",
                self.code(),
                path.display()
            ),
            Self::Corrupt { path, detail } => write!(
                f,
                "{}: persistence file {} is corrupt: {}. Hint: restore from backup.",
                self.code(),
                path.display(),
                detail
            ),
            Self::UnsupportedSchema {
                path,
                found,
                expected,
            } => write!(
                f,
                "{}: persistence schema {} at {} is unsupported; expected {}. Hint: back up the data directory before migrating.",
                self.code(),
                found,
                path.display(),
                expected
            ),
            Self::Unavailable { path, source } => write!(
                f,
                "{}: persistence path {} is unavailable: {}. Hint: verify filesystem permissions.",
                self.code(),
                path.display(),
                source
            ),
            Self::AuditAppendFailed { path, source } => write!(
                f,
                "{}: failed to append audit record at {}: {}. Hint: stop execution and repair the local disk or permissions.",
                self.code(),
                path.display(),
                source
            ),
            Self::BackupExists { path } => write!(
                f,
                "{}: backup target {} already exists. Hint: choose a new backup path.",
                self.code(),
                path.display()
            ),
            Self::BackupFailed { path, source } => write!(
                f,
                "{}: failed to back up data to {}: {}. Hint: verify write access and available space.",
                self.code(),
                path.display(),
                source
            ),
            Self::MigrationDryRunFailed { path, detail } => write!(
                f,
                "{}: migration dry run failed for {}: {}.",
                self.code(),
                path.display(),
                detail
            ),
        }
    }
}

impl std::error::Error for PersistenceError {}

impl DomainError {
    pub const fn code(&self) -> DomainErrorCode {
        match self {
            Self::EmptyField { .. } => DomainErrorCode::EmptyField,
            Self::InvalidPrice { .. } => DomainErrorCode::InvalidPrice,
            Self::InvalidQuantity { .. } => DomainErrorCode::InvalidQuantity,
            Self::InvalidTradingMode { .. } => DomainErrorCode::InvalidTradingMode,
            Self::InvalidDate { .. } => DomainErrorCode::InvalidDate,
            Self::ExpiredContract { .. } => DomainErrorCode::ExpiredContract,
            Self::InvalidScore { .. } => DomainErrorCode::InvalidScore,
            Self::InvalidConfidence { .. } => DomainErrorCode::InvalidConfidence,
            Self::InvalidPercent { .. } => DomainErrorCode::InvalidPercent,
            Self::InvalidOrderType { .. } => DomainErrorCode::InvalidOrderType,
            Self::InvalidTimestamp => DomainErrorCode::InvalidTimestamp,
        }
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(f, "{}: field `{}` must not be empty.", self.code(), field)
            }
            Self::InvalidPrice { value_micros } => write!(
                f,
                "{}: price value `{}` micro-units must be positive.",
                self.code(),
                value_micros
            ),
            Self::InvalidQuantity { value } => write!(
                f,
                "{}: quantity `{}` must be greater than zero.",
                self.code(),
                value
            ),
            Self::InvalidTradingMode { value } => write!(
                f,
                "{}: trading mode `{}` is not supported.",
                self.code(),
                value
            ),
            Self::InvalidDate { year, month, day } => write!(
                f,
                "{}: date {:04}-{:02}-{:02} is not a valid calendar date.",
                self.code(),
                year,
                month,
                day
            ),
            Self::ExpiredContract { expiration, as_of } => write!(
                f,
                "{}: expiration {} is not after valuation date {}.",
                self.code(),
                expiration,
                as_of
            ),
            Self::InvalidScore { value } => write!(
                f,
                "{}: score `{}` is outside the allowed range.",
                self.code(),
                value
            ),
            Self::InvalidConfidence { value } => write!(
                f,
                "{}: confidence `{}` is outside the allowed range.",
                self.code(),
                value
            ),
            Self::InvalidPercent { field, value } => write!(
                f,
                "{}: percent field `{}` has invalid value `{}`.",
                self.code(),
                field,
                value
            ),
            Self::InvalidOrderType { detail } => write!(
                f,
                "{}: invalid order type configuration: {}.",
                self.code(),
                detail
            ),
            Self::InvalidTimestamp => {
                write!(
                    f,
                    "{}: timestamp must be on or after the Unix epoch.",
                    self.code()
                )
            }
        }
    }
}

impl std::error::Error for DomainError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(err) => err.fmt(f),
            Self::Domain(err) => err.fmt(f),
            Self::Security(err) => err.fmt(f),
            Self::Input(err) => err.fmt(f),
            Self::Persistence(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for AppError {}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => write!(
                f,
                "E1001: failed to read config file {}: {}. Hint: verify the file exists and is readable.",
                path.display(),
                source
            ),
            Self::Parse { path, source } => write!(
                f,
                "E1002: failed to parse config file {}: {}. Hint: start from config/example.toml.",
                path.display(),
                source
            ),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<DomainError> for AppError {
    fn from(value: DomainError) -> Self {
        Self::Domain(value)
    }
}

impl From<ConfigError> for AppError {
    fn from(value: ConfigError) -> Self {
        Self::Config(value)
    }
}

impl From<SecurityError> for AppError {
    fn from(value: SecurityError) -> Self {
        Self::Security(value)
    }
}

impl From<InputError> for AppError {
    fn from(value: InputError) -> Self {
        Self::Input(value)
    }
}

impl From<PersistenceError> for AppError {
    fn from(value: PersistenceError) -> Self {
        Self::Persistence(value)
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_error_codes_are_stable() {
        let err = DomainError::InvalidPrice { value_micros: 0 };
        assert_eq!(err.code().as_str(), "DOMAIN_INVALID_PRICE");
        assert!(format!("{err}").contains("DOMAIN_INVALID_PRICE"));
    }

    #[test]
    fn app_error_wraps_domain_error() {
        let err = AppError::from(DomainError::InvalidQuantity { value: 0 });
        assert!(format!("{err}").contains("DOMAIN_INVALID_QUANTITY"));
    }

    #[test]
    fn input_error_codes_are_stable() {
        let err = InputError::Invalid {
            path: PathBuf::from("fixture.json"),
            detail: "bad json".to_string(),
        };
        assert_eq!(err.code().as_str(), "INPUT_INVALID");
        assert!(format!("{err}").contains("INPUT_INVALID"));
    }

    #[test]
    fn security_error_codes_are_stable() {
        let err = SecurityError::LiveTradingDisabled {
            mode: "live".to_string(),
        };
        assert_eq!(err.code().as_str(), "LIVE_TRADING_DISABLED");
        assert!(format!("{err}").contains("LIVE_TRADING_DISABLED"));
    }

    #[test]
    fn cli_error_codes_are_stable() {
        let err = CliError::UnknownCommand {
            command: "paper".to_string(),
        };
        assert_eq!(err.code().as_str(), "CLI_UNKNOWN_COMMAND");
        assert!(format!("{err}").contains("CLI_UNKNOWN_COMMAND"));
    }

    #[test]
    fn persistence_error_codes_are_stable() {
        let err = PersistenceError::UnsupportedSchema {
            path: PathBuf::from("schema.json"),
            found: 2,
            expected: 1,
        };
        assert_eq!(err.code().as_str(), "SCHEMA_UNSUPPORTED");
        assert!(format!("{err}").contains("SCHEMA_UNSUPPORTED"));
    }
}
