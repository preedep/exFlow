use log::info;
mod mod_azure;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    info!("Test");
    mod_azure::azure::adf_pipelines_run("DevFactory001", "pipeline_parallel_function", "NICK-RG-SEA-001", "2ad6d4fd-dcef-4a30-86c7-becd50d38034").await;
}
