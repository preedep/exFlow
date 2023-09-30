use actix_web::{HttpResponse, Responder, web};
use crate::mod_azure::azure::adf_pipelines_run;
use crate::mod_runtime_api::entities::PipelineRunRequest;

pub async fn post_run_pipeline(request: web::Json<PipelineRunRequest>) -> impl Responder {
     let res = adf_pipelines_run(request.subscription_id.as_str(),
    request.resource_group_name.as_str(),
    request.factory_name.as_str(),request.pipeline_name.as_str()).await;
     HttpResponse::Ok().finish()
}
pub async  fn get_status_pipeline() -> impl Responder {
    HttpResponse::Ok().finish()
}