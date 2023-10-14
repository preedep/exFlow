use actix_web::{HttpResponse, Responder, web};
use tracing_attributes::instrument;

use crate::mod_cores::errors::{
    ExFlowError, GENERAL_FUNCTION_NOT_SUPPORTED, GENERAL_PARAM_NOT_COMPLETE,
};
use crate::mod_cores::utils::ExFlowResult;
use crate::mod_runtime::adf_runtime::{
    ExFlowRuntimeActivityADFParam, ExFlowRuntimeADFActivityExecutor,
};
use crate::mod_runtime::entities::{
    ActivityType, ExFlowRuntimeActivityWebRequest, PipelineRunResponse,
};
use crate::mod_runtime::interface_runtime::ExFlowRuntimeActivityExecutor;

#[instrument]
pub async fn post_run_pipeline(
    request: web::Json<ExFlowRuntimeActivityWebRequest>,
) -> ExFlowResult<PipelineRunResponse> {
    //
    // Call run_process is same CLI
    //
    match &request.activity_type {
        ActivityType::RunTimeAdf => match &request.adf_request {
            None => Err(ExFlowError::new(GENERAL_PARAM_NOT_COMPLETE)),
            Some(request) => {
                let param = ExFlowRuntimeActivityADFParam::new(
                    request.subscription_id.as_str(),
                    request.resource_group_name.as_str(),
                    request.factory_name.as_str(),
                    request.pipeline_name.as_str(),
                    3u64,
                    request.clone().callback_url,
                );
                let runtime_executor = ExFlowRuntimeADFActivityExecutor::new();
                let runtime_res = runtime_executor.run(&param).await;
                runtime_res.map(|result| PipelineRunResponse {
                    run_id: result.0.run_id,
                })
            }
        },
        ActivityType::RuntimeApi => Err(ExFlowError::new(GENERAL_FUNCTION_NOT_SUPPORTED)),
        ActivityType::RuntimeCli => Err(ExFlowError::new(GENERAL_FUNCTION_NOT_SUPPORTED)),
    }
}

#[instrument]
pub async fn get_status_pipeline() -> impl Responder {
    HttpResponse::Ok().finish()
}
