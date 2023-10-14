use actix_web::web::Data;
use log::{debug, error};
use sqlx::MySqlPool;

use crate::mod_cores::errors::ExFlowError;
use crate::mod_db::entities::TblExFlowRuntimeClients;

pub async fn register_exflow_runtime(
    pool: Data<MySqlPool>,
    tbl: &TblExFlowRuntimeClients,
) -> Result<u64, ExFlowError> {
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
        r.rows_affected()
    })
    .map_err(|e| {
        error!("Insert failed with error {:?}", e);
        let e = e.as_database_error().map(|err| err.message());
        let mut msg = String::new();
        msg.push_str(e.unwrap_or(""));
        ExFlowError::new_string(msg)
    })
}
pub async fn get_register_runtime_list(
    pool: Data<MySqlPool>,
) -> Result<Vec<TblExFlowRuntimeClients>, ExFlowError> {
    let res = sqlx::query_as::<_, TblExFlowRuntimeClients>(
        r#"
        select * from tbl_exflow_runtime_clients
    "#,
    )
    .fetch_all(pool.as_ref())
    .await;
    res.map_err(|e| {
        error!("Get List of runtime clients failed with error {:?}", e);
        let e = e.as_database_error().map(|err| err.message());
        let mut msg = String::new();
        msg.push_str(e.unwrap_or(""));
        ExFlowError::new_string(msg)
    })
}
