use super::args::{
    ControllerArgs, GenerateArgs, GenerateCommand, ModuleArgs, ProviderArgs,
};
use super::render::{self, ControllerContext, ModuleContext, ProviderContext};
use super::types::{ComponentName, ControllerKind, Project};
use crate::error::{AppResult, CliError};
use crate::services::{output, wiring, workspace};
use crate::types::ProjectType;

use camino::Utf8PathBuf;

const MODULE_DECLARATIONS: &str = "// sword:module-declarations";
const MODULES: &str = "// sword:modules";
const CONTROLLER_DECLARATIONS: &str = "// sword:controller-declarations";
const CONTROLLERS: &str = "// sword:controllers";
const PROVIDER_DECLARATIONS: &str = "// sword:provider-declarations";
const PROVIDERS: &str = "// sword:providers";

pub struct GenerateAction {
    args: GenerateArgs,
    cwd: Utf8PathBuf,
}

impl GenerateAction {
    pub fn new(args: GenerateArgs, cwd: Utf8PathBuf) -> Self {
        Self { args, cwd }
    }

    pub fn handle(&self) -> AppResult<()> {
        let project = Project::discover(&self.cwd)?;

        output::intro(format!("Sword CLI v{}", env!("CARGO_PKG_VERSION")))?;

        match &self.args.command {
            GenerateCommand::Module(args) => generate_module(&project, args),
            GenerateCommand::Controller(args) => generate_controller(&project, args),
            GenerateCommand::Provider(args) => generate_provider(&project, args),
        }
    }
}

fn generate_module(project: &Project, args: &ModuleArgs) -> AppResult<()> {
    let name = ComponentName::parse(&args.name)?;
    let module_file = project.module_file(&name);

    if module_file.exists() {
        return Err(CliError::component_exists(module_file));
    }

    create_module(project, &name)?;

    output::success(format!("Module created at: {module_file}"))?;

    Ok(())
}

fn generate_controller(project: &Project, args: &ControllerArgs) -> AppResult<()> {
    let name = ComponentName::parse(&args.name)?;
    let module_file = project.module_file(&name);

    if !module_file.is_file() {
        return Err(CliError::missing_module(module_file, name.file_name()));
    }

    let controller_file =
        project.source.join(name.file_name()).join("controller.rs");

    if controller_file.exists() {
        return Err(CliError::component_exists(controller_file));
    }

    wiring::ensure_marker(&module_file, CONTROLLER_DECLARATIONS)?;
    wiring::ensure_marker(&module_file, CONTROLLERS)?;

    let kind = resolve_controller_kind(project.project_type, args.kind)?;
    let controller_type = format!("{}Controller", name.type_name());
    let context = ControllerContext {
        controller_type: controller_type.clone(),
        path: format!("/{}", name.file_name()),
    };

    let contents = match kind {
        ControllerKind::Web => render::controller_web(&context)?,
        ControllerKind::SocketIo => render::controller_socketio(&context)?,
    };

    workspace::write(&controller_file, &contents)?;

    wiring::insert_before_marker(
        &module_file,
        CONTROLLER_DECLARATIONS,
        "mod controller;",
    )?;
    wiring::insert_before_marker(
        &module_file,
        CONTROLLERS,
        &format!("controllers.register::<controller::{controller_type}>();"),
    )?;

    output::success(format!("Controller created at: {controller_file}"))?;

    Ok(())
}

fn generate_provider(project: &Project, args: &ProviderArgs) -> AppResult<()> {
    let provider = ComponentName::parse(&args.name)?;
    let module = ComponentName::parse(&args.module)?;
    let module_file = project.module_file(&module);

    if !module_file.is_file() {
        create_module(project, &module)?;
    }

    let provider_file = project
        .source
        .join(module.file_name())
        .join(format!("{}.rs", provider.file_name()));

    if provider_file.exists() {
        return Err(CliError::component_exists(provider_file));
    }

    wiring::ensure_marker(&module_file, PROVIDER_DECLARATIONS)?;
    wiring::ensure_marker(&module_file, PROVIDERS)?;

    let context = ProviderContext {
        provider_type: provider.type_name().to_string(),
    };

    let contents = render::provider(&context)?;

    workspace::write(&provider_file, &contents)?;

    wiring::insert_before_marker(
        &module_file,
        PROVIDER_DECLARATIONS,
        &format!("mod {};", provider.file_name()),
    )?;
    wiring::insert_before_marker(
        &module_file,
        PROVIDERS,
        &format!(
            "providers.register({}::{}::new());",
            provider.file_name(),
            provider.type_name()
        ),
    )?;

    output::success(format!("Provider created at: {provider_file}"))?;

    Ok(())
}

fn create_module(project: &Project, name: &ComponentName) -> AppResult<Utf8PathBuf> {
    let module_file = project.module_file(name);

    wiring::ensure_marker(&project.main, MODULE_DECLARATIONS)?;
    wiring::ensure_marker(&project.main, MODULES)?;

    let context = ModuleContext {
        module_type: format!("{}Module", name.type_name()),
    };

    let contents = render::module(&context)?;

    workspace::write(&module_file, &contents)?;

    wiring::insert_module_declaration(
        &project.main,
        MODULE_DECLARATIONS,
        name.file_name(),
    )?;
    wiring::insert_before_marker(
        &project.main,
        MODULES,
        &format!(
            ".with_module::<{}::{}Module>()",
            name.file_name(),
            name.type_name()
        ),
    )?;

    Ok(module_file)
}

fn resolve_controller_kind(
    project_type: ProjectType,
    requested: Option<ControllerKind>,
) -> AppResult<ControllerKind> {
    match (project_type, requested) {
        (ProjectType::Grpc, _) => Err(CliError::UnsupportedPhase {
            phase: "controller generation".to_string(),
            project_type: project_type.name().to_string(),
        }),
        (_, Some(ControllerKind::Web)) => Ok(ControllerKind::Web),
        (ProjectType::Web, Some(ControllerKind::SocketIo)) => {
            Err(CliError::UnsupportedPhase {
                phase: "socketio controllers".to_string(),
                project_type: project_type.name().to_string(),
            })
        }
        (ProjectType::Socketio, Some(ControllerKind::SocketIo))
        | (ProjectType::Socketio, None) => Ok(ControllerKind::SocketIo),
        (ProjectType::Web, None) => Ok(ControllerKind::Web),
    }
}
