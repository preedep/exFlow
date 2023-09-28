use azure_identity::DefaultAzureCredential;
use serde::{Deserialize, Serialize};

const VERSION: &'static str = "2018-06-01";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineCreateRunParams {
    #[serde(rename = "factoryName")]
    pub factory_name: String,
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
    #[serde(rename = "subscriptionId")]
    pub subscription_id:String,
    #[serde(rename = "api-version")]
    pub api_version:String,
    #[serde(rename = "isRecovery")]
    pub is_recovery : Option<bool>,
    #[serde(rename = "referencePipelineRunId")]
    pub reference_pipeline_run_id : Option<String>,
    #[serde(rename = "startActivityName")]
    pub start_activity_name: Option<String>,
    #[serde(rename = "startFromFailure")]
    pub start_from_failure: Option<bool>
}

impl PipelineCreateRunParams {
    pub fn new(
        factory_name: String,
        pipeline_name: String,
        resource_group_name: String,
        subscription_id:String) -> Self{

        PipelineCreateRunParams {
            factory_name,
            pipeline_name,
            resource_group_name,
            subscription_id,
            api_version: "".to_string(),
            is_recovery: None,
            reference_pipeline_run_id: None,
            start_activity_name: None,
            start_from_failure: None,
        }
    }
    pub fn to_query_string(&self) -> String {
        serde_qs::to_string(self).unwrap_or("".to_string())
    }
}
pub async fn test() {
    let credential = DefaultAzureCredential::default();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pipeline_crete_run_query_string() {

    }
}