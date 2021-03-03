use colored::*;

use crate::modules::commands::init::run_init;
use crate::modules::commands::migrate::run_migrate;
use crate::modules::commands::replicate::run_replicate;
use crate::modules::services::utils::init_check;
use crate::modules::types::cli::Cli;

pub mod init;
pub mod migrate;
pub mod replicate;

/// Init new replicator
pub fn init(args: Cli) -> () {
    if args.overwrite {
        run_init(args);
    } else {
        let init_state: bool = init_check(false);

        // Check if project already exists
        if !init_state {
            run_init(args);
        } else {
            println!("{}", "Replicator config already exists!".red());
        }
    }
}

/// Replicate persistent data
pub fn replicate(args: Cli) -> () {
    let init_state: bool = init_check(false);

    if init_state {
        run_replicate(args);
    } else {
        println!("{}", "Replicator config does not exist! Init first!".red());
    }
}

/// Migrate persistent data to database
pub fn migrate(args: Cli) -> () {
    let init_state: bool = init_check(false);

    if init_state {
        run_migrate(args);
    } else {
        println!("{}", "Replicator config does not exist! Init first!".red());
    }
}

// TODO: Add delete command for deleting replicated data
