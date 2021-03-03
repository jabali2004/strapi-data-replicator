use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub strapi_version: String,
    pub database: DatabaseConfig,
    pub replicated: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub database_type: String,
    pub database_version: String,
    pub database_name: String,
    pub host_information: HostInformation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HostInformation {
    pub address: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub ssl: bool,
}
