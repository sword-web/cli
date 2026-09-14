use crate::error::CliError;
use crate::types::{PlannedFile, ProjectType};
use liquid::{ParserBuilder, to_object};
use serde::Serialize;

/// A file to generate: destination path + embedded template source.
#[derive(Debug, Clone, Copy)]
struct TemplateFile {
    path: &'static str,
    source: &'static str,
}

const COMMON_TEMPLATES: &[TemplateFile] = &[
    TemplateFile {
        path: "Cargo.toml",
        source: include_str!("../../templates/common/Cargo.toml.liquid"),
    },
    TemplateFile {
        path: "config/config.toml",
        source: include_str!("../../templates/common/config.toml.liquid"),
    },
    TemplateFile {
        path: ".gitignore",
        source: include_str!("../../templates/common/gitignore.liquid"),
    },
];

/// Files generated for a project of the given type, relative to its root.
fn files_for(project_type: ProjectType) -> Vec<TemplateFile> {
    let mut files = COMMON_TEMPLATES.to_vec();

    let (main, controller) = match project_type {
        ProjectType::Web => (
            include_str!("../../templates/web/main.rs.liquid"),
            include_str!("../../templates/web/controller.rs.liquid"),
        ),
        ProjectType::Socketio => (
            include_str!("../../templates/socketio/main.rs.liquid"),
            include_str!("../../templates/socketio/controller.rs.liquid"),
        ),
        ProjectType::Grpc => (
            include_str!("../../templates/grpc/main.rs.liquid"),
            include_str!("../../templates/grpc/controller.rs.liquid"),
        ),
    };

    files.push(TemplateFile {
        path: "src/main.rs",
        source: main,
    });
    files.push(TemplateFile {
        path: "src/controller.rs",
        source: controller,
    });

    if project_type == ProjectType::Grpc {
        files.push(TemplateFile {
            path: "build.rs",
            source: include_str!("../../templates/grpc/build.rs.liquid"),
        });
        files.push(TemplateFile {
            path: "config/proto/hello.proto",
            source: include_str!("../../templates/grpc/proto/hello.proto.liquid"),
        });
    }

    files
}

/// Renders every file for `project_type` with the given template context.
pub fn render_files<S: Serialize>(
    project_type: ProjectType,
    context: &S,
) -> Result<Vec<PlannedFile>, CliError> {
    let parser = ParserBuilder::with_stdlib()
        .build()
        .map_err(|err| CliError::render("parse", err.to_string()))?;

    let mut planned = Vec::new();

    for template in files_for(project_type) {
        let contents = render(&parser, template.source, context)?;
        planned.push(PlannedFile {
            path: template.path.to_string(),
            contents,
        });
    }

    Ok(planned)
}

fn render<S: Serialize>(
    parser: &liquid::Parser,
    source: &str,
    context: &S,
) -> Result<String, CliError> {
    let template = parser
        .parse(source)
        .map_err(|err| CliError::render("parse", err.to_string()))?;

    let globals = to_object(context)
        .map_err(|err| CliError::render("serialize context", err.to_string()))?;

    template
        .render(&globals)
        .map_err(|err| CliError::render("render", err.to_string()))
}
