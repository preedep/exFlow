use std::fmt::{Display, Formatter};
use std::future::Future;
use std::io::SeekFrom::Start;
use std::process::Output;
use azure_core::auth::TokenCredential;
use azure_identity::DefaultAzureCredential;
use log::{debug, error};
use serde::{Deserialize, Serialize};

const ADF_API_VERSION: &'static str = "2018-06-01";
const AZURE_RES_REST_API_URL: &'static str = "https://management.azure.com";

const ADF_REST_API_RUN_CREATE_URI: &'static str = "/createRun";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ADFPipelineParams {
    #[serde(rename = "factoryName")]
    pub factory_name: String,
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ADFCloudError {
    #[serde(rename = "code")]
    pub error_code: Option<String>,
    #[serde(rename = "details")]
    pub error_details : Option<Vec<ADFCloudError>>,
    #[serde(rename = "message")]
    pub error_message : Option<String>,
    #[serde(rename = "target")]
    pub error_target : Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCloudError {
    #[serde(rename = "error")]
    error_cloud : Option<ADFCloudError>
}
impl Display for ADFCloudError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}",self)
    }
}
impl ADFCloudError {
    fn new(code: &str,message: &str) -> Self {
        ADFCloudError{
            error_code: Some(code.to_string()),
            error_details: None,
            error_message: Some(message.to_string()),
            error_target: None,
        }
    }
}
impl AzureCloudError {
    fn new(code: &str,message: &str) -> Self {
        AzureCloudError{
            error_cloud: Some(ADFCloudError::new(code,message)),
        }
    }
}
type ADFResult<T> = Result<T,AzureCloudError>;

impl ADFPipelineParams {
    pub fn new(
        factory_name: String,
        pipeline_name: String,
        resource_group_name: String,
        subscription_id: String,
    ) -> Self {
        ADFPipelineParams {
            factory_name,
            pipeline_name,
            resource_group_name,
            subscription_id,
            query_params: ADFPipelineParamsQueryString {
                api_version: ADF_API_VERSION.to_string(),
                is_recovery: None,
                reference_pipeline_run_id: None,
                start_activity_name: None,
                start_from_failure: None,
            },
        }
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
    fn to_url(&self) -> String {
        format!("{:0}/subscriptions/{:1}/resourceGroups/{:2}/providers/Microsoft.DataFactory/factories/{:3}/pipelines/{:4}",
                AZURE_RES_REST_API_URL,
                          self.subscription_id,
                          self.resource_group_name,
                          self.factory_name,
                          self.pipeline_name)
    }
    fn to_run_create_url(&self) -> String {
        let url = format!("{:0}{:1}", self.to_url(), ADF_REST_API_RUN_CREATE_URI);
        format!(
            "{:0}?{:1}",
            url,
            serde_qs::to_string(&self.query_params).unwrap()
        )
    }
    fn to_get_status_url(&self) -> String {
        format!(
            "{:0}?{:1}",
            self.to_url(),
            serde_qs::to_string(&self.query_params).unwrap()
        )
    }
}
pub async fn adf_pipelines_run(
    subscription_id: &str,
    resource_group_name: &str,
    factory_name: &str,
    pipeline_name: &str
) -> ADFResult<ADFCreateRunResponse>
{
    let credential = DefaultAzureCredential::default();
    let response = credential.get_token(AZURE_RES_REST_API_URL).await.unwrap();
    debug!("Access token : {:#?}", response);
    let create_run = ADFPipelineParams::new(
        factory_name.to_string(),
        pipeline_name.to_string(),
        resource_group_name.to_string(),
        subscription_id.to_string(),
    );
    let response = reqwest::Client::new()
        .post(create_run.to_run_create_url())
        .header(
            "Authorization",
            format!("Bearer {}", response.token.secret()),
        ).header("content-length", 0)
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
            error!("ADF Create Run Failed : > {}",e);
            let err = AzureCloudError::new(e.status().unwrap_or(
                http::StatusCode::INTERNAL_SERVER_ERROR).as_str(),
                               e.to_string().as_str());
            Err(err)
        }
    }
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
            "subscription_id".to_string(),
        );
        debug!("{:#?}", params);
    }
}
