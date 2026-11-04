use super::types::Feature;
use crate::types::ProjectType;

#[derive(Debug, clap::Args)]
pub struct NewArgs {
    /// Project directory and crate name.
    pub name: String,

    /// Application kind. Defaults to `web` with `--yes`.
    #[arg(long = "type", value_enum)]
    pub project_type: Option<ProjectType>,

    /// Enable an optional framework feature. Repeatable.
    #[arg(long = "feature", value_enum, value_name = "FEATURE")]
    pub features: Vec<Feature>,

    /// Initialize a git repository.
    #[arg(long, conflicts_with = "no_git")]
    pub git: bool,

    /// Initialize git and add the remote as `origin`.
    #[arg(long, value_name = "URL", conflicts_with = "no_git")]
    pub remote: Option<String>,

    /// Skip git initialization.
    #[arg(long, conflicts_with_all = ["git", "remote"])]
    pub no_git: bool,

    /// Accept all defaults without prompting (non-interactive).
    #[arg(long, short = 'y')]
    pub yes: bool,
}
