use crate::mod_runtime_api::entities::PipelineRunRequest;
use actix_web::{web, HttpResponse, Responder};
use tracing_attributes::instrument;
use log::info;

use crate::run_process;
#[instrument]
pub async fn post_run_pipeline(request: web::Json<PipelineRunRequest>) -> impl Responder {
    //
    // Call run_process is same CLI
    //
    let _result = run_process(&request.subscription_id,
    &request.resource_group_name,
        &request.factory_name,
        &request.pipeline_name,3u64,Some(Box::new(|resp|{
            info!("{:#?}",resp);
        }))).await;
    //result.unwrap().join_handle.join().expect("TODO: panic message");
    HttpResponse::Ok().finish()
}
#[instrument]
pub async fn get_status_pipeline() -> impl Responder {
    HttpResponse::Ok().finish()
}
