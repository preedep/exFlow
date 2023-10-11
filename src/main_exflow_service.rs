use crate::mod_service_cli::service_cli::ExFlowServiceArgs;
use clap::Parser;

mod mod_service_api;
mod mod_service_cli;
mod mod_ex_flow_utils;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let args = ExFlowServiceArgs::parse();
    args.run().await
}
