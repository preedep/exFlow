use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::mod_ex_flow_utils::entities::ExFlowError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExFlowRuntimeActivityResult {
    pub run_id: String,
}

pub type ExFlowRuntimeActivityExecutorResult<T> = Result<T, ExFlowError>;

/*
pub trait RunProcessCallback {
    fn on_completed(&self);
    fn on_failed(&self);
    fn on_running(&self);
}
*/

#[async_trait]
pub trait ExFlowRuntimeActivityExecutor<T> {
    type ItemResult;
    async fn run(&self, activity: &T) -> ExFlowRuntimeActivityExecutorResult<Self::ItemResult>;
}
