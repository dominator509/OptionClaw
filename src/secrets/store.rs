use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};

use crate::errors::{AppError, SecurityError};

use super::redaction::SecretString;

pub trait SecretStore {
    fn load(&self, name: &str) -> Result<SecretString, AppError>;
    fn store(&self, name: &str, value: SecretString) -> Result<(), AppError>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DisabledSecretStore;

impl SecretStore for DisabledSecretStore {
    fn load(&self, name: &str) -> Result<SecretString, AppError> {
        Err(SecurityError::SecretMissing {
            name: name.to_string(),
        }
        .into())
    }

    fn store(&self, _name: &str, _value: SecretString) -> Result<(), AppError> {
        Err(SecurityError::SecretStorageDisabled { path: None }.into())
    }
}

#[derive(Debug, Default)]
pub struct MemorySecretStore {
    secrets: Mutex<HashMap<String, String>>,
}

impl MemorySecretStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SecretStore for MemorySecretStore {
    fn load(&self, name: &str) -> Result<SecretString, AppError> {
        let guard = self
            .secrets
            .lock()
            .expect("secret store mutex should be usable");
        let Some(value) = guard.get(name) else {
            return Err(SecurityError::SecretMissing {
                name: name.to_string(),
            }
            .into());
        };
        Ok(SecretString::new(value.clone()))
    }

    fn store(&self, name: &str, value: SecretString) -> Result<(), AppError> {
        let mut guard = self
            .secrets
            .lock()
            .expect("secret store mutex should be usable");
        guard.insert(name.to_string(), value.into_inner());
        Ok(())
    }
}

pub fn reject_plaintext_secret_file(path: impl AsRef<Path>) -> Result<(), AppError> {
    let path = path.as_ref();
    require_restrictive_permissions(path)?;
    let contents = fs::read_to_string(path).map_err(|_| SecurityError::SecretStorageDisabled {
        path: Some(path.to_path_buf()),
    })?;

    let trimmed = contents.trim();
    if trimmed.starts_with("optionclaw-secret-v1:") {
        Ok(())
    } else {
        Err(SecurityError::SecretPlaintextRejected {
            path: path.to_path_buf(),
        }
        .into())
    }
}

pub fn require_restrictive_permissions(path: impl AsRef<Path>) -> Result<(), AppError> {
    let path = path.as_ref();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(path).map_err(|_| SecurityError::SecretStorageDisabled {
            path: Some(path.to_path_buf()),
        })?;
        let mode = metadata.permissions().mode();
        if mode & 0o077 != 0 {
            return Err(SecurityError::InsecureFilePermissions {
                path: path.to_path_buf(),
            }
            .into());
        }
    }

    #[cfg(windows)]
    {
        let _ = path;
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecretStoreReport {
    pub root: PathBuf,
    pub configured: bool,
    pub encrypted: bool,
}

pub fn describe_secret_store(root: impl AsRef<Path>) -> SecretStoreReport {
    SecretStoreReport {
        root: root.as_ref().to_path_buf(),
        configured: false,
        encrypted: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(unix)]
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[cfg(unix)]
    fn unique_temp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be on or after epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("optionclaw-secrets-{name}-{nanos}"));
        if path.exists() {
            let _ = fs::remove_dir_all(&path);
        }
        fs::create_dir_all(&path).expect("temp dir should be creatable");
        path
    }

    #[test]
    fn disabled_secret_store_fails_closed() {
        let store = DisabledSecretStore;
        let err = store
            .load("broker_api_key")
            .expect_err("missing secret should fail");
        assert!(format!("{err}").contains("SECRET_MISSING"));
    }

    #[test]
    fn memory_secret_store_redacts_display() {
        let store = MemorySecretStore::new();
        store
            .store("llm_api_key", "opclaw_fake_key".into())
            .expect("store should succeed");

        let secret = store.load("llm_api_key").expect("load should succeed");
        assert_eq!(secret.expose(), "opclaw_fake_key");
        assert_eq!(format!("{secret}"), "<redacted>");
    }

    #[cfg(unix)]
    #[test]
    fn restrictive_permissions_are_required_for_secret_files() {
        use std::os::unix::fs::PermissionsExt;

        let root = unique_temp_dir("permissions");
        let secret_path = root.join("secret.txt");
        fs::write(&secret_path, "optionclaw-secret-v1:encrypted")
            .expect("secret file should write");

        let mut permissions = fs::metadata(&secret_path)
            .expect("secret file should exist")
            .permissions();
        permissions.set_mode(0o644);
        fs::set_permissions(&secret_path, permissions).expect("permissions should update");

        let err = require_restrictive_permissions(&secret_path)
            .expect_err("insecure permissions should fail");
        assert!(format!("{err}").contains("INSECURE_FILE_PERMISSIONS"));
    }
}
