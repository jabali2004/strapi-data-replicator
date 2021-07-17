use std::borrow::Borrow;

use colored::Colorize;
use quicli::fs::read_file;

use crate::consts::DATA_PATH;
use crate::modules::services::databases;
use crate::modules::services::utils::{
    check_path_existence, get_config_using_env, get_latest_replication_version, read_config_file,
    read_replication_file_paths, verify_replication_version,
};
use crate::modules::types::cli::Cli;
use crate::modules::types::config::Config;

pub fn run_migrate(args: Cli) {
    let config: Config;

    if args.use_env == true {
        config = get_config_using_env();
    } else {
        config = read_config_file();
    }

    let replication_version: String;
    let data_path_exists = check_path_existence(DATA_PATH, true);

    if !data_path_exists {
        return;
    }

    if args.replication_version.is_empty() {
        replication_version = get_latest_replication_version();
    } else {
        replication_version = args.replication_version;
        if !verify_replication_version(replication_version.clone()) {
            println!("{}", "Version to migrate does not exist!".red());
            return;
        }
    }

    let paths = read_replication_file_paths(replication_version.clone());

    for path in paths {
        if path.is_ok() {
            let _path = path.unwrap();
            let data = read_file(_path.path());

            if data.is_ok() {
                let mut filename = _path.file_name().to_string_lossy().to_string();
                // remove '.sql' / '.json'
                match config.database.database_type.borrow() {
                    "mongodb" => filename.truncate(filename.len() - 5),
                    "mysql" => filename.truncate(filename.len() - 4),
                    _ => filename.truncate(filename.len() - 4),
                }

                databases::migrate_table(data.unwrap(), filename.clone(), config.clone());

                let migrated_message = format!("{} migrated", &filename);
                println!("{}", migrated_message.green());
            }
        }
    }

    println!("Successfully migrated to version {}!", &replication_version);
}
