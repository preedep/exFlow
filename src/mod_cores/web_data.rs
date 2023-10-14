use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::mod_cores::utils::SystemInformation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExFlowRuntimeRegisterRequest {
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "host_name")]
    pub host_name: Option<String>,
    #[serde(rename = "host_ip")]
    pub host_ip: String,
}

impl ExFlowRuntimeRegisterRequest {
    pub fn new(client_d: &str, sys_info: &SystemInformation) -> Self {
        ExFlowRuntimeRegisterRequest {
            client_id: client_d.to_string(),
            host_name: Some(sys_info.clone().host_name),
            host_ip: sys_info.clone().host_ip,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExFlowRuntimeRegisterResponse {
    #[serde(rename = "row_effected")]
    pub row_effected: u64,
}
impl Responder for ExFlowRuntimeRegisterResponse {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(&self)
    }
}
