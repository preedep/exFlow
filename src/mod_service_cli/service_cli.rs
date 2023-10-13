use actix_web::{App, HttpServer, middleware, web};
use actix_web::middleware::Logger;
use actix_web_opentelemetry::RequestTracing;
use clap::Parser;
use log::info;

use crate::mod_ex_flow_utils::uri_endpoints::{EX_FLOW_SERVICE_API_IR_REGISTER, EX_FLOW_SERVICE_API_SCOPE};
use crate::mod_ex_flow_utils::utils::set_global_apm_tracing;
use crate::mod_service_api::service_api::post_register_runtime;

const SERVICE_NAME: &'static str = "ExFlow-Service";

/// Simple program to greet a person
#[derive(Parser)]
#[command(bin_name = "exflow_service")]
#[command(name = "exFlow Service")]
#[command(author = "Preedee Ponchevin <preedee.digital@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "ExFlow (Extended) Flow Service, It's core module web service.")]
#[command(propagate_version = true)]
#[command(
    help_template = "{about-section}Version: {version} \n {author} \n\n {usage-heading} {usage} \n {all-args} {tab}"
)]
pub struct ExFlowServiceArgs {
    /// Run with specific port
    #[arg(short, long, default_value = "8081")]
    port_number: u16,
    /// Azure application insights connection string
    #[arg(short, long, required = false)]
    apm_connection_string: String,
}

impl ExFlowServiceArgs {
    pub async fn run(&self) -> std::io::Result<()> {
        info!("Run with Web Server mode");
        info!("ExFlow Runtime starting....");
        info!("Registering.. to exFlow service");

        let apm_connection_string = self.apm_connection_string.clone();
        set_global_apm_tracing(apm_connection_string.as_str(), SERVICE_NAME);

        HttpServer::new(|| {
            App::new()
                .wrap(middleware::DefaultHeaders::new().add(("ExFlow-Runtime-X-Version", "0.1")))
                .wrap(Logger::default())
                .wrap(Logger::new(
                    r#"%a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#,
                ))
                .wrap(RequestTracing::new())
                .service(web::scope(EX_FLOW_SERVICE_API_SCOPE).route(
                    EX_FLOW_SERVICE_API_IR_REGISTER,
                    web::post().to(post_register_runtime),
                ))
        })
        .workers(10)
        .bind(("0.0.0.0", self.port_number))?
        .run()
        .await
    }
}
