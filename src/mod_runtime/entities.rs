use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineRunRequest {
    #[serde(rename = "subscription_id")]
    pub subscription_id: String,
    #[serde(rename = "resource_group_name")]
    pub resource_group_name: String,
    #[serde(rename = "factory_name")]
    pub factory_name: String,
    #[serde(rename = "pipeline_name")]
    pub pipeline_name: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineRunResponse {
    #[serde(rename = "run_id")]
    pub run_id: String,
}

impl Responder for PipelineRunResponse {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        // Create response and set content type
        HttpResponse::Ok().json(&self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityType {
    RunTimeAdf,
    RuntimeApi,
    RuntimeCli,
}

impl ToString for ActivityType {
    fn to_string(&self) -> String {
        match self {
            ActivityType::RunTimeAdf => "RunTimeAdf".to_string(),
            ActivityType::RuntimeApi => "RuntimeApi".to_string(),
            ActivityType::RuntimeCli => "RuntimeCli".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExFlowRuntimeActivityWebRequest {
    #[serde(rename = "activity_type")]
    pub activity_type: ActivityType,

    #[serde(rename = "runtime_activity_adf_request")]
    pub adf_request: Option<PipelineRunRequest>,
}
