use log::{debug, error, info};
use crate::mod_azure::azure::adf_pipelines_get;

mod mod_azure;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let subscription_id = "2ad6d4fd-dcef-4a30-86c7-becd50d38034";
    let resource_group_name = "NICK-RG-SEA-001";
    let factory_name = "DevFactory001";
    let pipeline_name = "pipeline_parallel_function";

    info!("Test");
    let res = mod_azure::azure::adf_pipelines_run(
        subscription_id,
        resource_group_name,
        factory_name,
        pipeline_name,
    )
    .await;
    match res {
        Ok(x) => {
            debug!("{:#?}", x);
            let res_status =
                adf_pipelines_get(subscription_id,resource_group_name,factory_name,x.run_id.as_str()).await;

            match res_status {
                Ok(status) => {
                    debug!("{:#?}", status);
                }
                Err(er) => {
                    error!("Get status error : {:#?}",er);
                }
            }
        }
        Err(x) => {
            error!("{:#?}", x);
        }
    }
}
