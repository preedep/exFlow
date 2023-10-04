use crate::mod_azure::azure::adf_pipelines_run;
use crate::mod_runtime_api::entities::PipelineRunRequest;
use actix_web::{web, HttpResponse, Responder};

pub async fn post_run_pipeline(_request: web::Json<PipelineRunRequest>) -> impl Responder {
    HttpResponse::Ok().finish()
}
pub async fn get_status_pipeline() -> impl Responder {
    HttpResponse::Ok().finish()
}
