use actix_web::web;
use log::debug;
use sqlx::{MySqlPool};
use tracing_attributes::instrument;

use crate::mod_cores::errors::ExFlowError;
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
    let res = sqlx::query(
        r#"insert into
                tbl_exflow_runtime_clients(client_id,host_name,host_ip,updated_dt)
                values(?,?,?,datetime('now', 'localtime'))
                    on CONFLICT(client_id)
                        do update set host_name=? ,
                        host_ip=? ,
                        updated_dt = datetime('now', 'localtime')
                        where client_id = ?
             "#
    ).bind(&tbl.client_id)
        .bind(&tbl.host_name)
        .bind(&tbl.host_ip)
        .bind(&tbl.host_name)
        .bind(&tbl.host_ip)
        .execute(pool.get_ref()).await;
    res.map(|r|{
        debug!("post_register_runtime : {:#?}",r);
        ExFlowRuntimeRegisterResponse{
            row_effected : r.rows_affected()
        }
    }).map_err(|e|{
        let e = e.as_database_error().map(|err|{
            err.message()
        });
        let mut msg = String::new();
        msg.push_str(e.unwrap_or(""));
        ExFlowError::new_string(msg)
    })
}
