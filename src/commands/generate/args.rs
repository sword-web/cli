use super::types::ControllerKind;

#[derive(Debug, clap::Args)]
pub struct GenerateArgs {
    #[command(subcommand)]
    pub command: GenerateCommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum GenerateCommand {
    /// Generate a module and register it in the application.
    Module(ModuleArgs),
    /// Generate a controller inside an existing module.
    Controller(ControllerArgs),
    /// Generate a provider inside a module.
    Provider(ProviderArgs),
}

#[derive(Debug, clap::Args)]
pub struct ModuleArgs {
    pub name: String,
}

#[derive(Debug, clap::Args)]
pub struct ControllerArgs {
    pub name: String,

    /// Controller transport. Defaults to the project type.
    #[arg(long, value_enum)]
    pub kind: Option<ControllerKind>,
}

#[derive(Debug, clap::Args)]
pub struct ProviderArgs {
    pub name: String,

    /// Module that owns the provider.
    #[arg(long, default_value = "shared")]
    pub module: String,
}
