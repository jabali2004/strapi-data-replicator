use std::borrow::Cow;
use std::process::Command;

use colored::Colorize;
use mysql::prelude::*;
use mysql::*;

use crate::modules::types::config::Config;

/// List all tables for given database
pub fn list_tables(config: Config) -> Option<Vec<String>> {
    let mut conn =
        get_connection(config.clone()).expect("Error while trying to connect to mysql database!");

    let select_tables_query = format!(
        "SELECT table_name FROM information_schema.tables
                   WHERE table_schema = '{}';",
        config.database.database_name
    );

    let tables: Vec<String> = conn
        .query(select_tables_query)
        .expect("Error while running select query!");

    return Option::from(tables);
}

/// Return table as sql string using mysqldump
pub fn get_table(table_name: String, config: Config) -> Option<String> {
    let host_information = config.database.host_information.clone();

    let dump_command;
    let mut program_name: &str = "mysqldump";

    if cfg!(windows) {
        program_name = "mysqldump.exe";
    } else if cfg!(linux) {
        program_name = "mysqldump";
    }

    let password_len = host_information.password.len();
    if password_len > 0 {
        dump_command = Command::new(program_name)
            .args(&[
                format!("-u{}", host_information.username).as_str(),
                format!("-p={}", host_information.password).as_str(),
                format!("--host={}", host_information.address).as_str(),
                format!("--port={}", host_information.port).as_str(),
                format!("{}", config.database.database_name).as_str(),
                format!("{}", table_name).as_str(),
                "--no-create-info",
                "--skip-triggers",
                "--no-create-db",
                "--compact",
                "--column-statistics=0",
            ])
            .output()
            .expect("Error not able to run command!");
    } else {
        dump_command = Command::new(program_name)
            .args(&[
                format!("-u{}", host_information.username).as_str(),
                format!("--host={}", host_information.address).as_str(),
                format!("--port={}", host_information.port).as_str(),
                format!("{}", config.database.database_name).as_str(),
                format!("{}", table_name).as_str(),
                "--no-create-info",
                "--skip-triggers",
                "--no-create-db",
                "--compact",
                "--column-statistics=0",
            ])
            .output()
            .expect("Error not able to run command!");
    }

    let dump: Cow<str> = String::from_utf8_lossy(&dump_command.stdout);

    let dump_error = String::from_utf8_lossy(&dump_command.stderr);
    if dump_error.len() > 0 {
        println!("{} {}", "Error:".red(), dump_error);
    }

    if dump.len() > 0 {
        return Option::from(dump.to_string());
    }
    return None;
}

// Import table given as sql string
pub fn import_table(sql: String, table_name: String, config: Config) -> () {
    let mut conn =
        get_connection(config.clone()).expect("Error while trying to connect to mysql database!");
    let params = Params::from(());

    conn.query(format!("DELETE FROM `{}`;", table_name))
        .expect("Could not delete all rows of table!")
        .to_value();

    let prepped_sql = conn
        .prep(sql)
        .expect("Could not convert file to valid stmt!");

    conn.exec(prepped_sql, params)
        .expect("Error while running select query!")
        .to_value();
}

/// Get mysql connection configuration
fn get_opts(config: Config) -> OptsBuilder {
    let tcp_port: u16 = match config.database.host_information.port.parse() {
        Ok(value) => value,
        Err(_) => 3306,
    };

    let ssl_opts = SslOpts::default();

    SslOpts::accept_invalid_certs(&ssl_opts);
    SslOpts::skip_domain_validation(&ssl_opts);

    let opts = OptsBuilder::new()
        .ip_or_hostname(Option::from(config.database.host_information.address))
        .tcp_port(tcp_port)
        .user(Option::from(config.database.host_information.username))
        .pass(Option::from(config.database.host_information.password))
        .db_name(Option::from(config.database.database_name))
        .ssl_opts(match config.database.host_information.ssl {
            false => None,
            true => Some(ssl_opts),
        });

    return opts;
}

/// Create connection with mysql server and return ref
fn get_connection(config: Config) -> Result<PooledConn> {
    let pool = Pool::new(get_opts(config))?;

    let conn = pool.get_conn();

    return conn;
}
