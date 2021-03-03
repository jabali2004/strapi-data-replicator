use std::fs::read_to_string;
use std::io::{stdin, stdout, Write};

use colored::Colorize;
use quicli::fs::write_to_file;
use regex::Regex;

use crate::consts::CONFIG_FILE;
use crate::modules::commands::replicate::run_replicate;
use crate::modules::services::databases;
use crate::modules::services::utils::{get_config_input, get_config_string};
use crate::modules::types::cli::Cli;
use crate::modules::types::config::{Config, DatabaseConfig, HostInformation};
use crate::modules::types::package_json::PackageJson;

/// Run init command logic
pub fn run_init(args: Cli) {
    let mut new_config: Config;

    new_config = Config {
        strapi_version: "".to_string(),
        database: DatabaseConfig {
            database_type: get_config_string("DATABASE_TYPE"),
            database_version: get_config_string("DATABASE_VERSION"),
            database_name: get_config_string("DATABASE_NAME"),
            host_information: HostInformation {
                address: get_config_string("DATABASE_HOST"),
                port: get_config_string("DATABASE_PORT"),
                username: get_config_string("DATABASE_USERNAME"),
                password: get_config_string("DATABASE_PASSWORD"),
                ssl: match get_config_string("DATABASE_SSL").parse() {
                    Ok(value) => value,
                    Err(_) => false,
                },
            },
        },
        replicated: vec![
            "strapi_role".to_string(),
            "strapi_permission".to_string(),
            "users-permissions_permission".to_string(),
            "users-permissions_role".to_string(),
        ],
    };

    let package_json = read_to_string("package.json");

    if package_json.is_ok() {
        let package_json_obj: PackageJson = serde_json::from_str(&package_json.unwrap().as_str())
            .expect("Unable to parse package.json!");

        new_config.strapi_version = package_json_obj.dependencies.strapi;
    }

    print_generated_config(&mut new_config);

    println!("[Y] - Accept all  [N] - Overwrite configuration");

    let mut configuration_input: String = String::new();

    stdin()
        .read_line(&mut configuration_input)
        .expect("Failed to read from stdin");

    configuration_input = configuration_input
        .to_lowercase()
        .split_whitespace()
        .collect();

    if !configuration_input.is_empty() {
        if configuration_input == "n" && configuration_input != "y" {
            override_config(&mut new_config);
        }
        if configuration_input != "n" && configuration_input != "y" {
            println!("{}", "Exited!".red());
            return ();
        }
    } else {
        println!("{}", "Exited!".red());
        return ();
    }

    // Choose tables
    if select_tables(&mut new_config) {
        let config_json: String = serde_json::to_string_pretty(&new_config).unwrap();
        write_to_file(CONFIG_FILE, config_json.as_str()).ok();
    }

    // Ask if user wants to create replica
    println!();
    println!("Should a replication be created?");
    println!("[Y] - Replicate  [N] - Do not replicate");

    let mut configuration_input: String = String::new();

    stdin()
        .read_line(&mut configuration_input)
        .expect("Failed to read from stdin");

    configuration_input = configuration_input
        .to_lowercase()
        .split_whitespace()
        .collect();

    if configuration_input == "y" {
        run_replicate(args);
    }

    return ();
}

/// Start table selection dialog
fn select_tables(new_config: &mut Config) -> bool {
    println!();
    println!("{}", "Use standard selected tables?".green());
    for table in &new_config.replicated {
        println!("{}", table);
    }

    println!();
    println!("[Y] - Accept Selection  [N] - Select manually");

    let mut configuration_input: String = String::new();

    stdin()
        .read_line(&mut configuration_input)
        .expect("Failed to read from stdin");

    configuration_input = configuration_input
        .to_lowercase()
        .split_whitespace()
        .collect();

    if configuration_input != "y" {
        // Connect to database
        // List all tables / collections
        let tables = get_tables(new_config).clone();

        println!();
        println!("{}", "Tables / Collections: ".green());

        let mut index: i32 = 0;
        for table in tables.clone() {
            index += 1;
            println!("({}) {}", index, table);
        }

        println!();
        println!("Select tables/collections separated by ','");

        let mut selected_tables: String = String::new();
        print!("{}", "Selected tables/collections:".green());
        let _ = stdout().flush();
        stdin()
            .read_line(&mut selected_tables)
            .expect("Did not enter a correct string");

        let regex = Regex::new("^[0-9]+(,[0-9]+)*$").unwrap();

        selected_tables = selected_tables.to_lowercase().split_whitespace().collect();

        if !regex.is_match(&selected_tables) {
            println!(
                "{}",
                "Wrong format used for selecting tables/collections!".red()
            );
            return false;
        }

        let mut named_selected_tables: Vec<String> = Vec::new();
        for table_index in selected_tables.split(",") {
            let index: usize = match table_index.parse() {
                Ok(value) => value,
                Err(_) => continue,
            };

            if !&tables[index - 1].is_empty() {
                let table_name: String = String::from(&tables[index - 1]);
                named_selected_tables.push(table_name);

                new_config.replicated = named_selected_tables.clone();
            }
        }
    }

    return true;
}

/// Print all tables / collections in database
fn get_tables(new_config: &mut Config) -> Vec<String> {
    let raw_tables = databases::list(new_config.clone());
    if raw_tables.is_some() {
        let tables = raw_tables.unwrap();

        return tables;
    }

    return vec![];
}

/// Override data manually
fn override_config(new_config: &mut Config) {
    new_config.strapi_version = get_config_input("Strapi version:".to_string());

    let database_type: String = get_config_input("Database type [mysql,mongodb]:".to_string());

    new_config.database.database_type = match database_type.to_lowercase().as_str() {
        "mysql" => "mysql".to_string(),
        "mongodb" => "mysql".to_string(),
        _ => "mysql".to_string(),
    };

    new_config.database.database_version = get_config_input("Database version:".to_string());
    new_config.database.database_name = get_config_input("Database name:".to_string());

    new_config.database.host_information.address = get_config_input("Address:".to_string());
    new_config.database.host_information.port = get_config_input("Port:".to_string());
    new_config.database.host_information.username = get_config_input("Username:".to_string());
    new_config.database.host_information.password = get_config_input("Password:".to_string());

    new_config.database.host_information.ssl =
        match get_config_input("Enable ssl [true,false]:".to_string()).parse() {
            Ok(value) => value,
            Err(_) => false,
        };
}

/// Print generated config
fn print_generated_config(new_config: &mut Config) {
    println!("{}", "Generated config:".green());

    println!("Strapi version: {}", new_config.strapi_version);
    println!();

    println!("{}", "Database Information".blue());
    println!("Type: {}", new_config.database.database_type);
    println!("Version: {}", new_config.database.database_version);
    println!("Name: {}", new_config.database.database_name);

    println!();

    println!("{}", "Hosting Information".blue());
    println!("Address: {}", new_config.database.host_information.address);
    println!("Port: {}", new_config.database.host_information.port);
    println!(
        "Username: {}",
        new_config.database.host_information.username
    );
    println!(
        "Password: {}",
        new_config.database.host_information.password
    );
    println!("SSL Enabled: {}", new_config.database.host_information.ssl);

    println!();
}
