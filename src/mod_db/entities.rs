use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::mod_cores::web_data::ExFlowRuntimeRegisterRequest;

///
/// TblExFlowRuntimeClients is struct representing tbl_exflow_runtime_clients
/// Collect data when ExFlowRuntime is initialized and Register to ExFlow Service
///
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TblExFlowRuntimeClients {
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "host_name")]
    pub host_name: Option<String>,
    #[serde(rename = "host_ip")]
    pub host_ip: String,
    #[serde(rename = "created_dt")]
    pub created_dt: Option<chrono::NaiveDateTime>,
    #[serde(rename = "updated_dt")]
    pub updated_dt: Option<chrono::NaiveDateTime>,
}

impl From<ExFlowRuntimeRegisterRequest> for TblExFlowRuntimeClients {
    fn from(value: ExFlowRuntimeRegisterRequest) -> Self {
        TblExFlowRuntimeClients {
            client_id: value.client_id,
            host_name: Some(value.host_name.unwrap_or("".to_string())),
            host_ip: value.host_ip,
            created_dt: None,
            updated_dt: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TblExFlowApps {}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TblExFlowJobs {
    #[serde(rename = "app_id")]
    pub app_id: String,
    #[serde(rename = "job_id")]
    pub job_id: String,
    #[serde(rename = "cron_exp_seconds")]
    pub cron_exp_seconds: String,
    #[serde(rename = "cron_exp_minutes")]
    pub cron_exp_minutes: String,
    #[serde(rename = "cron_exp_hours")]
    pub cron_exp_hours: String,
    #[serde(rename = "cron_exp_day_of_month")]
    pub cron_exp_day_of_month: String,
    #[serde(rename = "cron_exp_month")]
    pub cron_exp_month: String,
    #[serde(rename = "cron_exp_day_of_week")]
    pub cron_exp_day_of_week: String,
    #[serde(rename = "cron_exp_year")]
    pub cron_exp_year: Option<String>,
    #[serde(rename = "created_dt")]
    pub created_dt: Option<chrono::NaiveDateTime>,
    #[serde(rename = "updated_dt")]
    pub updated_dt: Option<chrono::NaiveDateTime>,
    #[serde(rename = "updated_by")]
    pub updated_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TblExFlowClientIDs {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TblExFlowJobHistory {}
