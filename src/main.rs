pub mod cli;
pub mod commands;
pub mod error;
pub mod services;
pub mod types;

use crate::cli::Cli;

use camino::Utf8PathBuf;
use clap::Parser;
use error::{AppResult, CliError};

fn main() -> AppResult<()> {
    let cli = Cli::parse();
    let cwd = resolve_cwd(&cli)?;

    if let Err(error) = commands::dispatch(cli.command, cwd) {
        services::output::outro_cancel(describe(&error))?;
    }

    Ok(())
}

fn resolve_cwd(cli: &Cli) -> AppResult<Utf8PathBuf> {
    match &cli.cwd {
        Some(path) => Utf8PathBuf::from_path_buf(path.clone()).map_err(|_| {
            CliError::invalid_cwd(
                Utf8PathBuf::from_path_buf(path.clone()).unwrap_or_default(),
            )
        }),
        None => {
            let path = std::env::current_dir()
                .map_err(|_e| CliError::invalid_cwd(Utf8PathBuf::from(".")))?;

            Utf8PathBuf::from_path_buf(path)
                .map_err(|_| CliError::invalid_cwd(Utf8PathBuf::from(".")))
        }
    }
}

fn describe(error: &CliError) -> String {
    match error {
        CliError::InvalidCwd { path } => {
            format!("Invalid working directory: {path}")
        }
        CliError::InvalidProject { path } => {
            format!("Invalid project directory: {path}")
        }
        CliError::ProjectExists { path } => {
            format!("Project already exists: {path}")
        }
        CliError::Io(source) => format!("IO error: {source}"),
        CliError::Render { phase, message } => {
            format!("Template {phase} error: {message}")
        }
    }
}
