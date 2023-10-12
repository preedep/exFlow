use actix_web::{HttpResponse, Responder, web};
use tracing_attributes::instrument;

use crate::mod_runtime_api::entities::{
    ExFlowWebRuntimeError, PipelineRunRequest, PipelineRunResponse,
};

use crate::mod_runtime_cli::adf_runtime::ExFlowRuntimeADFActivityExecutor;
use crate::mod_runtime_cli::interface_runtime::{ExFlowRuntimeActivityADFParam, ExFlowRuntimeActivityExecutor};

type ExFlowWebRuntimeResult<T> = Result<T, ExFlowWebRuntimeError>;

#[instrument]
pub async fn post_run_pipeline(
    request: web::Json<PipelineRunRequest>,
) -> ExFlowWebRuntimeResult<PipelineRunResponse> {
    //
    // Call run_process is same CLI
    //
    /*
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
*/
    let param  = ExFlowRuntimeActivityADFParam::new(
        &request.subscription_id,
        &request.resource_group_name,
        &request.factory_name,
        &request.pipeline_name,
        3u64,
    );
    let runtime_executor =
        ExFlowRuntimeADFActivityExecutor::new();
    let  runtime_res=
        runtime_executor.run(&param).await;
    runtime_res
        .map(|result|{
            PipelineRunResponse{
                run_id: result.0.run_id,
            }
        } )
        .map_err(|e| ExFlowWebRuntimeError::new(e.to_string()))
}

#[instrument]
pub async fn get_status_pipeline() -> impl Responder {
    HttpResponse::Ok().finish()
}
