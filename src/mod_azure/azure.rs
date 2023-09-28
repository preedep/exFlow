use azure_identity::DefaultAzureCredential;
use log::debug;
use serde::{Deserialize, Serialize};

const ADF_API_VERSION: &'static str = "2018-06-01";
const ADF_REST_API_URL: &'static str = "https://management.azure.com/subscriptions";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineCreateRunParams {
    #[serde(rename = "factoryName")]
    pub factory_name: String,
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,

    pub query_params: PipelineCreateRunParamsQueryString,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineCreateRunParamsQueryString {
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

impl PipelineCreateRunParams {
    pub fn new(
        factory_name: String,
        pipeline_name: String,
        resource_group_name: String,
        subscription_id: String,
    ) -> Self {
        PipelineCreateRunParams {
            factory_name,
            pipeline_name,
            resource_group_name,
            subscription_id,
            query_params: PipelineCreateRunParamsQueryString {
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
    pub fn to_query_string(&self) -> String {
        //serde_urlencoded::to_string(self).unwrap_or("".to_string())
        let url = format!("{:0}/{:1}/resourceGroups/{:2}/providers/Microsoft.DataFactory/factories/{:3}/pipelines/{:4}/createRun",
                          ADF_REST_API_URL,
                          self.subscription_id,
                          self.resource_group_name,
                          self.factory_name,
                          self.pipeline_name);

        format!(
            "{}?{}",
            url,
            serde_qs::to_string(&self.query_params).unwrap()
        )
    }
}
pub async fn test() {
    let credential = DefaultAzureCredential::default();
    let mut params = PipelineCreateRunParams::new(
        "test-factory".to_string(),
        "test-pipeline".to_string(),
        "test-rg".to_string(),
        "test-subscription".to_string(),
    )
    .with_is_recovery(true)
    .with_reference_pipeline_run_id("xxxx".to_string())
    .with_start_activity_name("yyyy".to_string())
    .with_start_from_failure(false);

    debug!("{}", params.to_query_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;
    #[test]
    fn test_pipeline_crete_run_query_string() {
        let params = PipelineCreateRunParams::new(
            "factory_name".to_string(),
            "pipeline_name".to_string(),
            "resource_group_name".to_string(),
            "subscription_id".to_string(),
        );
        debug!("{:#?}", params);
    }
}
