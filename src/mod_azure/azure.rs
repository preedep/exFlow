use std::fmt::Display;
use std::future::Future;

use actix_web::body::MessageBody;
use azure_core::auth::{TokenCredential, TokenResponse};
use azure_identity::DefaultAzureCredential;
use chrono::Utc;
use log::{debug, error};

use crate::mod_azure::entities::{
    ADFCreateRunResponse, ADFPipelineParams, ADFPipelineRunResponse, ADFResult,
    AZURE_SPN_URL, AzureAccessTokenResult, AzureCloudError,
};

pub async fn get_azure_access_token_from(
    access_token: Option<TokenResponse>,
    spn_url: Option<String>,
) -> AzureAccessTokenResult<TokenResponse> {
    match access_token {
        None => {
            let credential = DefaultAzureCredential::default();
            let response = credential
                .get_token(spn_url.unwrap_or(AZURE_SPN_URL.to_string()).as_str())
                .await;
            response
                .map_err(|e| AzureCloudError { error_cloud: None })
                .map(|r| r)
        }
        Some(a) => {
            let diff = a.clone().expires_on.unix_timestamp() - Utc::now().timestamp();
            if diff <= 0 {
                // get new access token
                debug!("Request New Access token");
                let credential = DefaultAzureCredential::default();
                let response = credential
                    .get_token(spn_url.unwrap_or(AZURE_SPN_URL.to_string()).as_str())
                    .await;
                response
                    .map_err(|e| AzureCloudError { error_cloud: None })
                    .map(|r| r)
            } else {
                // use existing access token
                debug!("Use existing access token");
                Ok(a.clone())
            }
        }
    }
}

pub async fn adf_pipelines_get(
    token_response: &TokenResponse,
    subscription_id: &str,
    resource_group_name: &str,
    factory_name: &str,
    run_id: &str,
) -> ADFResult<ADFPipelineRunResponse> {
    //debug!("Access token : {:#?}", response);
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
            format!("Bearer {}", token_response.token.secret()),
        )
        .send()
        .await;

    return match response {
        Ok(r) => {
            if r.status() == http::StatusCode::OK {
                //debug!("ADF Create Run Success  : {:#?}", r);
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
    token_response: &TokenResponse,
    subscription_id: &str,
    resource_group_name: &str,
    factory_name: &str,
    pipeline_name: &str,
) -> ADFResult<ADFCreateRunResponse> {
    //let credential = DefaultAzureCredential::default();
    //let response = credential.get_token(AZURE_RES_REST_API_URL).await.unwrap();
    //debug!("Access token : {:#?}", response);
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
            format!("Bearer {}", token_response.token.secret()),
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
    use log::debug;

    use super::*;

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
