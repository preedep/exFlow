use actix_web::web::Data;
use log::error;
use sqlx::MySqlPool;

use crate::mod_cores::errors::ExFlowError;
use crate::mod_db::entities::TblExFlowJobs;

pub async fn get_job_list(pool: Data<MySqlPool>) -> Result<Vec<TblExFlowJobs>, ExFlowError> {
    let res = sqlx::query_as::<_, TblExFlowJobs>(
        r#"
        select * from tbl_exflow_jobs
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
