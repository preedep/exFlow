use clap::Parser;

use crate::mod_service_cli::service_cli::ExFlowServiceArgs;

mod mod_service_api;
mod mod_service_cli;
mod mod_ex_flow_utils;

mod mod_azure;
mod mod_runtime_api;
mod mod_runtime_cli;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let args = ExFlowServiceArgs::parse();
    args.run().await
}
