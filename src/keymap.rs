use crossterm::event::KeyCode;

use crate::storage::KeymapConfig;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AppAction {
    Quit,
    OpenProjects,
    OpenSettings,
    NextProject,
    PreviousProject,
    Confirm,
    None,
}

pub fn action_for_key(keymap: &KeymapConfig, key_code: KeyCode) -> AppAction {
    if key_matches(&keymap.quit, key_code) {
        AppAction::Quit
    } else if key_matches(&keymap.open_projects, key_code) {
        AppAction::OpenProjects
    } else if key_matches(&keymap.open_settings, key_code) {
        AppAction::OpenSettings
    } else if key_matches(&keymap.next_project, key_code) {
        AppAction::NextProject
    } else if key_matches(&keymap.previous_project, key_code) {
        AppAction::PreviousProject
    } else if key_matches(&keymap.confirm, key_code) {
        AppAction::Confirm
    } else {
        AppAction::None
    }
}

fn key_matches(configured_key: &str, key_code: KeyCode) -> bool {
    match configured_key.to_ascii_lowercase().as_str() {
        "enter" => key_code == KeyCode::Enter,
        "left" => key_code == KeyCode::Left,
        "right" => key_code == KeyCode::Right,
        "up" => key_code == KeyCode::Up,
        "down" => key_code == KeyCode::Down,
        key if key.chars().count() == 1 => {
            let configured_char = key.chars().next();
            matches!(key_code, KeyCode::Char(input_char) if Some(input_char) == configured_char)
        }
        _ => false,
    }
}
