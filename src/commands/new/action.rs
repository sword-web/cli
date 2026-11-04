use super::NewArgs;
use super::render::RenderContext;
use super::types::*;
use crate::error::{AppResult, CliError};
use crate::services::{output, prompt, template, workspace};
use crate::types::{GenerationPlan, ProjectType};

use camino::{Utf8Path, Utf8PathBuf};

pub struct NewAction {
    args: NewArgs,
    cwd: Utf8PathBuf,
}

impl NewAction {
    pub fn new(args: NewArgs, cwd: Utf8PathBuf) -> Self {
        Self { args, cwd }
    }

    pub fn handle(&self) -> AppResult<()> {
        output::intro(format!("Sword CLI v{}", env!("CARGO_PKG_VERSION")))?;

        let options = self.resolve_options()?;

        self.scaffold(&options)
    }

    fn resolve_options(&self) -> AppResult<NewProjectOptions> {
        let name = self.resolve_name()?;
        let project_type = self.resolve_project_type()?;
        let features = self.resolve_features(project_type)?;

        Ok(NewProjectOptions {
            name,
            project_type,
            features,
            git: self.resolve_git(),
        })
    }

    fn resolve_name(&self) -> AppResult<ProjectName> {
        if self.args.name == "." {
            Ok(ProjectName::from_cwd(&self.cwd))
        } else {
            ProjectName::parse(&self.args.name)
        }
    }

    fn resolve_project_type(&self) -> AppResult<ProjectType> {
        if let Some(project_type) = self.args.project_type {
            return Ok(project_type);
        }

        if self.args.yes {
            return Ok(ProjectType::Web);
        }

        let selection = prompt::select(
            "Pick an application kind",
            &[
                ("web", "Web", "HTTP REST API with middleware & routing"),
                ("socketio", "Socket.io", "Real-time WebSocket channels"),
                ("grpc", "gRPC", "gRPC API with contracts & streaming"),
            ],
        )
        .ok_or_else(|| CliError::invalid_project(self.cwd.clone()))?;

        Ok(ProjectType::parse(&selection).unwrap_or(ProjectType::Web))
    }

    fn resolve_features(&self, project_type: ProjectType) -> AppResult<FeatureSet> {
        if !self.args.features.is_empty() {
            let features = FeatureSet::from_features(&self.args.features);
            features.validate(project_type)?;
            return Ok(features);
        }

        if self.args.yes {
            return Ok(FeatureSet::default());
        }

        let features = match project_type {
            ProjectType::Web => FeatureSet {
                multipart: prompt::confirm("Enable multipart? (File uploads)"),
                validator: prompt::confirm("Enable validator? (DTO validation)"),
                grpc_reflection: false,
            },
            ProjectType::Socketio => FeatureSet {
                multipart: false,
                validator: prompt::confirm("Enable validator? (DTO validation)"),
                grpc_reflection: false,
            },
            ProjectType::Grpc => FeatureSet {
                multipart: false,
                validator: false,
                grpc_reflection: prompt::confirm(
                    "Enable grpc-reflection? (Server reflection for tooling)",
                ),
            },
        };

        Ok(features)
    }

    fn resolve_git(&self) -> GitConfig {
        if self.args.no_git {
            return GitConfig::default();
        }

        if let Some(remote) = &self.args.remote {
            return GitConfig {
                init: true,
                remote: Some(remote.clone()),
            };
        }

        if self.args.git {
            return GitConfig {
                init: true,
                remote: None,
            };
        }

        if self.args.yes {
            return GitConfig::default();
        }

        match prompt::select(
            "Configure git",
            &[
                ("skip", "Skip", "No git initialization"),
                ("init", "Initialize", "Create new repository"),
                ("link", "Link", "Connect to existing remote"),
            ],
        )
        .as_deref()
        {
            Some("init") => GitConfig {
                init: true,
                remote: None,
            },
            Some("link") => {
                let url = prompt::input(
                    "Enter repository URL",
                    "https://github.com/user/repo.git",
                )
                .unwrap_or_default();

                GitConfig {
                    init: true,
                    remote: Some(url),
                }
            }
            _ => GitConfig::default(),
        }
    }

    fn scaffold(&self, options: &NewProjectOptions) -> AppResult<()> {
        let in_place = self.args.name == ".";
        let root = if in_place {
            self.cwd.clone()
        } else {
            self.cwd.join(options.name.directory())
        };

        if in_place {
            ensure_empty_directory(&root)?;
        } else if root.exists() {
            return Err(CliError::project_exists(root));
        }

        let context = RenderContext::from(options);
        let files = template::render_files(options.project_type, &context)?;

        let plan = GenerationPlan {
            root: root.clone(),
            files,
        };

        let touched = workspace::apply(&plan)?;

        for path in &touched {
            output::step(format!("Writing {}", path.as_str()))?;
        }

        if options.git.init {
            workspace::init_git(&root, options.git.remote.as_deref())?;
        }

        output::success(format!("Project created at: {root}"))?;

        Ok(())
    }
}

/// `sword new .` must not overwrite an existing project. Hidden `.git` is
/// allowed so the command can run inside an already initialized repository.
fn ensure_empty_directory(path: &Utf8Path) -> AppResult<()> {
    if !path.exists() {
        return Ok(());
    }

    let mut entries = std::fs::read_dir(path)?;

    let occupied = entries.any(|entry| match entry {
        Ok(entry) => entry.file_name() != ".git",
        Err(_) => true,
    });

    if occupied {
        return Err(CliError::project_exists(path.to_owned()));
    }

    Ok(())
}
