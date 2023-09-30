use actix_web::{App, HttpServer, middleware, web};
use actix_web::middleware::Logger;
use log::{debug, error, info};
use crate::mod_runtime_api::runtime_api::{get_status_pipeline, post_run_pipeline};

mod mod_azure;
mod mod_runtime_api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
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
}
