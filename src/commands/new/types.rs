use crate::error::CliError;
use crate::naming;
use crate::types::ProjectType;

use camino::Utf8Path;
use clap::ValueEnum;
use convert_case::{Case, Casing};

/// Validated project name.
///
/// The directory keeps the user input, while the crate name is normalized to
/// kebab-case.
#[derive(Debug, Clone)]
pub struct ProjectName {
    directory: String,
    crate_name: String,
}

impl ProjectName {
    pub fn parse(input: &str) -> Result<Self, CliError> {
        let raw = naming::validate(input)?;

        Ok(Self {
            directory: raw.to_string(),
            crate_name: crate_name(raw),
        })
    }

    /// Builds a name from the directory the command runs in (`sword new .`).
    ///
    /// The directory itself can contain characters that are not valid in a
    /// crate name (for example, a temporary directory starting with a dot), so
    /// the crate name is sanitized instead of rejected.
    pub fn from_cwd(cwd: &Utf8Path) -> Self {
        let raw = cwd.file_name().unwrap_or("app");
        let directory = sanitize_directory(raw);

        Self {
            directory: raw.to_string(),
            crate_name: crate_name(&directory),
        }
    }

    pub fn directory(&self) -> &str {
        &self.directory
    }

    pub fn crate_name(&self) -> &str {
        &self.crate_name
    }
}

/// Optional features the generated project is configured with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Feature {
    Multipart,
    Validator,
    GrpcReflection,
}

impl Feature {
    pub fn name(&self) -> &'static str {
        match self {
            Feature::Multipart => "multipart",
            Feature::Validator => "validator",
            Feature::GrpcReflection => "grpc-reflection",
        }
    }

    fn sword_feature(&self) -> &'static str {
        match self {
            Feature::Multipart => "web-multipart",
            Feature::Validator => "validation-validator",
            Feature::GrpcReflection => "grpc-reflection",
        }
    }

    fn supports(self, project_type: ProjectType) -> bool {
        match self {
            Feature::Multipart => project_type == ProjectType::Web,
            Feature::Validator => {
                matches!(project_type, ProjectType::Web | ProjectType::Socketio)
            }
            Feature::GrpcReflection => project_type == ProjectType::Grpc,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FeatureSet {
    pub multipart: bool,
    pub validator: bool,
    pub grpc_reflection: bool,
}

impl FeatureSet {
    pub fn from_features(features: &[Feature]) -> Self {
        Self {
            multipart: features.contains(&Feature::Multipart),
            validator: features.contains(&Feature::Validator),
            grpc_reflection: features.contains(&Feature::GrpcReflection),
        }
    }

    pub fn validate(&self, project_type: ProjectType) -> Result<(), CliError> {
        let selected = [
            (Feature::Multipart, self.multipart),
            (Feature::Validator, self.validator),
            (Feature::GrpcReflection, self.grpc_reflection),
        ];

        for (feature, enabled) in selected {
            if enabled && !feature.supports(project_type) {
                return Err(CliError::unsupported_feature(
                    feature.name(),
                    project_type.name(),
                ));
            }
        }

        Ok(())
    }

    /// Feature names in the format expected by Sword's `Cargo.toml`.
    pub fn sword_features(&self) -> Vec<&'static str> {
        let mut features = Vec::new();

        if self.multipart {
            features.push(Feature::Multipart.sword_feature());
        }

        if self.validator {
            features.push(Feature::Validator.sword_feature());
        }

        if self.grpc_reflection {
            features.push(Feature::GrpcReflection.sword_feature());
        }

        features
    }
}

/// Git setup for the generated project.
#[derive(Debug, Clone, Default)]
pub struct GitConfig {
    pub init: bool,
    pub remote: Option<String>,
}

/// Fully specified options for a `new` command run.
#[derive(Debug, Clone)]
pub struct NewProjectOptions {
    pub name: ProjectName,
    pub project_type: ProjectType,
    pub features: FeatureSet,
    pub git: GitConfig,
}

/// Cargo package names keep digits together and use `-` as separator.
fn crate_name(raw: &str) -> String {
    raw.to_lowercase().replace('_', "-")
}

/// Turns an arbitrary directory name into a valid crate/module name.
fn sanitize_directory(raw: &str) -> String {
    let mut value: String = raw
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();

    value = value.trim_matches(|c| c == '_' || c == '-').to_string();

    if value.is_empty() {
        value = "app".to_string();
    }

    if value.starts_with(|c: char| c.is_ascii_digit()) {
        value.insert_str(0, "app_");
    }

    if naming::is_keyword(&value.to_case(Case::Snake)) {
        value = format!("app_{value}");
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    use camino::Utf8Path;

    #[test]
    fn normalizes_crate_and_module_names() {
        let name = ProjectName::parse("My App").unwrap_err();
        assert!(name.to_string().contains("only letters"));

        let name = ProjectName::parse("my-app").unwrap();
        assert_eq!(name.directory(), "my-app");
        assert_eq!(name.crate_name(), "my-app");
    }

    #[test]
    fn sanitizes_the_current_directory_name() {
        let name = ProjectName::from_cwd(Utf8Path::new("/tmp/.tmp123"));
        assert_eq!(name.crate_name(), "tmp123");

        let name = ProjectName::from_cwd(Utf8Path::new("/tmp/9app"));
        assert_eq!(name.crate_name(), "app-9app");
    }

    #[test]
    fn rejects_invalid_names() {
        assert!(ProjectName::parse("").is_err());
        assert!(ProjectName::parse("9app").is_err());
        assert!(ProjectName::parse("type").is_err());
        assert!(ProjectName::parse("my/app").is_err());
        assert!(ProjectName::parse("-app").is_err());
        assert!(ProjectName::parse("app_").is_err());
    }

    #[test]
    fn validates_features_per_project_type() {
        let features = FeatureSet {
            multipart: true,
            ..Default::default()
        };

        assert!(features.validate(ProjectType::Web).is_ok());
        assert!(features.validate(ProjectType::Socketio).is_err());
        assert!(features.validate(ProjectType::Grpc).is_err());
    }
}
