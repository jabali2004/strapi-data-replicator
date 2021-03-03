/// Path used for storing persistent data
pub const DATA_PATH: &str = ".replicated/";

/// Path used for storing configuration information
pub const CONFIG_FILE: &str = "replicate.json";

/// Databases
pub enum DATABASES {
    MySQL,
    MongoDB,
}
