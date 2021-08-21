use std::env;
use std::fs::{create_dir, read_dir, File, ReadDir};
use std::io::{stdin, stdout, BufReader, Write};
use std::path::Path;

use colored::Colorize;
use humanesort::prelude::*;
use regex::Regex;

use crate::consts::{CONFIG_FILE, DATA_PATH};
use crate::modules::types::config::{Config, DatabaseConfig, HostInformation};
use semver::{BuildMetadata, Prerelease, Version};

/// Get next replication version
pub fn get_next_version() -> String {
    let mut version: Version;

    let mut legit_paths = get_legit_paths();

    // Return first version
    if legit_paths.is_empty() {
        version = Version {
            major: 0,
            minor: 0,
            patch: 1,
            pre: Prerelease::EMPTY,
            build: BuildMetadata::EMPTY,
        };

        return version.to_string();
    }

    legit_paths.humane_sort();

    let last_path = legit_paths.last().unwrap();

    version = Version::parse(last_path).expect("Could not parse version!");

    // Increment version | 10 is max
    if version.patch == 10 {
        version.patch = 0;
        if version.minor == 10 {
            version.minor = 0;
            version.major += 1;
        } else {
            version.minor += 1;
        }
    } else {
        version.patch += 1;
    }

    return version.to_string();
}

/// Return latest replication version
pub fn get_latest_replication_version() -> String {
    let version: Version;
    let mut legit_paths = get_legit_paths();

    legit_paths.humane_sort();

    let last_path = legit_paths.last().unwrap();

    version = Version::parse(last_path).expect("Could not parse version!");

    return version.to_string();
}

/// Verify given replication version
pub fn verify_replication_version(version: String) -> bool {
    let mut version_exists: bool = false;
    let legit_paths = get_legit_paths();

    for legit_path in legit_paths {
        if legit_path == version {
            Version::parse(legit_path.as_str()).expect("Could not parse version!");
            version_exists = true;
            break;
        }
    }

    return version_exists;
}

/// Return ReadDir for all replications files of specific version
pub fn read_replication_file_paths(version: String) -> ReadDir {
    let paths = read_dir(format!("{}{}", DATA_PATH, version)).expect("Cannot read directory!");

    return paths;
}

/// Create data dir if directory does not exist
pub fn create_data_dir() {
    if !Path::new(DATA_PATH).exists() {
        create_dir(DATA_PATH).expect("Could not create directory!");
    }
}

/// Check if project is already existent
pub fn init_check(print_message: bool) -> bool {
    if !check_path_existence(CONFIG_FILE, print_message) {
        return false;
    }
    // Check if config file already exists
    read_config_file();

    return true;
}

/// Read config file and return config
pub fn read_config_file() -> Config {
    // Open the file in read-only mode with buffer.
    // let file = File::open(CONFIG_FILE).expect("Unable to read file!");
    let file = File::open(CONFIG_FILE);

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem with opening the file: {:?}", error),
    };

    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Config`.
    let config: Config = serde_json::from_reader(reader).expect("Unable to parse string to json!");

    return config;
}

/// Return config using environment variables
/// and the config file for replicated tables
pub fn get_config_using_env() -> Config {
    let file_config = read_config_file();

    let mut config = Config {
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
        replicated: vec![],
    };

    config.replicated = file_config.replicated.clone();

    return config;
}

/// Request user input with given string as explanation
/// and return it
pub fn get_config_input(variable_text: String) -> String {
    let mut s = String::new();
    print!("{}", variable_text);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");

    return s.split_whitespace().collect();
}

/// Make environment variable to lowercase and remove whitespaces
pub fn get_config_string(key: &str) -> String {
    let formatted_string: String;

    let option_string = env::var(key);

    formatted_string = match option_string {
        Ok(env) => env,
        Err(_) => String::new(),
    }
    .split_whitespace()
    .collect();

    return formatted_string;
}

/// Check if given file exists
pub fn check_path_existence(path: &str, print_message: bool) -> bool {
    // Check if path and file exists
    if !Path::new(path).exists() {
        if print_message {
            let error_message = format!("Path {} does not exist!", path);
            println!("{}", error_message.red());
        }

        return false;
    }

    return true;
}

/// Get legit paths in data dir
fn get_legit_paths() -> Vec<String> {
    let regex = Regex::new("^(\\d+\\.)?(\\d+\\.)?(\\*|\\d+)$").unwrap();
    let paths = read_dir(DATA_PATH).expect("Cannot read directory!");
    let mut legit_paths: Vec<String> = vec![];

    for path in paths {
        let file_name = path.unwrap().file_name().to_string_lossy().to_string();

        if regex.is_match(&file_name.as_str()) {
            legit_paths.push(file_name)
        }
    }

    return legit_paths;
}
