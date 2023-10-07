use actix_web::{CustomizeResponder, error, HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use derive_more::{Display, Error};

use serde::{Deserialize, Serialize};


#[derive(Debug, Display, Error)]
pub struct ExFlowWebRuntimeError {

}
impl error::ResponseError for ExFlowWebRuntimeError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}

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