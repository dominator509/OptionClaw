use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::errors::PersistenceError;

use super::{
    read_text,
    state::{read_state, write_state_atomic, PaperState},
    write_atomic_text,
};

pub const SCHEMA_VERSION: u32 = 1;
pub const SCHEMA_FILE_NAME: &str = "schema.json";
pub const AUDIT_DIR_NAME: &str = "audit";
pub const AUDIT_FILE_NAME: &str = "events.jsonl";
pub const PAPER_DIR_NAME: &str = "paper";
pub const PAPER_STATE_FILE_NAME: &str = "state.json";
pub const BACKUPS_DIR_NAME: &str = "backups";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateReport {
    pub root: PathBuf,
    pub schema_version: u32,
    pub created: bool,
    pub verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationReport {
    pub root: PathBuf,
    pub from_version: u32,
    pub to_version: u32,
    pub dry_run: bool,
    pub would_modify: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct SchemaMetadata {
    schema_version: u32,
}

#[derive(Debug, Clone)]
struct Layout {
    root: PathBuf,
}

impl Layout {
    fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    fn schema_path(&self) -> PathBuf {
        self.root.join(SCHEMA_FILE_NAME)
    }

    fn audit_dir(&self) -> PathBuf {
        self.root.join(AUDIT_DIR_NAME)
    }

    fn audit_path(&self) -> PathBuf {
        self.audit_dir().join(AUDIT_FILE_NAME)
    }

    fn paper_dir(&self) -> PathBuf {
        self.root.join(PAPER_DIR_NAME)
    }

    fn state_path(&self) -> PathBuf {
        self.paper_dir().join(PAPER_STATE_FILE_NAME)
    }

    fn backups_dir(&self) -> PathBuf {
        self.root.join(BACKUPS_DIR_NAME)
    }
}

pub fn init_data_dir(root: impl AsRef<Path>) -> Result<StateReport, PersistenceError> {
    let layout = Layout::new(root);
    fs::create_dir_all(&layout.root).map_err(|source| PersistenceError::Unavailable {
        path: layout.root.clone(),
        source: Box::new(source),
    })?;
    fs::create_dir_all(layout.audit_dir()).map_err(|source| PersistenceError::Unavailable {
        path: layout.audit_dir(),
        source: Box::new(source),
    })?;
    fs::create_dir_all(layout.paper_dir()).map_err(|source| PersistenceError::Unavailable {
        path: layout.paper_dir(),
        source: Box::new(source),
    })?;
    fs::create_dir_all(layout.backups_dir()).map_err(|source| PersistenceError::Unavailable {
        path: layout.backups_dir(),
        source: Box::new(source),
    })?;

    let mut created = false;

    let schema_path = layout.schema_path();
    if schema_path.exists() {
        let _ = read_schema_metadata(&schema_path)?;
    } else {
        write_atomic_text(
            &schema_path,
            &serde_json::to_string_pretty(&SchemaMetadata {
                schema_version: SCHEMA_VERSION,
            })
            .map_err(|source| PersistenceError::Corrupt {
                path: schema_path.clone(),
                detail: source.to_string(),
            })?,
        )?;
        created = true;
    }

    let audit_path = layout.audit_path();
    if !audit_path.exists() {
        write_atomic_text(&audit_path, "")?;
        created = true;
    }

    let state_path = layout.state_path();
    if state_path.exists() {
        let _ = read_state(&state_path)?;
    } else {
        write_state_atomic(&state_path, &PaperState::default())?;
        created = true;
    }

    Ok(StateReport {
        root: layout.root,
        schema_version: SCHEMA_VERSION,
        created,
        verified: true,
    })
}

pub fn verify_data_dir(root: impl AsRef<Path>) -> Result<StateReport, PersistenceError> {
    let layout = Layout::new(root);
    if !layout.root.exists() {
        return Err(PersistenceError::Missing { path: layout.root });
    }

    let schema = read_schema_metadata(&layout.schema_path())?;
    let _ = read_state(layout.state_path())?;
    verify_audit_file(&layout.audit_path())?;

    Ok(StateReport {
        root: layout.root,
        schema_version: schema.schema_version,
        created: false,
        verified: true,
    })
}

pub fn migrate_dry_run(root: impl AsRef<Path>) -> Result<MigrationReport, PersistenceError> {
    let layout = Layout::new(root);
    let schema = read_schema_metadata(&layout.schema_path())?;
    if schema.schema_version != SCHEMA_VERSION {
        return Err(PersistenceError::UnsupportedSchema {
            path: layout.schema_path(),
            found: schema.schema_version,
            expected: SCHEMA_VERSION,
        });
    }

    Ok(MigrationReport {
        root: layout.root,
        from_version: schema.schema_version,
        to_version: SCHEMA_VERSION,
        dry_run: true,
        would_modify: false,
    })
}

fn read_schema_metadata(path: &Path) -> Result<SchemaMetadata, PersistenceError> {
    let contents = read_text(path)?;
    let schema: SchemaMetadata =
        serde_json::from_str(&contents).map_err(|source| PersistenceError::Corrupt {
            path: path.to_path_buf(),
            detail: source.to_string(),
        })?;
    if schema.schema_version != SCHEMA_VERSION {
        return Err(PersistenceError::UnsupportedSchema {
            path: path.to_path_buf(),
            found: schema.schema_version,
            expected: SCHEMA_VERSION,
        });
    }
    Ok(schema)
}

fn verify_audit_file(path: &Path) -> Result<(), PersistenceError> {
    let contents = read_text(path)?;
    if contents.trim().is_empty() {
        return Ok(());
    }

    for (line_index, line) in contents.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        serde_json::from_str::<super::audit::AuditEvent>(line).map_err(|source| {
            PersistenceError::Corrupt {
                path: path.to_path_buf(),
                detail: format!("line {}: {}", line_index + 1, source),
            }
        })?;
    }

    Ok(())
}
