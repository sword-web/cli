use crate::error::{AppResult, CliError};

use camino::Utf8Path;

/// Fails when `marker` is not present in the file.
pub fn ensure_marker(path: &Utf8Path, marker: &str) -> AppResult<()> {
    let contents = std::fs::read_to_string(path)?;

    if contents.lines().any(|line| line.trim() == marker) {
        return Ok(());
    }

    Err(CliError::missing_marker(path.to_owned(), marker))
}

/// Inserts `mod <name>;` in alphabetical order among the simple module
/// declarations that appear before `marker`.
///
/// Rustfmt reorders contiguous `mod` declarations, so keeping them sorted
/// avoids a formatting diff in generated `main.rs` files.
pub fn insert_module_declaration(
    path: &Utf8Path,
    marker: &str,
    module_name: &str,
) -> AppResult<bool> {
    let contents = std::fs::read_to_string(path)?;
    let line = format!("mod {module_name};");

    if contents.lines().any(|current| current.trim() == line) {
        return Ok(false);
    }

    let lines: Vec<&str> = contents.lines().collect();
    let marker_index = lines
        .iter()
        .position(|current| current.trim() == marker)
        .ok_or_else(|| CliError::missing_marker(path.to_owned(), marker))?;

    let insertion = lines[..marker_index]
        .iter()
        .position(|current| {
            module_declaration_name(current)
                .map(|name| name > module_name)
                .unwrap_or(false)
        })
        .unwrap_or(marker_index);

    let marker_line = lines[marker_index];
    let indentation = " ".repeat(marker_line.len() - marker_line.trim_start().len());
    let rendered = format!("{indentation}{line}");

    let mut output = String::with_capacity(contents.len() + rendered.len() + 1);

    for (index, current) in lines.iter().enumerate() {
        if index == insertion {
            output.push_str(&rendered);
            output.push('\n');
        }

        output.push_str(current);
        output.push('\n');
    }

    std::fs::write(path, output)?;

    Ok(true)
}

fn module_declaration_name(line: &str) -> Option<&str> {
    let line = line.trim();
    let name = line.strip_prefix("mod ")?.strip_suffix(';')?;

    if name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        Some(name)
    } else {
        None
    }
}

/// Inserts `line` before the first line matching `marker`.
///
/// The indentation of the marker is reused, so wiring inside a builder chain
/// keeps the surrounding formatting. Returns `false` when the line is already
/// present, which makes the operation idempotent.
pub fn insert_before_marker(
    path: &Utf8Path,
    marker: &str,
    line: &str,
) -> AppResult<bool> {
    let contents = std::fs::read_to_string(path)?;
    let line = line.trim();

    if contents.lines().any(|current| current.trim() == line) {
        return Ok(false);
    }

    let marker_line = contents
        .lines()
        .find(|current| current.trim() == marker)
        .ok_or_else(|| CliError::missing_marker(path.to_owned(), marker))?;

    let indentation = " ".repeat(marker_line.len() - marker_line.trim_start().len());
    let rendered = format!("{indentation}{line}");

    let mut output = String::with_capacity(contents.len() + rendered.len() + 1);
    let mut inserted = false;

    for current in contents.lines() {
        if !inserted && current.trim() == marker {
            output.push_str(&rendered);
            output.push('\n');
            inserted = true;
        }

        output.push_str(current);
        output.push('\n');
    }

    std::fs::write(path, output)?;

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use camino::Utf8PathBuf;

    fn temp_file(contents: &str) -> (tempfile::TempDir, Utf8PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let path = Utf8PathBuf::from_path_buf(dir.path().join("main.rs")).unwrap();
        std::fs::write(&path, contents).unwrap();
        (dir, path)
    }

    #[test]
    fn inserts_before_marker_using_its_indentation() {
        let (_dir, path) = temp_file("fn main() {\n        // sword:modules\n}\n");

        assert!(
            insert_before_marker(&path, "// sword:modules", ".with_module::<X>()")
                .unwrap()
        );

        let contents = std::fs::read_to_string(&path).unwrap();
        assert!(
            contents
                .contains("        .with_module::<X>()\n        // sword:modules")
        );
    }

    #[test]
    fn is_idempotent() {
        let (_dir, path) = temp_file("// sword:modules\n");

        assert!(insert_before_marker(&path, "// sword:modules", "line").unwrap());
        assert!(!insert_before_marker(&path, "// sword:modules", "line").unwrap());
    }

    #[test]
    fn inserts_module_declarations_in_order() {
        let (_dir, path) =
            temp_file("mod controller;\n// sword:module-declarations\n");

        assert!(
            insert_module_declaration(
                &path,
                "// sword:module-declarations",
                "users"
            )
            .unwrap()
        );
        assert!(
            insert_module_declaration(
                &path,
                "// sword:module-declarations",
                "shared"
            )
            .unwrap()
        );
        assert!(
            insert_module_declaration(&path, "// sword:module-declarations", "app")
                .unwrap()
        );

        let contents = std::fs::read_to_string(&path).unwrap();
        assert!(contents.contains(
            "mod app;\nmod controller;\nmod shared;\nmod users;\n// sword:module-declarations"
        ));
    }

    #[test]
    fn module_declaration_is_idempotent() {
        let (_dir, path) = temp_file("// sword:module-declarations\n");

        assert!(
            insert_module_declaration(
                &path,
                "// sword:module-declarations",
                "users"
            )
            .unwrap()
        );
        assert!(
            !insert_module_declaration(
                &path,
                "// sword:module-declarations",
                "users"
            )
            .unwrap()
        );
    }

    #[test]
    fn fails_when_the_marker_is_missing() {
        let (_dir, path) = temp_file("fn main() {}\n");

        assert!(insert_before_marker(&path, "// sword:modules", "line").is_err());
    }
}
