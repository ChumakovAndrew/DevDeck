use std::{
    env, io,
    path::{Path, PathBuf},
};

pub struct StoragePaths {
    pub config_file: PathBuf,
    pub projects_file: PathBuf,
}

impl StoragePaths {
    pub fn new() -> io::Result<Self> {
        let config_dir = user_config_dir()?.join("cli_app");
        let data_dir = user_data_dir()?.join("cli_app");

        create_dir(&config_dir)?;
        create_dir(&data_dir)?;

        Ok(Self {
            config_file: config_dir.join("config.toml"),
            projects_file: data_dir.join("projects.toml"),
        })
    }
}

fn user_config_dir() -> io::Result<PathBuf> {
    if let Some(config_home) = env::var_os("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(config_home));
    }

    Ok(home_dir()?.join(".config"))
}

fn user_data_dir() -> io::Result<PathBuf> {
    if let Some(data_home) = env::var_os("XDG_DATA_HOME") {
        return Ok(PathBuf::from(data_home));
    }

    Ok(home_dir()?.join(".local").join("share"))
}

fn home_dir() -> io::Result<PathBuf> {
    env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "failed to resolve HOME directory"))
}

fn create_dir(path: &Path) -> io::Result<()> {
    std::fs::create_dir_all(path)
}
