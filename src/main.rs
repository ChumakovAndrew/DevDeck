mod app;
mod keymap;
mod project_importer;
mod project_launcher;
mod screens;
mod storage;
mod ui;

use std::io;

use app::{ActiveScreen, App, AppCommand};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use keymap::{AppAction, action_for_key};
use ratatui::{Terminal, backend::CrosstermBackend};
use storage::Storage;

fn main() -> Result<(), io::Error> {
    let storage = Storage::load_or_create()?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::from_storage(storage.config, storage.projects);
    let mut storage_error = None;

    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;

        if let Event::Key(key) = event::read()? {
            if app.is_input_active() {
                let command = match key.code {
                    KeyCode::Enter => app.confirm_input(),
                    KeyCode::Esc => {
                        app.cancel_input();
                        AppCommand::None
                    }
                    KeyCode::Backspace => {
                        app.pop_input_char();
                        AppCommand::None
                    }
                    KeyCode::Char(input_char) => {
                        app.push_input_char(input_char);
                        AppCommand::None
                    }
                    _ => AppCommand::None,
                };

                if let Err(error) = handle_app_command(command, &mut app) {
                    storage_error = Some(error);
                    break;
                }

                continue;
            }

            match action_for_key(&app.config.keymap, key.code) {
                AppAction::Quit => break,
                AppAction::OpenSettings => app.set_screen(ActiveScreen::Settings),
                AppAction::OpenProjects => app.set_screen(ActiveScreen::Projects),
                AppAction::NextProject => app.next_item(),
                AppAction::PreviousProject => app.previous_item(),
                AppAction::Confirm => {
                    if app.active_screen == ActiveScreen::Projects {
                        if let Some((ide, path)) = app.selected_project_launch_target() {
                            if let Err(error) = project_launcher::launch_project(&ide, &path) {
                                storage_error = Some(error);
                                break;
                            }
                        }

                        continue;
                    }

                    let command = app.confirm_settings_action();

                    if let Err(error) = handle_app_command(command, &mut app) {
                        storage_error = Some(error);
                        break;
                    }
                }
                AppAction::None => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Some(error) = storage_error {
        return Err(error);
    }

    Ok(())
}

fn handle_app_command(command: AppCommand, app: &mut App) -> io::Result<()> {
    match command {
        AppCommand::None => Ok(()),
        AppCommand::SaveConfig => storage::save_config(&app.config),
        AppCommand::SaveProjects => storage::save_projects(&app.projects),
        AppCommand::ImportProjectsFromDirectory(path) => {
            let imported_projects = project_importer::projects_from_directory(&path)?;
            app.add_projects(imported_projects);
            storage::save_projects(&app.projects)
        }
    }
}
