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
use crate::mod_runtime_cli::adf_runtime::ExFlowRuntimeADFActivityExecutor;
use crate::mod_runtime_cli::interface_runtime::{ExFlowRuntimeActivityADFParam, ExFlowRuntimeActivityExecutor, ExFlowRuntimeActivityExecutorResult};
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

                let param  = ExFlowRuntimeActivityADFParam::new(
                    subscription_id,
                    resource_group_name,
                    factory_name,
                    pipeline_name,
                    3u64,
                );
                let runtime_executor =
                    ExFlowRuntimeADFActivityExecutor::new();

                let  runtime_res=
                    runtime_executor.run(&param).await;
                match runtime_res {
                    Ok(r) => {
                        info!("Run with CLI arguments successfully > run_id [{:#?}]",r.0.run_id);
                        r.1.join().expect("Runtime activity waiting failed");
                    }
                    Err(e) => {
                        error!("Runtime activity failed : {:#?}",e);
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
