use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Error, Serialize, Deserialize)]
pub struct ExFlowRuntimeRegisterRequest {}