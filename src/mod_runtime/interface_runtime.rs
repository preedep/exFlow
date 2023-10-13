use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::mod_utils::errors::ExFlowError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExFlowRuntimeActivityResult {
    pub run_id: String,
}
pub type ExFlowRuntimeActivityExecutorResult<T> = Result<T, ExFlowError>;

#[async_trait]
pub trait ExFlowRuntimeActivityExecutor<T> {
    type ItemResult;
    async fn run(&self, activity: &T) -> ExFlowRuntimeActivityExecutorResult<Self::ItemResult>;
}
