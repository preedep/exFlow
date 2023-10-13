use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::mod_service_api::entities::ExFlowRuntimeRegisterRequest;

///
/// TblExFlowRuntimeClients is struct representing tbl_exflow_runtime_clients
/// Collect data when ExFlowRuntime is initialized and Register to ExFlow Service
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TblExFlowRuntimeClients {
    pub client_id: String,
    pub host_name: String,
    pub host_ip: String,
    pub register_id : String,
    pub created_dt : time::OffsetDateTime,
    pub updated_dt : time::OffsetDateTime,
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
