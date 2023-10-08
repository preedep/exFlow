mod mod_azure;
mod mod_runtime_api;
mod mod_runtime_cli;
mod mod_db;

use std::time::Duration;
use clokwerk::{AsyncScheduler, Job, TimeUnits};
use log::{debug, error, info};
use crate::mod_db::db_exflow::get_employees;


use crate::mod_runtime_api::entities::PipelineRunRequest;
async fn run_adf_job() {

    let request = PipelineRunRequest{
        subscription_id: "2ad6d4fd-dcef-4a30-86c7-becd50d38034".to_string(),
        resource_group_name: "NICK-RG-SEA-001".to_string(),
        factory_name: "DevFactory001".to_string(),
        pipeline_name: "pipeline_parallel_function".to_string(),
        callback_url: None,
    };

   let res = reqwest::Client::new().post("http://localhost:8082/api/v1/run_pipeline")
        .json(&request).send().await;

    match res {
        Ok(r) => {
            debug!("Response {:#?}", r);
        }
        Err(e) => {
            error!("Error {:#?}", e);
        }
    }
}
#[tokio::main]
async fn main(){
    pretty_env_logger::init();
    info!("ExFlow Scheduler Running...");

    get_employees().await;
    /*
    // Create a new scheduler
    let mut scheduler = AsyncScheduler::new();
// Add some tasks to it
    scheduler
        .every(1.minutes())
        .run(|| async {
            run_adf_job().await
        });
    scheduler
        .every(1.minutes())
        .run(|| async {
            info!("Scheduler Running 1");
            run_adf_job().await
        });
    scheduler
        .every(1.minutes())
        .run(|| async {
            info!("Scheduler Running 2");
            run_adf_job().await
        });
    loop {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
     */
}