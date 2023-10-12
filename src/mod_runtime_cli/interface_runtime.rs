use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::mod_ex_flow_utils::entities::ExFlowError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExFlowRuntimeActivityResult {
    pub run_id: String,
}

pub type ExFlowRuntimeActivityExecutorResult<T> = Result<T, ExFlowError>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExFlowRuntimeActivityADFParam {
    pub subscription_id: String,
    pub resource_group_name: String,
    pub factory_name: String,
    pub pipeline_name: String,
    pub callback_waiting_sec_time: u64,
    pub callback_url: Option<String>,
}
impl ExFlowRuntimeActivityADFParam {
    pub fn new(
        subscription_id: &str,
        resource_group_name: &str,
        factory_name: &str,
        pipeline_name: &str,
        callback_waiting_sec_time: u64,
        callback_url: Option<String>,
    ) -> Self {
        ExFlowRuntimeActivityADFParam {
            subscription_id: subscription_id.to_string(),
            resource_group_name: resource_group_name.to_string(),
            factory_name: factory_name.to_string(),
            pipeline_name: pipeline_name.to_string(),
            callback_waiting_sec_time,

            callback_url,
        }
    }
}

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
