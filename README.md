# Sword CLI

`sword-cli` provides a command line interface for scaffolding Sword projects.

- `sword`
- `swd` (short alias)

Both commands expose the same CLI surface.

## Install

```bash
cargo install --path .
```

Until the framework publishes a release with the current API, generated
projects pin `sword` to the revision declared in `src/types.rs`.

For gRPC projects, `protoc` must be installed because `build.rs` compiles the
`.proto` files during the build.

Generated projects are expected to pass `cargo fmt --check` and
`cargo clippy --all-targets -- -D warnings`.

## `sword new`

Creates a project, its initial configuration and its entry point.

```bash
sword new my-app
sword new my-app --type web --yes --no-git
sword new . --type socketio --yes
```

Options:

| Option | Description |
|---|---|
| `--type web\|socketio\|grpc` | Application kind. Defaults to `web` with `--yes`. |
| `--feature multipart\|validator\|grpc-reflection` | Enables an optional framework feature. Repeatable. |
| `--git` / `--no-git` | Initializes or skips the git repository. |
| `--remote <URL>` | Initializes git and adds `origin`. |
| `-y`, `--yes` | Uses defaults without prompting. |

The generated project contains the framework metadata that `generate` uses:

```toml
[package.metadata.sword]
project-type = "web"
```

## `sword generate`

Adds components inside a project and wires them automatically.

```bash
sword generate module users
sword generate controller users
sword generate provider database
```

- `module` creates `src/<module>/mod.rs` and registers it in `src/main.rs`.
- `controller` creates `src/<module>/controller.rs` and registers it in the module.
  It supports web and Socket.IO controllers.
- `provider` creates an `#[injectable(provider)]` inside a module. It defaults to
  the `shared` module and creates it when needed.

Existing files are never overwritten. The generator relies on the
`// sword:*` markers in `src/main.rs` and in the module files; if the marker is
missing, the command fails instead of editing the file blindly.
