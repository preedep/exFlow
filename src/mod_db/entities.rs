use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TblExFlowApps {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TblExFlowJobs {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TblExFlowClientIDs {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TblExFlowJobHistory {}
