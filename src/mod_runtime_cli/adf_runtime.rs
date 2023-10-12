use std::fmt::{Display, Formatter};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use async_trait::async_trait;
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::mod_azure::azure::{adf_pipelines_get, adf_pipelines_run, get_azure_access_token_from};
use crate::mod_azure::entities::{ADFPipelineRunResponse, ADFPipelineRunStatus};
use crate::mod_ex_flow_utils::entities::ExFlowError;
use crate::mod_ex_flow_utils::utils_ex_flow::string_to_static_str;
use crate::mod_runtime_cli::interface_runtime::{
    ExFlowRuntimeActivityExecutor, ExFlowRuntimeActivityExecutorResult, ExFlowRuntimeActivityResult,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExFlowRuntimeActivityADFParam {
    pub subscription_id: String,
    pub resource_group_name: String,
    pub factory_name: String,
    pub pipeline_name: String,
    pub callback_waiting_sec_time: u64,
    pub callback_url: Option<String>,
}
impl ExFlowRuntimeActivityADFParam {
    pub fn new(
        subscription_id: &str,
        resource_group_name: &str,
        factory_name: &str,
        pipeline_name: &str,
        callback_waiting_sec_time: u64,
        callback_url: Option<String>,
    ) -> Self {
        ExFlowRuntimeActivityADFParam {
            subscription_id: subscription_id.to_string(),
            resource_group_name: resource_group_name.to_string(),
            factory_name: factory_name.to_string(),
            pipeline_name: pipeline_name.to_string(),
            callback_waiting_sec_time,

            callback_url,
        }
    }
}
pub struct ExFlowRuntimeADFActivityExecutor;

impl ExFlowRuntimeADFActivityExecutor {
    pub fn new() -> Self {
        ExFlowRuntimeADFActivityExecutor {}
    }
}

#[async_trait]
impl ExFlowRuntimeActivityExecutor<ExFlowRuntimeActivityADFParam>
    for ExFlowRuntimeADFActivityExecutor
{
    type ItemResult = (ExFlowRuntimeActivityResult, JoinHandle<()>);
    async fn run(
        &self,
        activity: &ExFlowRuntimeActivityADFParam,
    ) -> ExFlowRuntimeActivityExecutorResult<Self::ItemResult> {
        let result = adf_run_process(
            &activity.subscription_id,
            &activity.resource_group_name,
            &activity.factory_name,
            &activity.pipeline_name,
            activity.callback_waiting_sec_time,
            Some(Box::new(move |response| {
                info!("{:#?}", response);
            })),
        )
        .await;

        result
            .map(|r| {
                let result = ExFlowRuntimeActivityResult { run_id: r.run_id };
                (result, r.join_handle)
            })
            .map_err(|e| ExFlowError::new(string_to_static_str(&e.error_message)))
    }
}

#[derive(Debug)]
struct RunProcessError {
    pub error_message: String,
}
impl RunProcessError {
    pub fn new(error_message: String) -> Self {
        RunProcessError { error_message }
    }
}
impl Display for RunProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
struct RunProcessJoinHandle {
    pub run_id: String,
    pub join_handle: JoinHandle<()>,
}

type RunProcessResult<T> = Result<T, RunProcessError>;

async fn adf_run_process(
    subscription_id: &String,
    resource_group_name: &String,
    factory_name: &String,
    pipeline_name: &String,
    waiting_sec_time: u64,
    callback_fn: Option<Box<dyn Fn(&ADFPipelineRunResponse) + Send>>,
) -> RunProcessResult<RunProcessJoinHandle> {
    let access_token_response = get_azure_access_token_from(None, None).await.unwrap();
    let res_run = adf_pipelines_run(
        &access_token_response,
        subscription_id.as_str(),
        resource_group_name.as_str(),
        factory_name.as_str(),
        pipeline_name.as_str(),
    )
    .await;

    match res_run {
        Ok(res) => {
            let s = string_to_static_str(subscription_id);
            let r = string_to_static_str(resource_group_name);
            let f = string_to_static_str(factory_name);
            let run_id = string_to_static_str(&res.run_id);

            let sender = thread::spawn(move || {
                let rt = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                rt.block_on(async {
                    loop {
                        async_std::task::sleep(Duration::from_secs(waiting_sec_time)).await;
                        //sleep(Duration::from_secs(waiting_sec_time));
                        let access_token_response =
                            get_azure_access_token_from(Some(access_token_response.clone()), None)
                                .await
                                .unwrap();

                        let res_get =
                            adf_pipelines_get(&access_token_response, s, r, f, run_id).await;

                        let is_running = match res_get {
                            Ok(r) => {
                                match r.to_owned().status.unwrap_or(ADFPipelineRunStatus::Failed) {
                                    ADFPipelineRunStatus::Queued
                                    | ADFPipelineRunStatus::InProgress => {
                                        //info!("{:#?}", r);
                                        //running
                                        match callback_fn.as_ref() {
                                            None => {}
                                            Some(callback) => {
                                                callback(&r);
                                            }
                                        }
                                        true
                                    }
                                    ADFPipelineRunStatus::Succeeded => {
                                        //info!("{:#?}", r);
                                        //finish the pipeline
                                        match callback_fn.as_ref() {
                                            None => {}
                                            Some(callback) => {
                                                callback(&r);
                                            }
                                        }
                                        false
                                    }
                                    ADFPipelineRunStatus::Failed
                                    | ADFPipelineRunStatus::Canceling
                                    | ADFPipelineRunStatus::Cancelled => {
                                        //finish the pipeline with error
                                        //error!("{:#?}", r);
                                        match callback_fn.as_ref() {
                                            None => {}
                                            Some(callback) => {
                                                callback(&r);
                                            }
                                        }
                                        false
                                    }
                                }
                            }
                            Err(e) => {
                                error!("{:#?}", e);
                                false
                            }
                        };
                        if !is_running {
                            break;
                        }
                    }
                });
            });
            let res_process = RunProcessJoinHandle {
                run_id: run_id.to_string(),
                join_handle: sender,
            };
            Ok(res_process)
        }
        Err(e) => {
            error!("{:?}", e);
            Err(RunProcessError::new(
                e.error_cloud.unwrap().error_message.unwrap(),
            ))
        }
    }
}
