use clap::Parser;

use crate::mod_runtime::runtime_cli::ExFlowRuntimeArgs;

mod mod_azure;
mod mod_utils;
mod mod_service;
mod mod_runtime;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let args = ExFlowRuntimeArgs::parse();
    args.run().await
}
