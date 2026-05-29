use std::{fs, io, path::Path};

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub selected_ide: String,
    pub key_layout: String,
    pub keymap: KeymapConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            selected_ide: "VS Code".to_string(),
            key_layout: "default".to_string(),
            keymap: KeymapConfig::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeymapConfig {
    pub quit: String,
    pub open_projects: String,
    pub open_settings: String,
    pub next_project: String,
    pub previous_project: String,
    pub confirm: String,
}

impl Default for KeymapConfig {
    fn default() -> Self {
        Self::default_layout()
    }
}

impl KeymapConfig {
    pub fn default_layout() -> Self {
        Self {
            quit: "q".to_string(),
            open_projects: "left".to_string(),
            open_settings: "right".to_string(),
            next_project: "down".to_string(),
            previous_project: "up".to_string(),
            confirm: "enter".to_string(),
        }
    }

    pub fn vim_layout() -> Self {
        Self {
            quit: "q".to_string(),
            open_projects: "h".to_string(),
            open_settings: "l".to_string(),
            next_project: "j".to_string(),
            previous_project: "k".to_string(),
            confirm: "enter".to_string(),
        }
    }
}

pub fn load_or_create_config(path: &Path) -> io::Result<AppConfig> {
    if !path.exists() {
        let config = AppConfig::default();
        save_config_to_path(path, &config)?;
        return Ok(config);
    }

    let config_content = fs::read_to_string(path)?;
    Ok(parse_config(&config_content))
}

pub fn save_config(config: &AppConfig) -> io::Result<()> {
    let paths = crate::storage::StoragePaths::new()?;
    save_config_to_path(&paths.config_file, config)
}

fn save_config_to_path(path: &Path, config: &AppConfig) -> io::Result<()> {
    fs::write(path, format_config(config))
}

fn parse_config(config_content: &str) -> AppConfig {
    let mut config = AppConfig::default();
    let mut in_keymap = false;

    for line in config_content.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line == "[keymap]" {
            in_keymap = true;
            continue;
        }

        if line.starts_with('[') {
            in_keymap = false;
            continue;
        }

        let Some((key, value)) = parse_key_value(line) else {
            continue;
        };

        if in_keymap {
            match key {
                "quit" => config.keymap.quit = value,
                "open_projects" => config.keymap.open_projects = value,
                "open_settings" => config.keymap.open_settings = value,
                "next_project" => config.keymap.next_project = value,
                "previous_project" => config.keymap.previous_project = value,
                "confirm" => config.keymap.confirm = value,
                _ => {}
            }
        } else if key == "selected_ide" {
            config.selected_ide = value;
        } else if key == "key_layout" {
            config.key_layout = value;
        }
    }

    config
}

fn format_config(config: &AppConfig) -> String {
    format!(
        "selected_ide = \"{}\"\n\
        key_layout = \"{}\"\n\n\
        [keymap]\n\
        quit = \"{}\"\n\
        open_projects = \"{}\"\n\
        open_settings = \"{}\"\n\
        next_project = \"{}\"\n\
        previous_project = \"{}\"\n\
        confirm = \"{}\"\n",
        escape_toml_string(&config.selected_ide),
        escape_toml_string(&config.key_layout),
        escape_toml_string(&config.keymap.quit),
        escape_toml_string(&config.keymap.open_projects),
        escape_toml_string(&config.keymap.open_settings),
        escape_toml_string(&config.keymap.next_project),
        escape_toml_string(&config.keymap.previous_project),
        escape_toml_string(&config.keymap.confirm),
    )
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
