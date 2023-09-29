use log::{debug, error, info};
mod mod_azure;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    info!("Test");
    let res = mod_azure::azure::adf_pipelines_run("2ad6d4fd-dcef-4a30-86c7-becd50d38034",
                                        "NICK-RG-SEA-001",
                                        "DevFactory001",
                                        "pipeline_parallel_function1").await;
    match res {
        Ok(x) => {
            debug!("{:#?}", x);
        }
        Err(x) => {
            error!("{:#?}", x);
        }
    }
}
