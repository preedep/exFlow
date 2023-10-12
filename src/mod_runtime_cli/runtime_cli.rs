use std::fmt::{Display, Formatter};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use actix_web::{App, HttpServer, middleware, web};
use actix_web::middleware::Logger;
use actix_web_opentelemetry::RequestTracing;
use clap::{command, Parser, Subcommand};
use http::StatusCode;
use log::{debug, error, info};

use crate::mod_azure::azure::{adf_pipelines_get, adf_pipelines_run, get_azure_access_token_from};
use crate::mod_azure::entities::{ADFPipelineRunResponse, ADFPipelineRunStatus};
use crate::mod_ex_flow_utils::uri::{EX_FLOW_SERVICE_API_IR_REGISTER, EX_FLOW_SERVICE_API_SCOPE};
use crate::mod_ex_flow_utils::utils_ex_flow::{get_system_info, set_global_apm_tracing};
use crate::mod_runtime_api::runtime_api::{get_status_pipeline, post_run_pipeline};
use crate::mod_service_api::entities::ExFlowRuntimeRegisterRequest;

const SERVICE_NAME: &'static str = "ExFlow-Runtime";

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
pub struct ExFlowRuntimeArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run in mode web service
    Runtime {
        /// exFlow Service Endpoint , with Port number [Ex. localhost:8082)
        #[arg(short, long)]
        ex_flow_service_endpoint: String,

        /// exFlow Runtime Client Id Get from ExFlow Portal
        #[arg(short, long)]
        client_id: String,
        /// Run with specific port
        #[arg(short, long, default_value = "8082")]
        port_number: u16,
        /// Azure application insights connection string
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

impl ExFlowRuntimeArgs {
    pub async fn run(&self) -> std::io::Result<()> {
        match &self.command {
            None => {
                println!("Exflow runtime support 2 modes [CLI or Runtime] , Please use --help for more information");
                Ok(())
            }
            Some(Commands::Cli {
                     subscription_id,
                     resource_group_name,
                     factory_name,
                     pipeline_name,
                 }) => {
                info!("Run with CLI arguments");
                let run_process_result = run_process(
                    subscription_id,
                    resource_group_name,
                    factory_name,
                    pipeline_name,
                    3u64,
                    Some(Box::new(move |response| {
                        info!("{:#?}", response);
                    })),
                )
                    .await;
                match run_process_result {
                    Ok(r) => {
                        info!("Waiting for process [{}] to finish", r.run_id);
                        r.join_handle.join().expect("Failed to join");
                    }
                    Err(e) => {
                        error!("Failed to run process {:?}", e);
                    }
                }
                Ok(())
            }
            Some(Commands::Runtime {
                     ex_flow_service_endpoint,
                     client_id,
                     port_number,
                     apm_connection_string,
                 }) => {
                info!("Run with Web Server mode");
                info!("ExFlow Runtime starting....");

                set_global_apm_tracing(apm_connection_string.as_str(), SERVICE_NAME);
                //info!("Registering.. to exFlow service [{}]",ex_flow_service_endpoint);
                let sys_info = get_system_info();
                let end_point = format!("http://{}{}{}",
                                        ex_flow_service_endpoint,
                                        EX_FLOW_SERVICE_API_SCOPE,
                                        EX_FLOW_SERVICE_API_IR_REGISTER);
                debug!("Registering... to exFlow service [{}]",end_point);

                match sys_info {
                    Ok(s) => {

                        let request = ExFlowRuntimeRegisterRequest::new(client_id.as_str(),&s);

                        let  register_res= reqwest::Client::new()
                            .post(end_point)
                            .json(&request)
                            .send().await;

                        match register_res {
                            Ok(r) => {
                                let is_register_complete = r.status() == StatusCode::OK;
                                if  is_register_complete {
                                    info!("Registering... to exFlow service [{:#?}]",r);
                                }else{
                                    panic!("Cannot register ExFlowRuntime : {:#?}",r);
                                }
                            }
                            Err(e) => {
                                panic!("Cannot register ExFlowRuntime {:?}",e);
                            }
                        }
                    }
                    Err(e) => {
                        panic!("Get system info failed : {}",e);
                    }
                }
                info!("ExFlow Runtime Started");
                HttpServer::new(|| {
                    App::new()
                        .wrap(
                            middleware::DefaultHeaders::new()
                                .add(("ExFlow-Runtime-X-Version", "0.1")),
                        )
                        .wrap(Logger::default())
                        .wrap(Logger::new(
                            r#"%a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#,
                        ))
                        .wrap(RequestTracing::new())
                        .service(
                            web::scope("/api/v1")
                                .route("/run_pipeline", web::post().to(post_run_pipeline))
                                .route("/get_status", web::get().to(get_status_pipeline)),
                        )
                })
                    .workers(10)
                    .bind(("0.0.0.0", *port_number))?
                    .run()
                    .await
            }
        }
    }
}
