use camino::Utf8PathBuf;
use clap::ValueEnum;

/// Framework revision pinned by generated projects.
///
/// Generated projects must be reproducible. Until the framework publishes a
/// version with the current API, the templates point at this exact revision.
pub const SWORD_REV: &str = "a29dc3017f0e40de9b6ffd6ac9fa24b6724e1c87";

/// Project kind selected for a `new` run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ProjectType {
    Web,
    Socketio,
    Grpc,
}

impl ProjectType {
    /// Template-friendly name.
    pub fn name(&self) -> &'static str {
        match self {
            ProjectType::Web => "web",
            ProjectType::Socketio => "socketio",
            ProjectType::Grpc => "grpc",
        }
    }

    /// Parses a project type from its template-friendly name.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "web" => Some(ProjectType::Web),
            "socketio" => Some(ProjectType::Socketio),
            "grpc" => Some(ProjectType::Grpc),
            _ => None,
        }
    }
}

/// A file to be written, with its rendered contents.
#[derive(Debug, Clone)]
pub struct PlannedFile {
    pub path: String,
    pub contents: String,
}

/// Everything needed to materialize a project on disk.
#[derive(Debug, Clone)]
pub struct GenerationPlan {
    pub root: Utf8PathBuf,
    pub files: Vec<PlannedFile>,
}
