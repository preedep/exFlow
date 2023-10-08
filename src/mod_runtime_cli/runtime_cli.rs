use crate::mod_azure::azure::{adf_pipelines_get, adf_pipelines_run, get_azure_access_token_from};
use crate::mod_azure::entities::{ADFPipelineRunResponse, ADFPipelineRunStatus};
use clap::{command, Parser, Subcommand};
use log::{error, info};
use std::fmt::{Display, Formatter};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

/// Simple program to greet a person
#[derive(Parser)]
#[command(bin_name = "exflow_runtime")]
#[command(name = "exFlow Runtime")]
#[command(author = "Preedee Ponchevin <preedee.digital@gmail.com>")]
#[command(version = "1.0")]
#[command(
    about = "ExFlow (Extended) Flow , Runtime for integration with ADF , Step Function , etc."
)]
#[command(propagate_version = true)]
#[command(
    help_template = "{about-section}Version: {version} \n {author} \n\n {usage-heading} {usage} \n {all-args} {tab}"
)]
pub struct ExFlowArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run in mode web service
    Runtime {
        /// exFlow Service Endpoint
        #[arg(short, long)]
        exflow_service_endpoint: String,
        #[arg(short, long, required = false)]
        apm_connection_string: String,
    },
    /// Run with specific resource
    Cli {
        /// Subscription Id
        #[arg(short, long)]
        subscription_id: String,

        /// Resource Group Name
        #[arg(short, long)]
        resource_group_name: String,

        /// Factory Name
        #[arg(short, long)]
        factory_name: String,

        /// Pipeline Name
        #[arg(short, long)]
        pipeline_name: String,
    },
}

#[derive(Debug)]
pub struct RunProcessError {
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

pub struct RunProcessJoinHandle {
    pub run_id: String,
    pub join_handle: JoinHandle<()>,
}
pub type RunProcessResult<T> = Result<T, RunProcessError>;

pub trait RunProcessCallback {
    fn on_completed(&self);
    fn on_failed(&self);
    fn on_running(&self);
}

fn string_to_static_str(s: &String) -> &'static str {
    Box::leak(s.clone().into_boxed_str())
}
pub async fn run_process(
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
