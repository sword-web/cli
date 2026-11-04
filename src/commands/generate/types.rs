use crate::error::{AppResult, CliError};
use crate::naming;
use crate::types::ProjectType;

use camino::{Utf8Path, Utf8PathBuf};
use clap::ValueEnum;
use convert_case::{Case, Casing};

/// Name of a generated component, normalized for files and Rust types.
#[derive(Debug, Clone)]
pub struct ComponentName {
    file_name: String,
    type_name: String,
}

impl ComponentName {
    pub fn parse(input: &str) -> AppResult<Self> {
        let raw = naming::validate(input)?;

        Ok(Self {
            type_name: raw.to_case(Case::Pascal),
            file_name: raw.to_case(Case::Snake),
        })
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }
}

/// Controller transport supported by the generator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ControllerKind {
    Web,
    #[value(name = "socketio")]
    SocketIo,
}

/// The project `generate` operates on.
#[derive(Debug, Clone)]
pub struct Project {
    pub project_type: ProjectType,
    pub source: Utf8PathBuf,
    pub main: Utf8PathBuf,
}

impl Project {
    /// Finds the closest Cargo manifest that declares the Sword metadata.
    pub fn discover(cwd: &Utf8Path) -> AppResult<Self> {
        for ancestor in cwd.ancestors() {
            let manifest = ancestor.join("Cargo.toml");

            if !manifest.is_file() {
                continue;
            }

            let contents = std::fs::read_to_string(&manifest)?;

            if let Some(project_type) = parse_project_type(&contents) {
                let source = ancestor.join("src");
                let main = source.join("main.rs");

                if !main.is_file() {
                    return Err(CliError::not_a_project(ancestor.to_owned()));
                }

                return Ok(Self {
                    project_type,
                    source,
                    main,
                });
            }
        }

        Err(CliError::not_a_project(cwd.to_owned()))
    }

    pub fn module_file(&self, module: &ComponentName) -> Utf8PathBuf {
        self.source.join(module.file_name()).join("mod.rs")
    }
}

fn parse_project_type(manifest: &str) -> Option<ProjectType> {
    let mut in_section = false;

    for line in manifest.lines() {
        let line = line.trim();

        if line.starts_with('[') {
            in_section = line == "[package.metadata.sword]";
            continue;
        }

        if !in_section {
            continue;
        }

        let Some(value) = line.strip_prefix("project-type") else {
            continue;
        };

        let value = value.trim_start().strip_prefix('=')?.trim();
        return ProjectType::parse(value.trim_matches('"'));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_component_names() {
        let name = ComponentName::parse("user-profile").unwrap();
        assert_eq!(name.file_name(), "user_profile");
        assert_eq!(name.type_name(), "UserProfile");
    }

    #[test]
    fn rejects_invalid_component_names() {
        assert!(ComponentName::parse("").is_err());
        assert!(ComponentName::parse("1user").is_err());
        assert!(ComponentName::parse("user/profile").is_err());
        assert!(ComponentName::parse("mod").is_err());
    }

    #[test]
    fn reads_project_type_from_metadata() {
        let manifest = r#"
[package]
name = "demo"

[package.metadata.sword]
project-type = "socketio"
"#;

        assert_eq!(parse_project_type(manifest), Some(ProjectType::Socketio));
        assert_eq!(parse_project_type("[package]\nname = \"demo\"\n"), None);
    }
}
