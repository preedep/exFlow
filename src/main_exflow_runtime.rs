use clap::Parser;

use crate::mod_runtime::runtime_cli::ExFlowRuntimeArgs;

mod mod_azure;
mod mod_runtime;
mod mod_cores;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let args = ExFlowRuntimeArgs::parse();
    args.run().await
}
