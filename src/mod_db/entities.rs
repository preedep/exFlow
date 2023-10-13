use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::mod_utils::web_data::ExFlowRuntimeRegisterRequest;

///
/// TblExFlowRuntimeClients is struct representing tbl_exflow_runtime_clients
/// Collect data when ExFlowRuntime is initialized and Register to ExFlow Service
///
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TblExFlowRuntimeClients {
    #[serde(rename="client_id")]
    pub client_id: String,
    #[serde(rename="host_name")]
    pub host_name: String,
    #[serde(rename="host_ip")]
    pub host_ip: String,
    #[serde(rename="register_id")]
    pub register_id: String,
    #[serde(rename="created_dt")]
    pub created_dt: time::OffsetDateTime,
    #[serde(rename="updated_dt")]
    pub updated_dt: time::OffsetDateTime,
}

impl From<ExFlowRuntimeRegisterRequest> for TblExFlowRuntimeClients {
    fn from(value: ExFlowRuntimeRegisterRequest) -> Self {
        let id = Uuid::new_v4();
        TblExFlowRuntimeClients {
            client_id: value.client_id,
            host_name: value.host_name.unwrap_or("".to_string()),
            host_ip: value.host_ip,
            register_id: id.to_string(),
            created_dt: OffsetDateTime::now_utc(),
            updated_dt: OffsetDateTime::now_utc(),
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
