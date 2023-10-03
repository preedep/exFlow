use crate::mod_runtime_api::runtime_api::{get_status_pipeline, post_run_pipeline};
use actix_web::middleware::Logger;
use actix_web::{middleware, web, App, HttpServer};
use clap::Parser;

use log::{debug, error, info};
use crate::mod_runtime_cli::runtime_cli::{Commands, ExFlowArgs, run_process };


mod mod_azure;
mod mod_runtime_api;
mod mod_runtime_cli;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let args = ExFlowArgs::parse();

    match &args.command {
        None => {
            println!("Exflow runtime support 2 modes [CLI or Runtime] , Please use --help for more information");
            Ok(())
        }
        Some(Commands::Cli {subscription_id,resource_group_name,factory_name,pipeline_name }) => {
            info!("Run with CLI arguments");
            Ok(())
        }
        Some(Commands::Runtime {exflow_service_endpoint}) => {
            info!("Run with Web Server mode");
            info!("ExFlow Runtime starting....");
            info!("Registering.. to exFlow service");
            HttpServer::new(|| {
                App::new()
                    .wrap(Logger::default())
                    .wrap(Logger::new("%a %{User-Agent}i"))
                    .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.1")))
                    .service(web::scope("/api/v1")
                        .route("/run_pipeline",web::post().to(post_run_pipeline))
                        .route("/get_status",web::get().to(get_status_pipeline))
                    )
            }).workers(10)
                .bind(("0.0.0.0",8082))?.run().await
        }
    }
}
