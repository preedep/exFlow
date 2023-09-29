use log::debug;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

const ADF_API_VERSION: &'static str = "2018-06-01";
pub const AZURE_RES_REST_API_URL: &'static str = "https://management.azure.com";

const ADF_REST_API_RUN_CREATE_URI: &'static str = "createRun";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ADFPipelineRunStatus {
    #[serde(rename = "Queued")]
    Queued,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Succeeded")]
    Succeeded,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Canceling")]
    Canceling,
    #[serde(rename = "Cancelled")]
    Cancelled
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ADFPipelineParams {
    #[serde(rename = "factoryName")]
    pub factory_name: String,
    #[serde(rename = "pipelineName")]
    pub pipeline_name: Option<String>,
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
    #[serde(rename = "runId")]
    pub run_id: Option<String>,

    pub query_params: ADFPipelineParamsQueryString,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ADFPipelineParamsQueryString {
    #[serde(rename = "api-version")]
    pub api_version: String,
    #[serde(rename = "isRecovery")]
    is_recovery: Option<bool>,
    #[serde(rename = "referencePipelineRunId")]
    reference_pipeline_run_id: Option<String>,
    #[serde(rename = "startActivityName")]
    start_activity_name: Option<String>,
    #[serde(rename = "startFromFailure")]
    start_from_failure: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ADFCreateRunResponse {
    #[serde(rename = "runId")]
    pub run_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ADFPipelineRunResponse {
    #[serde(rename = "runId")]
    pub run_id: Option<String>,
    #[serde(rename = "pipelineName")]
    pub pipeline_name: Option<String>,
    #[serde(rename = "parameters")]
    pub parameters: Option<serde_json::Value>,
    #[serde(rename = "invokedBy")]
    pub invoked_by: Option<ADFPipelineRuneInvokedBy>,
    #[serde(rename = "runStart")]
    pub run_start: Option<String>,
    #[serde(rename = "runEnd")]
    pub run_end: Option<String>,
    #[serde(rename = "durationInMs")]
    pub duration_in_ms: Option<i64>,
    #[serde(rename = "status")]
    pub status: Option<ADFPipelineRunStatus>,
    #[serde(rename = "message")]
    pub message: Option<String>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ADFPipelineRuneInvokedBy {
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "invokedByType")]
    pub invoked_by_type : Option<String>,
    #[serde(rename = "pipelineName")]
    pub pipeline_name : Option<String>,
    #[serde(rename = "pipelineRunId")]
    pub pipeline_run_id : Option<String>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ADFCloudError {
    #[serde(rename = "code")]
    pub error_code: Option<String>,
    #[serde(rename = "details")]
    pub error_details: Option<Vec<ADFCloudError>>,
    #[serde(rename = "message")]
    pub error_message: Option<String>,
    #[serde(rename = "target")]
    pub error_target: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCloudError {
    #[serde(rename = "error")]
    error_cloud: Option<ADFCloudError>,
}
impl Display for ADFCloudError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
impl ADFCloudError {
    fn new(code: &str, message: &str) -> Self {
        ADFCloudError {
            error_code: Some(code.to_string()),
            error_details: None,
            error_message: Some(message.to_string()),
            error_target: None,
        }
    }
}
impl AzureCloudError {
    pub(crate) fn new(code: &str, message: &str) -> Self {
        AzureCloudError {
            error_cloud: Some(ADFCloudError::new(code, message)),
        }
    }
}
pub type ADFResult<T> = Result<T, AzureCloudError>;

impl ADFPipelineParams {
    pub fn new(subscription_id: String, resource_group_name: String, factory_name: String) -> Self {
        ADFPipelineParams {
            factory_name,
            pipeline_name: None,
            resource_group_name,
            subscription_id,
            run_id: None,
            query_params: ADFPipelineParamsQueryString {
                api_version: ADF_API_VERSION.to_string(),
                is_recovery: None,
                reference_pipeline_run_id: None,
                start_activity_name: None,
                start_from_failure: None,
            },
        }
    }
    pub fn with_pipeline_name(&mut self, pipeline_name: String) -> Self {
        self.pipeline_name = Some(pipeline_name);
        self.clone()
    }
    pub fn with_is_recovery(&mut self, is_recovery: bool) -> Self {
        self.query_params.is_recovery = Some(is_recovery);
        self.clone()
    }
    pub fn with_reference_pipeline_run_id(&mut self, reference_pipeline_run_id: String) -> Self {
        self.query_params.reference_pipeline_run_id = Some(reference_pipeline_run_id);
        self.clone()
    }
    pub fn with_start_activity_name(&mut self, start_activity_name: String) -> Self {
        self.query_params.start_activity_name = Some(start_activity_name);
        self.clone()
    }
    pub fn with_start_from_failure(&mut self, start_from_failure: bool) -> Self {
        self.query_params.start_from_failure = Some(start_from_failure);
        self.clone()
    }
    pub fn with_run_id(&mut self, run_id: String) -> Self {
        self.run_id = Some(run_id);
        self.clone()
    }
    ///
    /// Base URL
    ///
    fn to_url(&self) -> String {
        format!("{:0}/subscriptions/{:1}/resourceGroups/{:2}/providers/Microsoft.DataFactory/factories/{:3}",
                AZURE_RES_REST_API_URL,
                self.subscription_id,
                self.resource_group_name,
                self.factory_name)
    }
    ///
    /// URL for Pipeline RunCreate
    ///
    pub(crate) fn to_run_create_url(&self) -> String {
        let url = format!(
            "{:0}/pipelines/{:1}/{:2}",
            self.to_url(),
            self.pipeline_name.to_owned().unwrap_or("".to_string()),
            ADF_REST_API_RUN_CREATE_URI
        );
        let url = format!(
            "{:0}?{:1}",
            url,
            serde_qs::to_string(&self.query_params).unwrap()
        );
        debug!("to_run_create_url = {:?}", url);
        url
    }
    ///
    /// URL for Get Pipeline Status
    ///
    pub(crate) fn to_get_status_url(&self) -> String {
        ///pipelineruns/{runId}?api-version=2018-06-01
        let url = format!(
            "{:0}/pipelineruns/{:1}",
            self.to_url(),
            self.to_owned().run_id.unwrap_or("".to_string())
        );
        let url = format!(
            "{:0}?{:1}",
            url,
            serde_qs::to_string(&self.query_params).unwrap()
        );
        debug!("to_get_status_url = {:?}", url);
        url
    }
}
