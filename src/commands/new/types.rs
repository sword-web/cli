use crate::types::ProjectType;
use convert_case::{Case, Casing};

/// Validated project name, normalized to a snake_case crate/module path.
#[derive(Debug, Clone)]
pub struct ProjectName(String);

impl ProjectName {
    /// Normalizes a user-provided name (e.g. `My App` -> `my_app`).
    pub fn parse(input: &str) -> Self {
        let value = input
            .to_case(Case::Snake)
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect();

        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Optional features the generated project is configured with.
#[derive(Debug, Clone, Copy, Default)]
pub struct FeatureSet {
    pub multipart: bool,
    pub validator: bool,
    pub grpc_reflection: bool,
}

/// Database the generated project connects to.
#[derive(Debug, Clone, Default)]
pub enum Database {
    #[default]
    None,
    Redis,
    Mongo,
    Sqlx(SqlxFlavor),
}

impl Database {
    /// Maps the choice to the `(db, db_flavor)` pair expected by the templates.
    pub fn as_template(&self) -> Option<(&'static str, Option<&'static str>)> {
        match self {
            Database::None => None,
            Database::Redis => Some(("redis", None)),
            Database::Mongo => Some(("mongodb", None)),
            Database::Sqlx(flavor) => Some(("sqlx", Some(flavor.as_str()))),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SqlxFlavor {
    Postgres,
    Mysql,
    Sqlite,
}

impl SqlxFlavor {
    pub fn as_str(&self) -> &'static str {
        match self {
            SqlxFlavor::Postgres => "postgres",
            SqlxFlavor::Mysql => "mysql",
            SqlxFlavor::Sqlite => "sqlite",
        }
    }
}

/// Git setup for the generated project.
#[derive(Debug, Clone, Default)]
pub struct GitConfig {
    pub init: bool,
    pub remote: Option<String>,
}

/// Fully specified options for a `new` command run
#[derive(Debug, Clone)]
pub struct NewProjectOptions {
    pub name: ProjectName,
    pub project_type: ProjectType,
    pub features: FeatureSet,
    pub database: Database,
    pub addons: Vec<String>,
    pub git: GitConfig,
}

impl NewProjectOptions {
    /// Non-interactive defaults used when `--yes` is passed: no optional
    /// features, no database, no addons and no git setup.
    pub fn defaults(name: ProjectName, project_type: ProjectType) -> Self {
        Self {
            name,
            project_type,
            features: FeatureSet::default(),
            database: Database::None,
            addons: Vec::new(),
            git: GitConfig::default(),
        }
    }
}
