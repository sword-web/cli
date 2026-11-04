pub mod cli;
pub mod commands;
pub mod error;
pub mod naming;
pub mod services;
pub mod types;

use crate::cli::Cli;

use camino::Utf8PathBuf;
use clap::Parser;
use error::{AppResult, CliError};

pub fn run() -> std::process::ExitCode {
    match try_run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            let _ = services::output::outro_cancel(error.to_string());
            std::process::ExitCode::FAILURE
        }
    }
}

fn try_run() -> AppResult<()> {
    let cli = Cli::parse();
    let cwd = resolve_cwd(&cli)?;

    commands::dispatch(cli.command, cwd)
}

fn resolve_cwd(cli: &Cli) -> AppResult<Utf8PathBuf> {
    match &cli.cwd {
        Some(path) => Utf8PathBuf::from_path_buf(path.clone()).map_err(|path| {
            CliError::invalid_cwd(
                Utf8PathBuf::from_path_buf(path).unwrap_or_default(),
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
