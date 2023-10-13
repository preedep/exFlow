use actix_web::{HttpResponse, Responder, web};
use log::{debug, error};
use sqlx::SqlitePool;
use tracing_attributes::instrument;

use crate::mod_db::entities::TblExFlowRuntimeClients;
use crate::mod_utils::web_data::ExFlowRuntimeRegisterRequest;

#[instrument]
pub async fn post_register_runtime(
    pool: web::Data<SqlitePool>,
    request: web::Json<ExFlowRuntimeRegisterRequest>,
) -> impl Responder {
    debug!("post_register_runtime : {:#?}", request);
    let tbl = TblExFlowRuntimeClients::from(request.0);
    debug!("TBL register request: {:#?}", tbl);


    let res = sqlx::query(
        r#"insert into tbl_exflow_runtime_clients(client_id,host_name,host_ip,register_id) values(?,?,?,?)"#
    ).bind(tbl.client_id)
        .bind(tbl.host_name)
        .bind(tbl.host_ip)
        .bind(tbl.register_id)
        .execute(pool.get_ref()).await;

    match res{
        Ok(r) => {
            debug!("{:#?}",r);
        }
        Err(e) => {
            error!("{:#?}",e);
        }
    }
    HttpResponse::Ok().finish()
}
