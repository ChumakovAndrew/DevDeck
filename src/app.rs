use crate::storage::{AppConfig, SavedProject, default_projects};

const SETTINGS_OPTIONS_COUNT: usize = 5;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ActiveScreen {
    Projects,
    Settings,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum InputMode {
    None,
    AddProjectName,
    AddProjectPath,
    AddProjectsDirectory,
}

pub enum AppCommand {
    None,
    SaveConfig,
    SaveProjects,
    ImportProjectsFromDirectory(String),
}

pub struct App {
    pub active_screen: ActiveScreen,
    pub config: AppConfig,
    pub projects: Vec<SavedProject>,
    pub selected_project_idx: usize,
    pub selected_settings_idx: usize,
    pub input_mode: InputMode,
    pub pending_project_name: String,
    pub pending_project_path: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            active_screen: ActiveScreen::Projects,
            config: AppConfig::default(),
            projects: default_projects(),
            selected_project_idx: 0,
            selected_settings_idx: 0,
            input_mode: InputMode::None,
            pending_project_name: String::new(),
            pending_project_path: String::new(),
        }
    }
}

impl App {
    pub fn from_storage(config: AppConfig, projects: Vec<SavedProject>) -> Self {
        Self {
            active_screen: ActiveScreen::Projects,
            config,
            projects,
            selected_project_idx: 0,
            selected_settings_idx: 0,
            input_mode: InputMode::None,
            pending_project_name: String::new(),
            pending_project_path: String::new(),
        }
    }

    pub fn set_screen(&mut self, screen: ActiveScreen) {
        if self.is_input_active() {
            return;
        }

        self.active_screen = screen;
    }

    pub fn next_item(&mut self) {
        if self.is_input_active() {
            return;
        }

        match self.active_screen {
            ActiveScreen::Projects => {
                if self.selected_project_idx + 1 < self.projects.len() {
                    self.selected_project_idx += 1;
                }
            }
            ActiveScreen::Settings => {
                if self.selected_settings_idx + 1 < SETTINGS_OPTIONS_COUNT {
                    self.selected_settings_idx += 1;
                }
            }
        }
    }

    pub fn previous_item(&mut self) {
        if self.is_input_active() {
            return;
        }

        match self.active_screen {
            ActiveScreen::Projects => {
                if self.selected_project_idx > 0 {
                    self.selected_project_idx -= 1;
                }
            }
            ActiveScreen::Settings => {
                if self.selected_settings_idx > 0 {
                    self.selected_settings_idx -= 1;
                }
            }
        }
    }

    pub fn confirm_settings_action(&mut self) -> AppCommand {
        if self.active_screen != ActiveScreen::Settings || self.is_input_active() {
            return AppCommand::None;
        }

        match self.selected_settings_idx {
            0 => {
                self.toggle_ide();
                AppCommand::SaveConfig
            }
            1 => {
                self.toggle_key_layout();
                AppCommand::SaveConfig
            }
            2 => {
                self.start_project_input();
                AppCommand::None
            }
            3 => {
                self.start_projects_directory_input();
                AppCommand::None
            }
            4 => self.delete_selected_project(),
            _ => AppCommand::None,
        }
    }

    pub fn selected_project_launch_target(&self) -> Option<(String, String)> {
        let project = self.projects.get(self.selected_project_idx)?;
        let ide = project
            .ide
            .clone()
            .unwrap_or_else(|| self.config.selected_ide.clone());

        Some((ide, project.path.clone()))
    }

    pub fn key_layout_label(&self) -> &str {
        match self.config.key_layout.as_str() {
            "vim" => "Vim",
            _ => "Default",
        }
    }

    pub fn is_input_active(&self) -> bool {
        self.input_mode != InputMode::None
    }

    pub fn input_value(&self) -> &str {
        match self.input_mode {
            InputMode::None => "",
            InputMode::AddProjectName => &self.pending_project_name,
            InputMode::AddProjectPath => &self.pending_project_path,
            InputMode::AddProjectsDirectory => &self.pending_project_path,
        }
    }

