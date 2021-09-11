use std::env;
use std::path::Path;

// use quicli::prelude::{read_file, CliResult};
use colored::Colorize;
use structopt::StructOpt;

use modules::types::cli::Cli;

use crate::modules::commands::{init, migrate, replicate, tables};

mod consts;
mod modules;

fn main() -> () {
    let args: Cli = Cli::from_args();

    match args.path.is_empty() {
        false => {
            let root = Path::new(&args.path);
            assert!(env::set_current_dir(&root).is_ok());
        }
        _ => {}
    }

    if !args.use_env {
        dotenv::vars();
    }

    println!(
        "{}",
        "Welcome to the persistent data replicator for Strapi:".green()
    );
    println!("Version: {}", env!("CARGO_PKG_VERSION").to_string());
    print!("\n");

    let command = &args.command;
    match command.as_str() {
        "init" | "Init" => init(args),
        "replicate" | "Replicate" => replicate(args),
        "migrate" | "Migrate" => migrate(args),
        "tables" | "Tables" => tables(args),
        _ => {}
    }
}
