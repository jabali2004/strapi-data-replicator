use serde::Serialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Serialize)]
pub struct Cli {
    /// Available commands:
    /// init, replicate, migrate, info, tables
    pub command: String,
    /// Overwrite existing project configuration
    #[structopt(short = "f", long = "force")]
    pub overwrite: bool,
    /// Use environment variables
    #[structopt(short = "e", long = "env")]
    pub use_env: bool,
    // Working path
    #[structopt(default_value = ".")]
    pub path: String,
    // Specify version to migrate
    #[structopt(short = "v", long = "replication-version", default_value = "")]
    pub replication_version: String,
}
