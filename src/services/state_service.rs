use std::path::Path;

use crate::{
    errors::AppError,
    persistence::{
        init_data_dir as persist_init_data_dir, verify_data_dir as persist_verify_data_dir,
        StateReport,
    },
};

pub fn init_state(path: impl AsRef<Path>) -> Result<StateReport, AppError> {
    Ok(persist_init_data_dir(path)?)
}

pub fn verify_state(path: impl AsRef<Path>) -> Result<StateReport, AppError> {
    Ok(persist_verify_data_dir(path)?)
}
