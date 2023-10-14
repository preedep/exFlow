use actix_web::web;
use log::{debug};
use sqlx::MySqlPool;
use tracing_attributes::instrument;
use crate::mod_db::db_exflow::register_exflow_runtime;


use crate::mod_cores::utils::ExFlowResult;
use crate::mod_cores::web_data::{ExFlowRuntimeRegisterRequest, ExFlowRuntimeRegisterResponse};
use crate::mod_db::entities::TblExFlowRuntimeClients;

#[instrument]
pub async fn post_register_runtime(
    pool: web::Data<MySqlPool>,
    request: web::Json<ExFlowRuntimeRegisterRequest>,
) -> ExFlowResult<ExFlowRuntimeRegisterResponse> {
    debug!("post_register_runtime : {:#?}", request);
    let tbl = TblExFlowRuntimeClients::from(request.0);
    debug!("TBL register request: {:#?}", tbl);
    register_exflow_runtime(pool, &tbl).await
}
