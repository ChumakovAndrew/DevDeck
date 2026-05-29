use std::{env, io, path::PathBuf, process::Command};

pub fn launch_project(ide: &str, project_path: &str) -> io::Result<()> {
    let executable = ide_executable(ide);
    let expanded_path = expand_home(project_path);

    Command::new(executable).arg(expanded_path).spawn()?;

    Ok(())
}

fn ide_executable(ide: &str) -> String {
    match ide.to_ascii_lowercase().as_str() {
        "vs code" | "vscode" | "code" => "code".to_string(),
        "zed" => "zed".to_string(),
        _ => ide.to_string(),
    }
}

fn expand_home(project_path: &str) -> PathBuf {
    if project_path == "~" {
        return home_dir().unwrap_or_else(|| PathBuf::from(project_path));
    }

    if let Some(stripped_path) = project_path.strip_prefix("~/") {
        if let Some(home_dir) = home_dir() {
            return home_dir.join(stripped_path);
        }
    }

    PathBuf::from(project_path)
}

fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}
