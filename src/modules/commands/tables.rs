use crate::modules::services::databases;
use crate::modules::services::utils::{get_config_using_env, read_config_file};
use crate::modules::types::cli::Cli;
use crate::modules::types::config::Config;

/// Print all tables
pub fn run_tables(args: Cli) -> () {
    let config: Config;

    if args.use_env == true {
        config = get_config_using_env();
    } else {
        config = read_config_file();
    }

    let tables: Vec<String> = get_tables(config);

    println!("Tables / Collections:");
    for table in tables {
        println!("{}", table);
    }
}

/// Return vec with table names
fn get_tables(config: Config) -> Vec<String> {
    let raw_tables = databases::list(config.clone());
    if raw_tables.is_some() {
        let tables = raw_tables.unwrap();
        return tables;
    }

    return vec![];
}
