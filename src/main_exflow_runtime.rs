use crate::mod_runtime_api::runtime_api::{get_status_pipeline, post_run_pipeline};
use actix_web::middleware::Logger;
use actix_web::{middleware, web, App, HttpServer};
use actix_web_opentelemetry::RequestTracing;
use clap::Parser;

use crate::mod_runtime_cli::runtime_cli::{run_process, Commands, ExFlowRuntimeArgs};
use log::{debug, error, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use crate::mod_utils::utils::set_global_tracing;

mod mod_azure;
mod mod_runtime_api;
mod mod_runtime_cli;

mod mod_utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let args = ExFlowRuntimeArgs::parse();

    match &args.command {
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
            exflow_service_endpoint,
            apm_connection_string,
        }) => {
            info!("Run with Web Server mode");
            info!("ExFlow Runtime starting....");
            info!("Registering.. to exFlow service");
            ///
            /*
            if apm_connection_string.len() > 0 {
                debug!("APPLICATIONINSIGHTS_CON_STRING = {}", apm_connection_string);
                let exporter =
                    opentelemetry_application_insights::new_pipeline_from_connection_string(
                        apm_connection_string,
                    )
                    .unwrap()
                    .with_client(reqwest::Client::new())
                    .with_service_name("ExFlow-Runtime")
                    .install_batch(opentelemetry::runtime::Tokio);

                let telemetry = tracing_opentelemetry::layer().with_tracer(exporter);
                let subscriber = Registry::default().with(telemetry);
                tracing::subscriber::set_global_default(subscriber)
                    .expect("setting global default failed");
            }*/
            set_global_tracing(&apm_connection_string);
            ////
            HttpServer::new(|| {
                App::new()
                    .wrap(
                        middleware::DefaultHeaders::new().add(("ExFlow-Runtime-X-Version", "0.1")),
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
            .bind(("0.0.0.0", 8082))?
            .run()
            .await
        }
    }
}
