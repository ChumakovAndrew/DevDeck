use std::{fs, io, path::Path};

#[derive(Clone, Debug)]
pub struct SavedProject {
    pub name: String,
    pub path: String,
    pub ide: Option<String>,
}

pub fn default_projects() -> Vec<SavedProject> {
    vec![
        SavedProject {
            name: "my-rust-app".to_string(),
            path: "~/projects/my-rust-app".to_string(),
            ide: None,
        },
        SavedProject {
            name: "web-frontend".to_string(),
            path: "~/projects/web-frontend".to_string(),
            ide: None,
        },
        SavedProject {
            name: "backend-api".to_string(),
            path: "~/projects/backend-api".to_string(),
            ide: None,
        },
    ]
}

pub fn load_or_create_projects(path: &Path) -> io::Result<Vec<SavedProject>> {
    if !path.exists() {
        let projects = default_projects();
        save_projects_to_path(path, &projects)?;
        return Ok(projects);
    }

    let projects_content = fs::read_to_string(path)?;
    Ok(parse_projects(&projects_content))
}

pub fn save_projects(projects: &[SavedProject]) -> io::Result<()> {
    let paths = crate::storage::StoragePaths::new()?;
    save_projects_to_path(&paths.projects_file, projects)
}

fn save_projects_to_path(path: &Path, projects: &[SavedProject]) -> io::Result<()> {
    fs::write(path, format_projects(projects))
}

fn parse_projects(projects_content: &str) -> Vec<SavedProject> {
    let mut projects = Vec::new();
    let mut current_project: Option<SavedProject> = None;

    for line in projects_content.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line == "[[projects]]" {
            push_if_valid(&mut projects, current_project.take());
            current_project = Some(SavedProject {
                name: String::new(),
                path: String::new(),
                ide: None,
            });
            continue;
        }

        let Some(project) = current_project.as_mut() else {
            continue;
        };
        let Some((key, value)) = parse_key_value(line) else {
            continue;
        };

        match key {
            "name" => project.name = value,
            "path" => project.path = value,
            "ide" => project.ide = Some(value),
            _ => {}
        }
    }

    push_if_valid(&mut projects, current_project);

    if projects.is_empty() {
        default_projects()
    } else {
        projects
    }
}

fn format_projects(projects: &[SavedProject]) -> String {
    let mut output = String::new();

    for project in projects {
        output.push_str("[[projects]]\n");
        output.push_str(&format!(
            "name = \"{}\"\n",
            escape_toml_string(&project.name)
        ));
        output.push_str(&format!(
            "path = \"{}\"\n",
            escape_toml_string(&project.path)
        ));

        if let Some(ide) = &project.ide {
            output.push_str(&format!("ide = \"{}\"\n", escape_toml_string(ide)));
        }

        output.push('\n');
    }

    output
}

fn push_if_valid(projects: &mut Vec<SavedProject>, project: Option<SavedProject>) {
    let Some(project) = project else {
        return;
    };

    if !project.name.is_empty() && !project.path.is_empty() {
        projects.push(project);
    }
}

fn parse_key_value(line: &str) -> Option<(&str, String)> {
    let (key, value) = line.split_once('=')?;
    Some((key.trim(), unquote(value.trim())))
}

fn unquote(value: &str) -> String {
    value
        .trim_matches('"')
        .replace("\\\"", "\"")
        .replace("\\\\", "\\")
}

fn escape_toml_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
