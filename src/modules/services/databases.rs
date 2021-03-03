use crate::consts::DATABASES;
use crate::modules::types::config::Config;

mod mongodb;
mod mysql;

/// List all tables / collection for given database
pub fn list(config: Config) -> Option<Vec<String>> {
    let database_type: DATABASES = get_database_type(config.database.database_type.clone());
    return match database_type {
        DATABASES::MongoDB => mongodb::list_tables(config),
        DATABASES::MySQL => mysql::list_tables(config),
    };
}

/// Replicate table or collection
pub fn replicate_table(table_name: String, config: Config) -> Option<String> {
    let database_type: DATABASES = get_database_type(config.database.database_type.clone());

    return match database_type {
        DATABASES::MongoDB => mongodb::get_table(table_name, config),
        DATABASES::MySQL => mysql::get_table(table_name, config),
    };
}

/// Migrate table or collection
pub fn migrate_table(data: String, table: String, config: Config) {
    let database_type: DATABASES = get_database_type(config.database.database_type.clone());

    return match database_type {
        DATABASES::MongoDB => mongodb::import_table(data, table, config),
        DATABASES::MySQL => mysql::import_table(data, table, config),
    };
}

/// Return typ of database
fn get_database_type(string: String) -> DATABASES {
    return match string.as_str() {
        "mysql" => DATABASES::MySQL,
        "mongodb" => DATABASES::MongoDB,
        _ => DATABASES::MySQL,
    };
}

// TODO: Add trait impl. for basic / standard database class -> basic interface
// TODO: Add postgres support