    pub fn input_label(&self) -> &str {
        match self.input_mode {
            InputMode::None => "",
            InputMode::AddProjectName => "Project name",
            InputMode::AddProjectPath => "Project path",
            InputMode::AddProjectsDirectory => "Projects directory",
        }
    }

    pub fn push_input_char(&mut self, input_char: char) {
        match self.input_mode {
            InputMode::None => {}
            InputMode::AddProjectName => self.pending_project_name.push(input_char),
            InputMode::AddProjectPath | InputMode::AddProjectsDirectory => {
                self.pending_project_path.push(input_char)
            }
        }
    }

    pub fn pop_input_char(&mut self) {
        match self.input_mode {
            InputMode::None => {}
            InputMode::AddProjectName => {
                self.pending_project_name.pop();
            }
            InputMode::AddProjectPath => {
                self.pending_project_path.pop();
            }
            InputMode::AddProjectsDirectory => {
                self.pending_project_path.pop();
            }
        }
    }

    pub fn confirm_input(&mut self) -> AppCommand {
        match self.input_mode {
            InputMode::None => AppCommand::None,
            InputMode::AddProjectName => {
                if !self.pending_project_name.trim().is_empty() {
                    self.input_mode = InputMode::AddProjectPath;
                }

                AppCommand::None
            }
            InputMode::AddProjectPath => {
                if self.pending_project_path.trim().is_empty() {
                    return AppCommand::None;
                }

                self.projects.push(SavedProject {
                    name: self.pending_project_name.trim().to_string(),
                    path: self.pending_project_path.trim().to_string(),
                    ide: None,
                });
                self.selected_project_idx = self.projects.len().saturating_sub(1);
                self.cancel_input();

                AppCommand::SaveProjects
            }
            InputMode::AddProjectsDirectory => {
                if self.pending_project_path.trim().is_empty() {
                    return AppCommand::None;
                }

                let directory_path = self.pending_project_path.trim().to_string();
                self.cancel_input();

                AppCommand::ImportProjectsFromDirectory(directory_path)
            }
        }
    }

    pub fn add_projects(&mut self, projects: Vec<SavedProject>) {
        for project in projects {
            if self.projects.iter().any(|saved| saved.path == project.path) {
                continue;
            }

            self.projects.push(project);
        }

        if !self.projects.is_empty() {
            self.selected_project_idx = self.projects.len().saturating_sub(1);
        }
    }

    pub fn cancel_input(&mut self) {
        self.input_mode = InputMode::None;
        self.pending_project_name.clear();
        self.pending_project_path.clear();
    }

    fn toggle_ide(&mut self) {
        self.config.selected_ide = if self.config.selected_ide == "VS Code" {
            "Zed".to_string()
        } else {
            "VS Code".to_string()
        };
    }

    fn toggle_key_layout(&mut self) {
        if self.config.key_layout == "vim" {
            self.config.key_layout = "default".to_string();
            self.config.keymap = crate::storage::KeymapConfig::default_layout();
        } else {
            self.config.key_layout = "vim".to_string();
            self.config.keymap = crate::storage::KeymapConfig::vim_layout();
        }
    }

    fn start_project_input(&mut self) {
        self.pending_project_name.clear();
        self.pending_project_path.clear();
        self.input_mode = InputMode::AddProjectName;
    }

    fn start_projects_directory_input(&mut self) {
        self.pending_project_name.clear();
        self.pending_project_path.clear();
        self.input_mode = InputMode::AddProjectsDirectory;
    }

    fn delete_selected_project(&mut self) -> AppCommand {
        if self.projects.is_empty() {
            return AppCommand::None;
        }

        self.projects.remove(self.selected_project_idx);

        if self.selected_project_idx >= self.projects.len() {
            self.selected_project_idx = self.projects.len().saturating_sub(1);
        }

        AppCommand::SaveProjects
    }
}
