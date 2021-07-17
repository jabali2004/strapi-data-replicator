use std::fs::create_dir;

use colored::Colorize;
use quicli::fs::write_to_file;

use crate::consts::DATA_PATH;
use crate::modules::services::databases;
use crate::modules::services::utils::read_replication_file_paths;
use crate::modules::services::utils::{
    create_data_dir, get_config_using_env, get_next_version, read_config_file,
};
use crate::modules::types::cli::Cli;
use crate::modules::types::config::Config;

/// Run replicate command logic
pub fn run_replicate(args: Cli) {
    let config: Config;

    if args.use_env == true {
        config = get_config_using_env();
    } else {
        config = read_config_file();
    }

    create_data_dir();

    let path = DATA_PATH;
    let next_version = get_next_version();

    let file_ending = match config.database.database_type.as_str() {
        "mongodb" => String::from("json"),
        "mysql" => String::from("sql"),
        _ => String::from("sql"),
    };

    create_dir(format!("{}{}", path, next_version)).expect("Could not create directory!");

    let mut error_flag: bool = false;

    for table in config.replicated.clone() {
        let table_data = databases::replicate_table(table.clone(), config.clone());

        if table_data.is_some() {
            write_to_file(
                format!("{}{}/{}.{}", path, next_version, table.clone(), file_ending),
                &table_data.unwrap(),
            )
            .expect("Error while writing to file!");
            let replicated_message = format!("{} replicated", &table);
            println!("{}", replicated_message.green());
        } else {
            // Set error flag
            error_flag = true;

            let error_message = format!("Error while replicating table {}.", &table);
            println!("{}", error_message.red());
        }
    }

    let count = read_replication_file_paths(next_version.clone()).count();

    if error_flag && count <= 0 {
        println!("{}", "Error while replicating database tables!".red());
    } else if error_flag {
        println!("{}", "Partially replicated database tables!".yellow());
        println!("{}", format!("Replication version: {}", next_version));
    } else {
        println!("{}", "Successfully replicated database tables!".green());
        println!("{}", format!("Replication version: {}", next_version));
    }
}
