use super::NewArgs;
use super::render::RenderContext;
use super::types::*;
use crate::error::{AppResult, CliError};
use crate::services::{output, prompt, template, workspace};
use crate::types::{GenerationPlan, ProjectType};

use camino::Utf8PathBuf;

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

        let options = if self.args.yes {
            let name = self.resolve_name();
            let project_type = self.args.project_type.unwrap_or(ProjectType::Web);

            NewProjectOptions::defaults(name, project_type)
        } else {
            self.resolve_options()?
        };

        self.scaffold(&options)
    }

    fn resolve_options(&self) -> AppResult<NewProjectOptions> {
        let name = self.resolve_name();

        let project_type = match self.args.project_type {
            Some(project_type) => project_type,
            None => {
                let selection = prompt::select(
                    "Pick an application kind",
                    &[
                        ("web", "Web", "HTTP REST API with middleware & routing"),
                        ("socketio", "Socket.io", "Real-time WebSocket channels"),
                        ("grpc", "gRPC", "gRPC API with contracts & streaming"),
                    ],
                )
                .ok_or_else(|| CliError::invalid_project(self.cwd.clone()))?;

                match selection.as_str() {
                    "socketio" => ProjectType::Socketio,
                    "grpc" => ProjectType::Grpc,
                    _ => ProjectType::Web,
                }
            }
        };

        let mut multipart = false;
        let mut validator = false;
        let mut grpc_reflection = false;

        match project_type {
            ProjectType::Web => {
                multipart = prompt::confirm("Enable multipart? (File uploads)");
                validator = prompt::confirm("Enable validator? (DTO validation)");
            }
            ProjectType::Socketio => {
                validator = prompt::confirm("Enable validator? (DTO validation)");
            }
            ProjectType::Grpc => {
                grpc_reflection = prompt::confirm(
                    "Enable grpc-reflection? (Server reflection for tooling)",
                );
            }
        }

        let features = FeatureSet {
            multipart,
            validator,
            grpc_reflection,
        };

        Ok(NewProjectOptions {
            name,
            project_type,
            features,
            database: self.resolve_database(),
            addons: self.resolve_addons(),
            git: self.resolve_git(),
        })
    }

    fn resolve_name(&self) -> ProjectName {
        if self.args.name != "." {
            return ProjectName::parse(&self.args.name);
        }

        let name = self
            .cwd
            .file_name()
            .map(str::to_string)
            .unwrap_or_else(|| "project".to_string());

        ProjectName::parse(&name)
    }

    fn resolve_database(&self) -> Database {
        let choice = prompt::select(
            "Select database",
            &[
                ("none", "None", "No database, stateless API"),
                ("redis", "Redis", "In-memory cache & session store"),
                ("mongodb", "MongoDB", "NoSQL document database"),
                ("sqlx", "SQLx", "SQL ORM with compile-time checks"),
            ],
        )
        .unwrap_or_else(|| "none".to_string());

        match choice.as_str() {
            "redis" => Database::Redis,
            "mongodb" => Database::Mongo,
            "sqlx" => {
                let flavor = prompt::select(
                    "Select SQLx database",
                    &[
                        ("postgres", "PostgreSQL", "Feature-rich RDBMS"),
                        ("mysql", "MySQL", "Popular relational database"),
                        ("sqlite", "SQLite", "Embedded database"),
                    ],
                )
                .unwrap_or_else(|| "postgres".to_string());

                Database::Sqlx(match flavor.as_str() {
                    "mysql" => SqlxFlavor::Mysql,
                    "sqlite" => SqlxFlavor::Sqlite,
                    _ => SqlxFlavor::Postgres,
                })
            }
            _ => Database::None,
        }
    }

    fn resolve_addons(&self) -> Vec<String> {
        prompt::multi_select(
            "Select additional libraries",
            &[
                ("serde_json", "serde_json", "JSON serialization"),
                ("uuid", "uuid", "UUID generation (v4)"),
                ("chrono", "chrono", "Date & time utilities"),
                ("jsonwebtoken", "jsonwebtoken", "JWT authentication"),
                ("argon2", "argon2", "Password hashing"),
                ("ldap3", "ldap3", "LDAP client"),
                ("bon", "bon", "Macro based Builder patterns"),
                ("reqwest", "reqwest", "HTTP client"),
            ],
        )
    }

    fn resolve_git(&self) -> GitConfig {
        if self.args.no_git {
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
        let root = self.cwd.join(options.name.as_str());

        if root.exists() {
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
