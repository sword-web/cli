use std::fs;
use std::path::Path;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn sword() -> Command {
    Command::cargo_bin("sword").unwrap()
}

fn new_project(dir: &TempDir, name: &str, project_type: &str) {
    let mut command = sword();
    command
        .current_dir(dir.path())
        .args(["new", name, "--type", project_type, "--yes", "--no-git"])
        .assert()
        .success();
}

fn read(path: impl AsRef<Path>) -> String {
    fs::read_to_string(path).unwrap()
}

#[test]
fn new_generates_a_web_project() {
    let dir = TempDir::new().unwrap();
    new_project(&dir, "demo", "web");

    let root = dir.path().join("demo");

    assert!(root.join("Cargo.toml").is_file());
    assert!(root.join("config/config.toml").is_file());
    assert!(root.join("src/main.rs").is_file());
    assert!(root.join("src/controller.rs").is_file());

    let manifest = read(root.join("Cargo.toml"));
    assert!(manifest.contains("rev = \"47b6bc2beefad2ad2bc944202ca595fa16db4aea\""));
    assert!(manifest.contains("project-type = \"web\""));

    let main = read(root.join("src/main.rs"));
    assert!(main.contains("async fn main() -> Result<(), Error>"));
    assert!(main.contains("// sword:modules"));
}

#[test]
fn new_in_place_scaffolds_the_current_directory() {
    let dir = TempDir::new().unwrap();
    let mut command = sword();
    command
        .current_dir(dir.path())
        .args(["new", ".", "--type", "web", "--yes", "--no-git"])
        .assert()
        .success();

    assert!(dir.path().join("Cargo.toml").is_file());
    assert!(dir.path().join("src/main.rs").is_file());

    let name = dir.path().file_name().unwrap();
    assert!(!dir.path().join(name).exists());
}

#[test]
fn new_keeps_the_directory_name() {
    let dir = TempDir::new().unwrap();
    new_project(&dir, "my-app", "web");

    let root = dir.path().join("my-app");
    assert!(root.is_dir());
    assert!(read(root.join("Cargo.toml")).contains("name = \"my-app\""));
}

#[test]
fn generate_module_and_controller_wires_the_project() {
    let dir = TempDir::new().unwrap();
    new_project(&dir, "demo", "web");

    let mut module = sword();
    module
        .current_dir(dir.path().join("demo"))
        .args(["generate", "module", "users"])
        .assert()
        .success();

    let module_file = dir.path().join("demo/src/users/mod.rs");
    assert!(module_file.is_file());

    let main = read(dir.path().join("demo/src/main.rs"));
    assert!(main.contains("mod users;"));
    assert!(main.contains(".with_module::<users::UsersModule>()"));

    let mut controller = sword();
    controller
        .current_dir(dir.path().join("demo"))
        .args(["generate", "controller", "users"])
        .assert()
        .success();

    let controller_file = dir.path().join("demo/src/users/controller.rs");
    assert!(controller_file.is_file());

    let module = read(module_file);
    assert!(module.contains("mod controller;"));
    assert!(
        module.contains("controllers.register::<controller::UsersController>();")
    );
}

#[test]
fn generate_keeps_module_declarations_sorted() {
    let dir = TempDir::new().unwrap();
    new_project(&dir, "demo", "web");

    let mut users = sword();
    users
        .current_dir(dir.path().join("demo"))
        .args(["generate", "module", "users"])
        .assert()
        .success();

    let mut shared = sword();
    shared
        .current_dir(dir.path().join("demo"))
        .args(["generate", "provider", "database"])
        .assert()
        .success();

    let mut app = sword();
    app.current_dir(dir.path().join("demo"))
        .args(["generate", "module", "app"])
        .assert()
        .success();

    let main = read(dir.path().join("demo/src/main.rs"));
    let app = main.find("mod app;").unwrap();
    let controller = main.find("mod controller;").unwrap();
    let shared = main.find("mod shared;").unwrap();
    let users = main.find("mod users;").unwrap();

    assert!(app < controller);
    assert!(controller < shared);
    assert!(shared < users);
}

#[test]
fn generate_provider_creates_and_wires_the_shared_module() {
    let dir = TempDir::new().unwrap();
    new_project(&dir, "demo", "web");

    let mut command = sword();
    command
        .current_dir(dir.path().join("demo"))
        .args(["generate", "provider", "database"])
        .assert()
        .success();

    let provider = dir.path().join("demo/src/shared/database.rs");
    assert!(provider.is_file());
    assert!(read(provider).contains("#[injectable(provider)]"));

    let module = read(dir.path().join("demo/src/shared/mod.rs"));
    assert!(module.contains("mod database;"));
    assert!(module.contains("providers.register(database::Database::new());"));

    let main = read(dir.path().join("demo/src/main.rs"));
    assert!(main.contains(".with_module::<shared::SharedModule>()"));
}

#[test]
fn generate_requires_a_sword_project() {
    let dir = TempDir::new().unwrap();
    let mut command = sword();
    command
        .current_dir(dir.path())
        .args(["generate", "module", "users"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not a Sword project"));
}

#[test]
fn generate_controller_requires_its_module() {
    let dir = TempDir::new().unwrap();
    new_project(&dir, "demo", "web");

    let mut command = sword();
    command
        .current_dir(dir.path().join("demo"))
        .args(["generate", "controller", "users"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("module not found"));
}

#[test]
fn generate_does_not_overwrite_components() {
    let dir = TempDir::new().unwrap();
    new_project(&dir, "demo", "web");

    let mut first = sword();
    first
        .current_dir(dir.path().join("demo"))
        .args(["generate", "module", "users"])
        .assert()
        .success();

    let mut second = sword();
    second
        .current_dir(dir.path().join("demo"))
        .args(["generate", "module", "users"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn new_rejects_invalid_features_and_names() {
    let dir = TempDir::new().unwrap();

    let mut feature = sword();
    feature
        .current_dir(dir.path())
        .args([
            "new",
            "demo",
            "--type",
            "grpc",
            "--feature",
            "multipart",
            "--yes",
            "--no-git",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unsupported feature"));

    let mut name = sword();
    name.current_dir(dir.path())
        .args(["new", "9app", "--yes", "--no-git"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot start with a digit"));
}
