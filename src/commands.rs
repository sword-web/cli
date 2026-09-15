pub mod new {
    mod action;
    mod args;
    mod render;
    mod types;

    pub use action::NewAction;
    pub use args::NewArgs;
}

use crate::cli::Command as CliCommand;
use crate::commands::new::NewAction;
use crate::error::AppResult;
use camino::Utf8PathBuf;

/// A CLI subcommand.
pub trait Command {
    fn execute(&self) -> AppResult<()>;
}

impl Command for NewAction {
    fn execute(&self) -> AppResult<()> {
        self.handle()
    }
}

/// Resolves the parsed CLI command and runs it.
pub fn dispatch(command: CliCommand, cwd: Utf8PathBuf) -> AppResult<()> {
    match command {
        CliCommand::New(args) => NewAction::new(args, cwd).execute(),
    }
}
