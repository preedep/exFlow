use actix_web::web;
use clokwerk::{Job, TimeUnits};
use log::{debug, error, info};
use sqlx::{Error, MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;
use crate::mod_cores::errors::ExFlowError;
use crate::mod_db::db_exflow_runtime::get_register_runtime_list;
use crate::mod_db::entities::TblExFlowRuntimeClients;

use crate::mod_runtime::entities::PipelineRunRequest;

mod mod_azure;
mod mod_cores;

mod mod_runtime;

mod mod_service;

mod mod_db;

async fn run_adf_job() {
    let request = PipelineRunRequest {
        subscription_id: "2ad6d4fd-dcef-4a30-86c7-becd50d38034".to_string(),
        resource_group_name: "NICK-RG-SEA-001".to_string(),
        factory_name: "DevFactory001".to_string(),
        pipeline_name: "pipeline_parallel_function".to_string(),
        callback_url: None,
    };

    let res = reqwest::Client::new()
        .post("http://localhost:8082/api/v1/run_pipeline")
        .json(&request)
        .send()
        .await;

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
async fn main() {
    pretty_env_logger::init();
    info!("ExFlow Scheduler Running...");
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect("mssql://exflow_user:P@ssw0rd@localhost:3306/exFlowDb")
        .await;
    match pool {
        Ok(p) => {
            let r =
                get_register_runtime_list(web::Data::new(p.clone())).await;
            match r {
                Ok(l) => {
                    for item in l.iter() {
                        debug!("{:#?}" ,item);
                    }
                }
                Err(e) => {
                    error!("Error {:#?}", e);
                }
            }
        }
        Err(e) => {
            error!("Error {:?}", e);
        }
    }
}
