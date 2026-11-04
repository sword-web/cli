use super::types::NewProjectOptions;
use crate::types::{ProjectType, SWORD_REV};

use serde::Serialize;

/// Context injected into every `new` `.liquid` template at render time.
#[derive(Debug, Serialize)]
pub struct RenderContext {
    project_name: String,
    project_type: String,
    sword_rev: &'static str,
    sword_features_quoted: String,
    host: &'static str,
    port: u16,
}

impl From<&NewProjectOptions> for RenderContext {
    fn from(options: &NewProjectOptions) -> Self {
        let (host, port) = match options.project_type {
            ProjectType::Grpc => ("127.0.0.1", 50051u16),
            _ => ("127.0.0.1", 8080u16),
        };

        let mut features = vec![options.project_type.name().to_string()];
        features.extend(
            options
                .features
                .sword_features()
                .into_iter()
                .map(str::to_string),
        );

        Self {
            project_name: options.name.crate_name().to_string(),
            project_type: options.project_type.name().to_string(),
            sword_rev: SWORD_REV,
            sword_features_quoted: features
                .iter()
                .map(|feature| format!("\"{feature}\""))
                .collect::<Vec<_>>()
                .join(", "),
            host,
            port,
        }
    }
}
