use std::fmt::Display;
use std::io::Result;

pub fn intro(title: impl Display) -> Result<()> {
    cliclack::intro(title)
}

pub fn step(message: impl Display) -> Result<()> {
    cliclack::log::step(message)
}

pub fn success(message: impl Display) -> Result<()> {
    cliclack::log::success(message)
}

pub fn error(message: impl Display) -> Result<()> {
    cliclack::log::error(message)
}

pub fn outro_cancel(message: impl Display) -> Result<()> {
    cliclack::outro_cancel(message)
}
