use actix_web::{HttpResponse, Responder, web};
use tracing_attributes::instrument;
use crate::mod_ex_flow_utils::errors::{FUNCTION_NOT_SUPPORTED, PARAM_NOT_COMPLETE};

use crate::mod_runtime_api::entities::{ActivityType, ExFlowRuntimeActivityWebRequest, ExFlowRuntimeWebError, PipelineRunRequest, PipelineRunResponse};
use crate::mod_runtime_cli::adf_runtime::ExFlowRuntimeADFActivityExecutor;
use crate::mod_runtime_cli::interface_runtime::{ExFlowRuntimeActivityADFParam, ExFlowRuntimeActivityExecutor};

type ExFlowWebRuntimeResult<T> = Result<T, ExFlowRuntimeWebError>;

#[instrument]
pub async fn post_run_pipeline(
    request: web::Json<ExFlowRuntimeActivityWebRequest>,
) -> ExFlowWebRuntimeResult<PipelineRunResponse> {
    //
    // Call run_process is same CLI
    //
    match &request.activity_type {
        ActivityType::RunTimeAdf => {
            match &request.adf_request {
                None => {
                    Err(ExFlowRuntimeWebError::new(PARAM_NOT_COMPLETE.to_string()))
                }
                Some(request) => {
                    let param  = ExFlowRuntimeActivityADFParam::new(
                        request.subscription_id.as_str(),
                        request.resource_group_name.as_str(),
                        request.factory_name.as_str(),
                        request.pipeline_name.as_str(),
                        3u64,
                        request.clone().callback_url
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
                        .map_err(|e| ExFlowRuntimeWebError::new(e.to_string()))
                }
            }
        }
        ActivityType::RuntimeApi => {
            Err(ExFlowRuntimeWebError::new(FUNCTION_NOT_SUPPORTED.to_string()))
        }
        ActivityType::RuntimeCli => {
            Err(ExFlowRuntimeWebError::new(FUNCTION_NOT_SUPPORTED.to_string()))
        }
    }
}

#[instrument]
pub async fn get_status_pipeline() -> impl Responder {
    HttpResponse::Ok().finish()
}
