# CLI App

A small terminal project manager built with Rust, Ratatui, and Crossterm.

The app keeps a saved list of local projects, lets you launch the selected project in your editor, and stores user preferences such as IDE and key layout in local config files.

## Features

- Terminal UI with separate **Projects** and **Settings** screens
- Launch the selected project in VS Code, Zed, or a custom command
- Add a project manually by name and path
- Import all projects from a directory
- Delete saved projects
- Switch between default and Vim-like navigation
- Persist settings and projects between runs
- No extra runtime services or database

## Screens

### Projects

The Projects screen shows the saved project list.

Press `Enter` on a selected project to launch it in the configured IDE.

### Settings

The Settings screen lets you:

- Change the default IDE
- Switch key layout
- Add one project manually
- Import all projects from a directory
- Delete the currently selected project

## Key Layouts

### Default

| Action | Key |
| --- | --- |
| Open Projects screen | `Left` |
| Open Settings screen | `Right` |
| Move up | `Up` |
| Move down | `Down` |
| Confirm / launch | `Enter` |
| Quit | `q` |

### Vim

| Action | Key |
| --- | --- |
| Open Projects screen | `h` |
| Open Settings screen | `l` |
| Move up | `k` |
| Move down | `j` |
| Confirm / launch | `Enter` |
| Quit | `q` |

During text input:

- `Enter` continues or saves
- `Backspace` deletes a character
- `Esc` cancels input

## Installation

Clone the repository:

```bash
git clone <repo-url>
cd cli_app
```

Run the app:

```bash
cargo run
```

Build a release binary:

```bash
cargo build --release
```

The compiled binary will be available at:

```text
target/release/cli_app
```

## IDE Launching

The app maps IDE names to executable commands:

| IDE value | Command |
| --- | --- |
| `VS Code` | `code` |
| `vscode` | `code` |
| `code` | `code` |
| `Zed` | `zed` |
| custom value | used as the executable name |

Project paths starting with `~/` are expanded using `$HOME`.

Make sure your editor command is available in `PATH`.

## Storage

The app creates local user files automatically.

```text
~/.config/cli_app/config.toml
~/.local/share/cli_app/projects.toml
```

If `XDG_CONFIG_HOME` or `XDG_DATA_HOME` are set, those directories are used instead.

### `config.toml`

```toml
selected_ide = "VS Code"
key_layout = "default"

[keymap]
quit = "q"
open_projects = "left"
open_settings = "right"
next_project = "down"
previous_project = "up"
confirm = "enter"
```

### `projects.toml`

```toml
[[projects]]
name = "my-rust-app"
path = "~/projects/my-rust-app"

[[projects]]
name = "backend-api"
path = "~/projects/backend-api"
ide = "Zed"
```

If a project has its own `ide`, that value overrides the global `selected_ide`.

## Project Structure

```text
src/
  app.rs                Application state and actions
  keymap.rs             Key-to-action mapping
  main.rs               Terminal setup and event loop
  project_importer.rs   Directory import logic
  project_launcher.rs   IDE launch logic
  screens/              Screen renderers
  storage/              Config and project persistence
  ui/                   Shared layout and navigation rendering
```

## Tech Stack

- Rust 2024 edition
- Ratatui
- Crossterm

## Status

This is an early-stage CLI project manager. The current focus is a fast local workflow for saving, browsing, importing, and launching projects from the terminal.
