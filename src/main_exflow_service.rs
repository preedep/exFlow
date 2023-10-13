use clap::Parser;

use crate::mod_service::service_cli::ExFlowServiceArgs;

mod mod_service;
mod mod_utils;

mod mod_db;

mod mod_azure;
mod mod_runtime;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let args = ExFlowServiceArgs::parse();
    args.run().await
}
