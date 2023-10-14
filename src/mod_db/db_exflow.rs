use crate::mod_cores::errors::ExFlowError;
use crate::mod_cores::web_data::ExFlowRuntimeRegisterResponse;
use crate::mod_db::entities::TblExFlowRuntimeClients;
use actix_web::web::Data;
use log::{debug, error};
use sqlx::MySqlPool;

pub async fn register_exflow_runtime(
    pool: Data<MySqlPool>,
    tbl: &TblExFlowRuntimeClients,
) -> Result<ExFlowRuntimeRegisterResponse, ExFlowError> {
    let res = sqlx::query(
        r#"insert into
                tbl_exflow_runtime_clients(client_id,host_name,host_ip,updated_dt)
                values(?,?,?,CURRENT_TIMESTAMP())
                    on DUPLICATE key update host_name=? ,
                        host_ip=? ,
                        updated_dt = CURRENT_TIMESTAMP()
             "#,
    )
    .bind(&tbl.client_id)
    .bind(&tbl.host_name)
    .bind(&tbl.host_ip)
    .bind(&tbl.host_name)
    .bind(&tbl.host_ip)
    .execute(pool.get_ref())
    .await;
    res.map(|r| {
        debug!("post_register_runtime : {:#?}", r);
        ExFlowRuntimeRegisterResponse {
            row_effected: r.rows_affected(),
        }
    })
    .map_err(|e| {
        error!("Insert failed with error {:?}", e);
        let e = e.as_database_error().map(|err| err.message());
        let mut msg = String::new();
        msg.push_str(e.unwrap_or(""));
        ExFlowError::new_string(msg)
    })
}
