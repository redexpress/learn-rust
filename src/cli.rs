use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lrust", version, about)]
pub(crate) enum Cli {
    Sqlite {
        #[command(subcommand)]
        cmd: SqliteCmd,
    },
    Sqlx {
        #[command(subcommand)]
        cmd: SqlxCmd,
    },
    #[cfg(unix)]
    Rockdb {
        #[command(subcommand)]
        cmd: RockdbCmd,
    },
}

#[derive(Subcommand)]
pub(crate) enum SqliteCmd {
    Demo,
}

#[derive(Subcommand)]
pub(crate) enum SqlxCmd {
    Demo,
}

#[cfg(unix)]
#[derive(Subcommand)]
pub(crate) enum RockdbCmd {
    Demo,
}
