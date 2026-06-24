use crate::{
    domain::{OrderIntent, Price},
    errors::AppError,
    risk::{authorize_execution, ExecutionGates},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    Paper,
    Live,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionStatus {
    PaperFilled,
    LiveDisabled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionReport {
    pub mode: ExecutionMode,
    pub status: ExecutionStatus,
    pub intent_id: String,
    pub filled_quantity: u32,
    pub fill_price: Option<Price>,
}

pub trait PaperExecutor {
    fn execute(&self, intent: &OrderIntent) -> Result<ExecutionReport, AppError>;
}

pub trait ExecutionProvider {
    fn execute_live(&self, intent: &OrderIntent) -> Result<ExecutionReport, AppError>;
}

pub fn authorize_live_mode() -> Result<(), AppError> {
    authorize_execution(crate::domain::TradingMode::Live, ExecutionGates::default())
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FixturePaperExecutor;

impl FixturePaperExecutor {
    pub fn new() -> Self {
        Self
    }
}

impl PaperExecutor for FixturePaperExecutor {
    fn execute(&self, intent: &OrderIntent) -> Result<ExecutionReport, AppError> {
        Ok(ExecutionReport {
            mode: ExecutionMode::Paper,
            status: ExecutionStatus::PaperFilled,
            intent_id: intent.id.clone(),
            filled_quantity: intent.quantity.get(),
            fill_price: intent.limit_price.or(Some(intent.estimated_max_loss)),
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DisabledExecutionProvider;

impl DisabledExecutionProvider {
    pub fn new() -> Self {
        Self
    }
}

impl ExecutionProvider for DisabledExecutionProvider {
    fn execute_live(&self, intent: &OrderIntent) -> Result<ExecutionReport, AppError> {
        Ok(ExecutionReport {
            mode: ExecutionMode::Live,
            status: ExecutionStatus::LiveDisabled,
            intent_id: intent.id.clone(),
            filled_quantity: 0,
            fill_price: None,
        })
    }
}

pub const LAYER: &str = "execution";
