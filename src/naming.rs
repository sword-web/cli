use crate::error::CliError;

use convert_case::{Case, Casing};

const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false",
    "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
    "pub", "ref", "return", "self", "static", "struct", "super", "trait", "true",
    "type", "unsafe", "use", "where", "while", "async", "await", "dyn", "abstract",
    "become", "box", "do", "final", "macro", "override", "priv", "typeof",
    "unsized", "virtual", "yield", "try", "union",
];

/// Validates a user-provided project or component name.
pub fn validate(input: &str) -> Result<&str, CliError> {
    let raw = input.trim();

    if raw.is_empty() {
        return Err(CliError::invalid_name(input, "the name cannot be empty"));
    }

    if !raw
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(CliError::invalid_name(
            input,
            "use only letters, digits, `-` or `_`",
        ));
    }

    if raw.starts_with('-')
        || raw.starts_with('_')
        || raw.ends_with('-')
        || raw.ends_with('_')
    {
        return Err(CliError::invalid_name(
            input,
            "the name cannot start or end with `-` or `_`",
        ));
    }

    if raw.starts_with(|c: char| c.is_ascii_digit()) {
        return Err(CliError::invalid_name(
            input,
            "the name cannot start with a digit",
        ));
    }

    if is_keyword(&raw.to_case(Case::Snake)) {
        return Err(CliError::invalid_name(
            input,
            "the name is a reserved Rust keyword",
        ));
    }

    Ok(raw)
}

pub fn is_keyword(value: &str) -> bool {
    RUST_KEYWORDS.contains(&value)
}
