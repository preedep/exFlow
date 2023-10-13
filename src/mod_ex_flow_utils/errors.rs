use std::fmt::{Display,Formatter};
use derive_more::Error;
use serde::{Deserialize, Serialize};

pub const GENERAL_FUNCTION_NOT_SUPPORTED: &'static str = "Not Supported Yet";
pub const GENERAL_PARAM_NOT_COMPLETE: &'static str = "Params Not Complete";

pub const RUNTIME_ERROR: &'static str = "ExFlow Runtime Error";



#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub struct ExFlowError {}
impl ExFlowError {
    pub fn new(msg: &'static str) -> Self {
        ExFlowError {}
    }
}
impl Display for ExFlowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExFlowError")
    }
}

