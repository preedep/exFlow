use actix_web::{HttpResponse, Responder, web};
use log::info;
use tracing_attributes::instrument;

use crate::mod_runtime_api::entities::{
    ExFlowWebRuntimeError, PipelineRunRequest, PipelineRunResponse,
};
use crate::mod_runtime_cli::runtime_cli::run_process;

type ExFlowWebRuntimeResult<T> = Result<T, ExFlowWebRuntimeError>;

#[instrument]
pub async fn post_run_pipeline(
    request: web::Json<PipelineRunRequest>,
) -> ExFlowWebRuntimeResult<PipelineRunResponse> {
    //
    // Call run_process is same CLI
    //
    let result = run_process(
        &request.subscription_id,
        &request.resource_group_name,
        &request.factory_name,
        &request.pipeline_name,
        3u64,
        Some(Box::new(|resp| {
            info!("{:#?}", resp);
        })),
    )
        .await;

    result
        .map(|result| PipelineRunResponse {
            run_id: result.run_id,
        })
        .map_err(|e| ExFlowWebRuntimeError::new(e.to_string()))
}

#[instrument]
pub async fn get_status_pipeline() -> impl Responder {
    HttpResponse::Ok().finish()
}
