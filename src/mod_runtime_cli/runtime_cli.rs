use clap::{Parser, Subcommand};
/// Simple program to greet a person
#[derive(Parser)]
#[command(author = "Preedee Ponchevin <preedee.digital@gmail.com>", version = "0.1.0", about, long_about = None)]
pub struct RuntimeArgs {
    /// exFlow Service Endpoint
    #[arg(short, long)]
    exflow_service_endpoint: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run runtime with specific resource
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

