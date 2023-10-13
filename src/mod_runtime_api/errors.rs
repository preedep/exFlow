use std::fmt::{Display, Formatter};

use actix_web::{error, HttpResponse};
use derive_more::Error;
use serde::{Deserialize, Serialize};

use crate::mod_azure::entities::AzureCloudError;

#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub struct ExFlowRuntimeWebError {
    #[serde(rename = "error_message")]
    pub error_message: String,
    #[serde(rename = "adf_error")]
    pub adf_error: Option<AzureCloudError>,
}

impl ExFlowRuntimeWebError {
    pub fn new(error_message: String) -> Self {
        ExFlowRuntimeWebError { error_message, adf_error: None }
    }
    pub fn add_adf_error(&mut self,adf_error: &AzureCloudError) -> Self {
        self.adf_error = Some(adf_error.clone());
        self.clone()
    }
}
impl Display for ExFlowRuntimeWebError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
impl error::ResponseError for ExFlowRuntimeWebError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(&self)
    }
}

