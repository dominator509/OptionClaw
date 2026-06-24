use std::{
    fs,
    io::{self, Write},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::errors::PersistenceError;

pub mod audit;
pub mod backup;
pub mod schema;
pub mod state;

pub use audit::{append_audit, read_audit_events, AuditEvent, AuditEventType};
pub use backup::{backup_data_dir, BackupReport};
pub use schema::{init_data_dir, migrate_dry_run, verify_data_dir, MigrationReport, StateReport};
pub use state::{read_state, write_state_atomic, PaperState, StoredPosition};

pub const LAYER: &str = "persistence";

pub(crate) fn write_atomic_text(path: &Path, contents: &str) -> Result<(), PersistenceError> {
    let parent = path.parent().ok_or_else(|| PersistenceError::Unavailable {
        path: path.to_path_buf(),
        source: Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "path has no parent directory",
        )),
    })?;

    fs::create_dir_all(parent).map_err(|source| PersistenceError::Unavailable {
        path: parent.to_path_buf(),
        source: Box::new(source),
    })?;

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| PersistenceError::Unavailable {
            path: path.to_path_buf(),
            source: Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "system clock is before Unix epoch",
            )),
        })?
        .as_nanos();
    let temp_path = parent.join(format!(".{}.{}.tmp", std::process::id(), nanos));

    let write_result = (|| -> Result<(), PersistenceError> {
        let mut file =
            fs::File::create(&temp_path).map_err(|source| PersistenceError::Unavailable {
                path: temp_path.clone(),
                source: Box::new(source),
            })?;
        file.write_all(contents.as_bytes())
            .map_err(|source| PersistenceError::Unavailable {
                path: temp_path.clone(),
                source: Box::new(source),
            })?;
        file.sync_all()
            .map_err(|source| PersistenceError::Unavailable {
                path: temp_path.clone(),
                source: Box::new(source),
            })?;
        fs::rename(&temp_path, path).map_err(|source| PersistenceError::Unavailable {
            path: path.to_path_buf(),
            source: Box::new(source),
        })?;
        Ok(())
    })();

    if write_result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }

    write_result
}

pub(crate) fn read_text(path: &Path) -> Result<String, PersistenceError> {
    fs::read_to_string(path).map_err(|source| {
        if source.kind() == io::ErrorKind::NotFound {
            PersistenceError::Missing {
                path: path.to_path_buf(),
            }
        } else {
            PersistenceError::Unavailable {
                path: path.to_path_buf(),
                source: Box::new(source),
            }
        }
    })
}
