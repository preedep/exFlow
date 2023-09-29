use crate::mod_azure::entities::{
    ADFCreateRunResponse, ADFPipelineParams, ADFPipelineRunResponse, ADFResult, AzureCloudError,
    AZURE_RES_REST_API_URL,
};
use azure_core::auth::TokenCredential;
use azure_identity::DefaultAzureCredential;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::io::SeekFrom::Start;
use std::process::Output;

pub async fn adf_pipelines_get(
    subscription_id: &str,
    resource_group_name: &str,
    factory_name: &str,
    run_id: &str,
) -> ADFResult<ADFPipelineRunResponse> {
    let credential = DefaultAzureCredential::default();
    let response = credential.get_token(AZURE_RES_REST_API_URL).await.unwrap();
    debug!("Access token : {:#?}", response);
    let get_url = ADFPipelineParams::new(
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
