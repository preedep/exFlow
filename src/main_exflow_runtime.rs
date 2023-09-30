use crate::mod_runtime_api::runtime_api::{get_status_pipeline, post_run_pipeline};
use actix_web::middleware::Logger;
use actix_web::{middleware, web, App, HttpServer};

use log::{debug, error, info};

mod mod_azure;
mod mod_runtime_api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    clap::Command::new("exflow_runtime")
        .bin_name("exflow_runtime")
        .version("0.1.0")
        .author("Preedee Ponchevin <preedee.digital@gmail.com>")
        .about("Exflow-runtime is runtime for integrate with Web Exflow Service")
        .arg(
            clap::Arg::new("mode")
                .value_name("mode")
                .help("mode = cli or runtime")
                .required(false)
                .default_value("runtime")
        )
        .get_matches();

    /*
    debug!("ExFlow Runtime starting....");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.1")))
            .service(web::scope("/api/v1")
                .route("/run_pipeline",web::post().to(post_run_pipeline))
                .route("/get_status",web::get().to(get_status_pipeline))
        )
    }).workers(10)
        .bind(("0.0.0.0",8082))?.run().await
     */
    Ok(())
}
