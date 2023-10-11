use clap::{Parser, Subcommand};
use log::{debug, info};
use crate::mod_utils::utils::set_global_tracing;


/// Simple program to greet a person
#[derive(Parser)]
#[command(bin_name = "exflow_service")]
#[command(name = "exFlow Service")]
#[command(author = "Preedee Ponchevin <preedee.digital@gmail.com>")]
#[command(version = "1.0")]
#[command(
about = "ExFlow (Extended) Flow Service, It's core module web service."
)]
#[command(propagate_version = true)]
#[command(
help_template = "{about-section}Version: {version} \n {author} \n\n {usage-heading} {usage} \n {all-args} {tab}"
)]
pub struct ExFlowServiceArgs {
    /// Run with specific port
    #[arg(short,long,default_value="8082")]
    port_number:usize,
    /// Azure application insights connection string
    #[arg(short, long, required = false)]
    apm_connection_string: String,
}
impl ExFlowServiceArgs {
    pub fn run(&self) {
        info!("Run with Web Server mode");
        info!("ExFlow Runtime starting....");
        info!("Registering.. to exFlow service");

        let apm_connection_string = self.apm_connection_string.clone();
        set_global_tracing(&apm_connection_string);


    }
}