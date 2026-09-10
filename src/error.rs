use thiserror::Error;

pub type AppResult<T> = Result<T, CliError>;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid project directory: {path}")]
    InvalidProject { path: String },

    #[error("invalid working directory: {path}")]
    InvalidCwd { path: String },

    #[error("project already exists: {path}")]
    ProjectExists { path: String },

    #[error("failed to {phase} template: {message}")]
    Render { phase: String, message: String },
}

impl CliError {
    pub fn render(phase: &str, message: String) -> Self {
        Self::Render {
            phase: phase.to_string(),
            message,
        }
    }

    pub fn invalid_project(path: camino::Utf8PathBuf) -> Self {
        Self::InvalidProject {
            path: path.to_string(),
        }
    }

    pub fn invalid_cwd(path: camino::Utf8PathBuf) -> Self {
        Self::InvalidCwd {
            path: path.to_string(),
        }
    }

    pub fn project_exists(path: camino::Utf8PathBuf) -> Self {
        Self::ProjectExists {
            path: path.to_string(),
        }
    }
}
