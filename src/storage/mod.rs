mod config;
mod paths;
mod projects;

pub use config::{AppConfig, KeymapConfig, load_or_create_config, save_config};
pub use paths::StoragePaths;
pub use projects::{SavedProject, default_projects, load_or_create_projects, save_projects};

use std::io;

pub struct Storage {
    pub config: AppConfig,
    pub projects: Vec<SavedProject>,
}

impl Storage {
    pub fn load_or_create() -> io::Result<Self> {
        let paths = StoragePaths::new()?;
        let config = load_or_create_config(&paths.config_file)?;
        let projects = load_or_create_projects(&paths.projects_file)?;

        Ok(Self { config, projects })
    }
}
