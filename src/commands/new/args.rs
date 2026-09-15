use crate::types::ProjectType;

#[derive(Debug, clap::Args)]
pub struct NewArgs {
    pub name: String,

    #[arg(long = "type", value_enum)]
    pub project_type: Option<ProjectType>,

    #[arg(long)]
    pub no_git: bool,

    /// Accept all defaults without prompting (non-interactive).
    #[arg(long, short = 'y')]
    pub yes: bool,
}
