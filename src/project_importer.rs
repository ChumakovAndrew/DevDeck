use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use crate::storage::SavedProject;

pub fn projects_from_directory(directory_path: &str) -> io::Result<Vec<SavedProject>> {
    let directory_path = expand_home(directory_path);
    let mut projects = Vec::new();

    for entry in fs::read_dir(directory_path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if !file_type.is_dir() {
            continue;
        }

        let path = entry.path();
        let Some(name) = directory_name(&path) else {
            continue;
        };

        projects.push(SavedProject {
            name,
            path: path.to_string_lossy().into_owned(),
            ide: None,
        });
    }

    projects.sort_by(|left, right| left.name.cmp(&right.name));

    Ok(projects)
}

fn directory_name(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(str::to_string)
}

fn expand_home(path: &str) -> PathBuf {
    if path == "~" {
        return home_dir().unwrap_or_else(|| PathBuf::from(path));
    }

    if let Some(stripped_path) = path.strip_prefix("~/") {
        if let Some(home_dir) = home_dir() {
            return home_dir.join(stripped_path);
        }
    }

    PathBuf::from(path)
}

fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}
