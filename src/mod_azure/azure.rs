use azure_core::auth::TokenCredential;
use azure_identity::DefaultAzureCredential;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::io::SeekFrom::Start;
use std::process::Output;

const ADF_API_VERSION: &'static str = "2018-06-01";
const AZURE_RES_REST_API_URL: &'static str = "https://management.azure.com";

const ADF_REST_API_RUN_CREATE_URI: &'static str = "createRun";

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
    pub status: Option<String>,
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
    fn new(code: &str, message: &str) -> Self {
        AzureCloudError {
            error_cloud: Some(ADFCloudError::new(code, message)),
        }
    }
}
type ADFResult<T> = Result<T, AzureCloudError>;

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
    fn to_run_create_url(&self) -> String {
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
    fn to_get_status_url(&self) -> String {
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
pub async fn adf_pipelines_get(
    subscription_id: &str,
    resource_group_name: &str,
    factory_name: &str,
    run_id: &str,
) -> ADFResult<ADFPipelineRunResponse> {
    let credential = DefaultAzureCredential::default();
    let response = credential.get_token(AZURE_RES_REST_API_URL).await.unwrap();
    debug!("Access token : {:#?}", response);
    let  get_url= ADFPipelineParams::new(
        subscription_id.to_string(),
        resource_group_name.to_string(),
        factory_name.to_string(),
    )
    .with_run_id(run_id.to_string());
    let response = reqwest::Client::new()
        .get(get_url.to_get_status_url())
        .header(
            "Authorization",
            format!("Bearer {}", response.token.secret()),
        )
        .send()
        .await;

    return match response {
        Ok(r) => {
            if r.status() == http::StatusCode::OK {
                debug!("ADF Create Run Success");
                Ok(r.json::<ADFPipelineRunResponse>().await.unwrap())
            } else {
                error!("ADF Create Run Failed");
                Err(r.json::<AzureCloudError>().await.unwrap())
            }
        }
        Err(e) => {
            error!("ADF Create Run Failed : > {}", e);
            let err = AzureCloudError::new(
                e.status()
                    .unwrap_or(http::StatusCode::INTERNAL_SERVER_ERROR)
                    .as_str(),
                e.to_string().as_str(),
            );
            Err(err)
        }
    };
}
pub async fn adf_pipelines_run(
    subscription_id: &str,
    resource_group_name: &str,
    factory_name: &str,
    pipeline_name: &str,
) -> ADFResult<ADFCreateRunResponse> {
    let credential = DefaultAzureCredential::default();
    let response = credential.get_token(AZURE_RES_REST_API_URL).await.unwrap();
    debug!("Access token : {:#?}", response);
    let create_run = ADFPipelineParams::new(
        subscription_id.to_string(),
        resource_group_name.to_string(),
        factory_name.to_string(),
    )
    .with_pipeline_name(pipeline_name.to_string());
    let response = reqwest::Client::new()
        .post(create_run.to_run_create_url())
        .header(
            "Authorization",
            format!("Bearer {}", response.token.secret()),
        )
        .header("content-length", 0)
        .send()
        .await;

    return match response {
        Ok(r) => {
            if r.status() == http::StatusCode::OK {
                debug!("ADF Create Run Success");
                Ok(r.json::<ADFCreateRunResponse>().await.unwrap())
            } else {
                error!("ADF Create Run Failed");
                Err(r.json::<AzureCloudError>().await.unwrap())
            }
        }
        Err(e) => {
            error!("ADF Create Run Failed : > {}", e);
            let err = AzureCloudError::new(
                e.status()
                    .unwrap_or(http::StatusCode::INTERNAL_SERVER_ERROR)
                    .as_str(),
                e.to_string().as_str(),
            );
            Err(err)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;
    #[test]
    fn test_pipeline_crete_run_query_string() {
        let params = ADFPipelineParams::new(
            "factory_name".to_string(),
            "pipeline_name".to_string(),
            "resource_group_name".to_string(),
        );
        debug!("{:#?}", params);
    }
}
