use std::fmt::{Display, Formatter};

use derive_more::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone,Error, Serialize, Deserialize)]
pub struct ExFlowError {

}
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInformation {
    pub host_name: String,
    pub host_ip: String,
}

impl SystemInformation {
    pub fn new(host_name: &str, host_ip: &str) -> Self {
        SystemInformation {
            host_name: host_name.to_string(),
            host_ip: host_ip.to_string(),
        }
    }
}