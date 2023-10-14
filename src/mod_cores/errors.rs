use std::fmt::{Display, Formatter};

use actix_web::body::BoxBody;
use actix_web::{error, HttpResponse};
use derive_more::Error;
use serde::{Deserialize, Serialize};

use crate::mod_azure::entities::AzureCloudError;

pub const GENERAL_FUNCTION_NOT_SUPPORTED: &'static str = "Not Supported Yet";
pub const GENERAL_PARAM_NOT_COMPLETE: &'static str = "Params Not Complete";

pub const RUNTIME_ERROR: &'static str = "ExFlow Runtime Error";

#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub struct ExFlowError {
    #[serde(rename = "error_message")]
    pub error_message: String,
    #[serde(rename = "runtime_adf_error")]
    pub runtime_adf_error: Option<AzureCloudError>,
}

impl ExFlowError {
    pub fn new(msg: &'static str) -> Self {
        ExFlowError {
            error_message: "".to_string(),
            runtime_adf_error: None,
        }
    }
    pub fn new_string(msg: String) -> Self {
        ExFlowError {
            error_message: msg,
            runtime_adf_error: None,
        }
    }
    pub fn new_with_runtime() -> Self {
        ExFlowError {
            error_message: RUNTIME_ERROR.to_string(),
            runtime_adf_error: None,
        }
    }

    pub fn add_adf_error(&mut self, adf_error: &AzureCloudError) -> Self {
        self.runtime_adf_error = Some(adf_error.clone());
        self.clone()
    }
}

impl Display for ExFlowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExFlowError : {:#?}", self)
    }
}

impl error::ResponseError for ExFlowError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::InternalServerError().json(&self)
    }
}
