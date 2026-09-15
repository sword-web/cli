use crate::commands::new::NewArgs;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "sword",
    version,
    about = "Sword CLI for project scaffolding",
    propagate_version = true
)]
pub struct Cli {
    #[arg(long, global = true, value_name = "PATH")]
    pub cwd: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    New(NewArgs),
}
