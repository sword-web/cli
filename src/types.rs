use camino::Utf8PathBuf;
use clap::ValueEnum;

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
