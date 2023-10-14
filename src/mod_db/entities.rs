use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::mod_cores::web_data::ExFlowRuntimeRegisterRequest;

///
/// TblExFlowRuntimeClients is struct representing tbl_exflow_runtime_clients
/// Collect data when ExFlowRuntime is initialized and Register to ExFlow Service
///
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TblExFlowRuntimeClients {
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "host_name")]
    pub host_name: String,
    #[serde(rename = "host_ip")]
    pub host_ip: String,
    #[serde(rename = "created_dt")]
    pub created_dt: Option<chrono::NaiveDateTime>,
    #[serde(rename = "updated_dt")]
    pub updated_dt: Option<chrono::NaiveDateTime>,
}
impl From<ExFlowRuntimeRegisterRequest> for TblExFlowRuntimeClients {
    fn from(value: ExFlowRuntimeRegisterRequest) -> Self {
        TblExFlowRuntimeClients {
            client_id: value.client_id,
            host_name: value.host_name.unwrap_or("".to_string()),
            host_ip: value.host_ip,
            created_dt: None,
            updated_dt: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TblExFlowApps {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TblExFlowJobs {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TblExFlowClientIDs {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TblExFlowJobHistory {}
