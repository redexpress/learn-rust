mod cli;
mod database;

use std::error::Error;

use clap::Parser;

use crate::cli::{Cli, SqliteCmd, SqlxCmd};
#[cfg(unix)]
use crate::cli::RockdbCmd;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result: Result<(), Box<dyn Error>> = match cli {
        Cli::Sqlite { cmd } => match cmd {
            SqliteCmd::Demo => database::sqlite::run(cmd).map_err(|e| e.into()),
        },
        Cli::Sqlx { cmd } => match cmd {
            SqlxCmd::Demo => database::sqlx::run(cmd).await.map_err(|e| e.into()),
        },
        #[cfg(unix)]
        Cli::Rockdb { cmd } => match cmd {
            RockdbCmd::Demo => database::rockdb::run(cmd).map_err(|e| e.into()),
        },
    };

    if let Err(e) = result {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
