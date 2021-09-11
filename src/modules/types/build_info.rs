use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct BuildInfo {
    pub version: String,
    pub gitVersion: String,
}
