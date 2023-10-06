use crate::mod_runtime_api::entities::PipelineRunRequest;
use actix_web::{web, HttpResponse, Responder};
use tracing_attributes::instrument;

#[instrument]
pub async fn post_run_pipeline(_request: web::Json<PipelineRunRequest>) -> impl Responder {


    HttpResponse::Ok().finish()
}
#[instrument]
pub async fn get_status_pipeline() -> impl Responder {
    HttpResponse::Ok().finish()
}
