use super::types::NewProjectOptions;
use crate::types::ProjectType;

use serde::Serialize;

/// Context injected into every `new` `.liquid` template at render time.
#[derive(Debug, Serialize)]
pub struct RenderContext {
    project_name: String,
    project_type: String,
    sword_features_quoted: String,
    db: Option<String>,
    db_flavor: Option<String>,
    addons: Vec<String>,
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

        if options.features.multipart {
            features.push("web-multipart".to_string());
        }

        if options.features.validator {
            features.push("validation-validator".to_string());
        }

        if options.features.grpc_reflection {
            features.push("grpc-reflection".to_string());
        }

        let (db, db_flavor) = match options.database.as_template() {
            Some((db, flavor)) => (Some(db.to_string()), flavor.map(str::to_string)),
            None => (None, None),
        };

        Self {
            project_name: options.name.as_str().to_string(),
            project_type: options.project_type.name().to_string(),
            sword_features_quoted: features
                .iter()
                .map(|feature| format!("\"{feature}\""))
                .collect::<Vec<_>>()
                .join(", "),
            db,
            db_flavor,
            addons: options.addons.clone(),
            host,
            port,
        }
    }
}
