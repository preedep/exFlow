use clap::{command, Parser, Subcommand};
/// Simple program to greet a person
#[derive(Parser)]
#[command(bin_name = "exflow_runtime")]
#[command(name = "exFlow Runtime")]
#[command(author = "Preedee Ponchevin <preedee.digital@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "exFlow (Extended) Flow , Runtime for integration with ADF , Step Function , etc.")]
#[command(propagate_version = true)]
#[command(
help_template = "{author-with-newline} {about-section}Version: {version} \n {usage-heading} {usage} \n {all-args} {tab}"
)]
pub struct ExFlowArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run in mode web service
    Runtime {
        /// exFlow Service Endpoint
        #[arg(short, long)]
        exflow_service_endpoint: String,
    },
    /// Run with specific resource
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

pub async fn run_process(command: &Commands) {

}