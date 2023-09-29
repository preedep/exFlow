use std::thread::sleep;
use std::time::Duration;
use crate::mod_azure::azure::{adf_pipelines_get, adf_pipelines_run};
use log::{debug, error, info};
use crate::mod_azure::entities::ADFPipelineRunStatus;

mod mod_azure;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let subscription_id = "2ad6d4fd-dcef-4a30-86c7-becd50d38034";
    let resource_group_name = "NICK-RG-SEA-001";
    let factory_name = "DevFactory001";
    let pipeline_name = "pipeline_parallel_function";

    let res = adf_pipelines_run(
        subscription_id,
        resource_group_name,
        factory_name,
        pipeline_name,
    )
    .await;
    match res {
        Ok(x) => {
            debug!("{:#?}", x);
            loop {
                sleep(Duration::from_secs(10));
                let res_status = adf_pipelines_get(
                    subscription_id,
                    resource_group_name,
                    factory_name,
                    x.run_id.as_str(),
                ).await;
                match res_status {
                    Ok(status) => {
                        match status.status.clone().unwrap() {
                            ADFPipelineRunStatus::Queued |
                            ADFPipelineRunStatus::InProgress => {
                                info!("Status : {:#?}", status.status.unwrap());
                            }
                            ADFPipelineRunStatus::Succeeded |
                            ADFPipelineRunStatus::Failed |
                            ADFPipelineRunStatus::Canceling |
                            ADFPipelineRunStatus::Cancelled => {
                                info!("Status : {:#?}", status.status.unwrap());
                                break
                            }
                        }
                    }
                    Err(er) => {
                        error!("Get status error : {:#?}", er);
                        break
                    }
                }
            }

        }
        Err(x) => {
            error!("{:#?}", x);
        }
    }
}
