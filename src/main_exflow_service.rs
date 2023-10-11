use clap::Parser;
use crate::mod_service_cli::service_cli::ExFlowServiceArgs;

mod mod_service_api;
mod mod_service_cli;
mod mod_utils;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let args = ExFlowServiceArgs::parse();
    args.run();
}
