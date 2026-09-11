pub mod cli;
pub mod commands;
pub mod error;
pub mod types;

use crate::cli::Cli;

use camino::Utf8PathBuf;
use clap::Parser;
use error::{AppResult, CliError};

fn main() -> AppResult<()> {
    let cli = Cli::parse();
    let cwd = resolve_cwd(&cli)?;

    println!("{cwd}");

    Ok(())
}

fn resolve_cwd(cli: &Cli) -> AppResult<Utf8PathBuf> {
    match &cli.cwd {
        Some(path) => Utf8PathBuf::from_path_buf(path.clone())
            .map_err(|_| CliError::invalid_cwd(Utf8PathBuf::from("."))),
        None => {
            let path = std::env::current_dir()
                .map_err(|_e| CliError::invalid_cwd(Utf8PathBuf::from(".")))?;

            Utf8PathBuf::from_path_buf(path)
                .map_err(|_| CliError::invalid_cwd(Utf8PathBuf::from(".")))
        }
    }
}
