use crate::error::AppResult;
use crate::services::template;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ModuleContext {
    pub module_type: String,
}

#[derive(Debug, Serialize)]
pub struct ControllerContext {
    pub controller_type: String,
    pub path: String,
}

#[derive(Debug, Serialize)]
pub struct ProviderContext {
    pub provider_type: String,
}

pub fn module(context: &ModuleContext) -> AppResult<String> {
    render(
        include_str!("../../../templates/generate/module.rs.liquid"),
        context,
    )
}

pub fn controller_web(context: &ControllerContext) -> AppResult<String> {
    render(
        include_str!("../../../templates/generate/controller_web.rs.liquid"),
        context,
    )
}

pub fn controller_socketio(context: &ControllerContext) -> AppResult<String> {
    render(
        include_str!("../../../templates/generate/controller_socketio.rs.liquid"),
        context,
    )
}

pub fn provider(context: &ProviderContext) -> AppResult<String> {
    render(
        include_str!("../../../templates/generate/provider.rs.liquid"),
        context,
    )
}

fn render<S: Serialize>(source: &str, context: &S) -> AppResult<String> {
    template::render_source(source, context)
}
