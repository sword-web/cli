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

    #[error("invalid name `{name}`: {reason}")]
    InvalidName { name: String, reason: String },

    #[error("not a Sword project: {path}")]
    NotAProject { path: String },

    #[error("component already exists: {path}")]
    ComponentExists { path: String },

    #[error("module not found: {path}. Run `sword generate module {name}` first")]
    MissingModule { path: String, name: String },

    #[error("wiring marker not found in {path}: `{marker}`")]
    MissingMarker { path: String, marker: String },

    #[error("unsupported feature `{feature}` for a {project_type} project")]
    UnsupportedFeature {
        feature: String,
        project_type: String,
    },

    #[error("phase `{phase}` is not available for a {project_type} project")]
    UnsupportedPhase { phase: String, project_type: String },

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

    pub fn invalid_name(name: &str, reason: &str) -> Self {
        Self::InvalidName {
            name: name.to_string(),
            reason: reason.to_string(),
        }
    }

    pub fn not_a_project(path: camino::Utf8PathBuf) -> Self {
        Self::NotAProject {
            path: path.to_string(),
        }
    }

    pub fn component_exists(path: camino::Utf8PathBuf) -> Self {
        Self::ComponentExists {
            path: path.to_string(),
        }
    }

    pub fn missing_module(path: camino::Utf8PathBuf, name: &str) -> Self {
        Self::MissingModule {
            path: path.to_string(),
            name: name.to_string(),
        }
    }

    pub fn missing_marker(path: camino::Utf8PathBuf, marker: &str) -> Self {
        Self::MissingMarker {
            path: path.to_string(),
            marker: marker.to_string(),
        }
    }

    pub fn unsupported_feature(feature: &str, project_type: &str) -> Self {
        Self::UnsupportedFeature {
            feature: feature.to_string(),
            project_type: project_type.to_string(),
        }
    }
}
