use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::errors::PersistenceError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupReport {
    pub source: PathBuf,
    pub backup: PathBuf,
    pub files_copied: usize,
}

pub fn backup_data_dir(
    source: impl AsRef<Path>,
    backup: impl AsRef<Path>,
) -> Result<BackupReport, PersistenceError> {
    let source = source.as_ref();
    let backup = backup.as_ref();

    if !source.exists() {
        return Err(PersistenceError::Missing {
            path: source.to_path_buf(),
        });
    }

    if backup.exists() {
        return Err(PersistenceError::BackupExists {
            path: backup.to_path_buf(),
        });
    }

    if backup.starts_with(source) {
        return Err(PersistenceError::BackupFailed {
            path: backup.to_path_buf(),
            source: Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "backup target must not be inside the source directory",
            )),
        });
    }

    fs::create_dir_all(backup).map_err(|source_err| PersistenceError::BackupFailed {
        path: backup.to_path_buf(),
        source: Box::new(source_err),
    })?;

    let files_copied = copy_tree(source, backup)?;

    Ok(BackupReport {
        source: source.to_path_buf(),
        backup: backup.to_path_buf(),
        files_copied,
    })
}

fn copy_tree(source: &Path, destination: &Path) -> Result<usize, PersistenceError> {
    let mut copied = 0;
    for entry in fs::read_dir(source).map_err(|source_err| PersistenceError::BackupFailed {
        path: source.to_path_buf(),
        source: Box::new(source_err),
    })? {
        let entry = entry.map_err(|source_err| PersistenceError::BackupFailed {
            path: source.to_path_buf(),
            source: Box::new(source_err),
        })?;
        let file_type = entry
            .file_type()
            .map_err(|source_err| PersistenceError::BackupFailed {
                path: entry.path(),
                source: Box::new(source_err),
            })?;
        let target = destination.join(entry.file_name());
        if file_type.is_dir() {
            fs::create_dir_all(&target).map_err(|source_err| PersistenceError::BackupFailed {
                path: target.clone(),
                source: Box::new(source_err),
            })?;
            copied += copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), &target).map_err(|source_err| {
                PersistenceError::BackupFailed {
                    path: target.clone(),
                    source: Box::new(source_err),
                }
            })?;
            copied += 1;
        }
    }
    Ok(copied)
}
