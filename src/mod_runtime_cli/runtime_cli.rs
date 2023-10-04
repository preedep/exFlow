use crate::mod_azure::azure::{adf_pipelines_get, adf_pipelines_run};
use crate::mod_azure::entities::{ADFPipelineRunStatus, ADFResult};
use clap::{command, Parser, Subcommand};
use log::{debug, error, info};
use std::thread::sleep;
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

fn string_to_static_str(s: &String) -> &'static str {
    Box::leak(s.clone().into_boxed_str())
}
pub async fn run_process(
    subscription_id: &String,
    resource_group_name: &String,
    factory_name: &String,
    pipeline_name: &String,
) {
    let res_run = adf_pipelines_run(
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
            let handle = tokio::spawn(async move {
                loop {
                    sleep(Duration::from_secs(3));
                    let res_get = adf_pipelines_get(s, r, f, res.run_id.as_str()).await;
                    match res_get {
                        Ok(r) => {
                            match r.to_owned().status.unwrap_or(ADFPipelineRunStatus::Failed) {
                                ADFPipelineRunStatus::Queued | ADFPipelineRunStatus::InProgress => {
                                    info!("{:?}", r);
                                }
                                ADFPipelineRunStatus::Succeeded => {
                                    info!("{:?}", r);
                                    break;
                                }
                                ADFPipelineRunStatus::Failed
                                | ADFPipelineRunStatus::Canceling
                                | ADFPipelineRunStatus::Cancelled => {
                                    error!("{:?}", r);
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            error!("{:?}", e);
                            break;
                        }
                    }
                }
            })
            .await;
        }
        Err(e) => {
            error!("{:?}", e);
        }
    }
}
