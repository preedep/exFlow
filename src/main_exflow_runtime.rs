use clap::Parser;

use crate::mod_runtime_cli::runtime_cli::ExFlowRuntimeArgs;

mod mod_azure;
mod mod_runtime_api;
mod mod_runtime_cli;
mod mod_utils;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let args = ExFlowRuntimeArgs::parse();
    args.run().await
}
