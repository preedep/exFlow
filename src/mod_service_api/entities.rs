
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExFlowRuntimeRegisterRequest {
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "host_name")]
    pub host_name: Option<String>,
    #[serde(rename = "host_ip")]
    pub host_ip: String,
}
