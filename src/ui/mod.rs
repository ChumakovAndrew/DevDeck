use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Tabs},
};

use crate::{
    app::{ActiveScreen, App},
    screens::{render_projects_screen, render_settings_screen},
};

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.size());

    render_navigation(frame, chunks[0], app.active_screen);

    match app.active_screen {
        ActiveScreen::Projects => render_projects_screen(frame, chunks[1], app),
        ActiveScreen::Settings => render_settings_screen(frame, chunks[1], app),
    }
}

fn render_navigation(frame: &mut Frame, area: ratatui::layout::Rect, active_screen: ActiveScreen) {
    let titles = vec![" [←] 1. Projects ", " 2. Settings [→] "];
    let active_tab_idx = match active_screen {
        ActiveScreen::Projects => 0,
        ActiveScreen::Settings => 1,
    };

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" Navigation "))
        .select(active_tab_idx)
        .highlight_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(tabs, area);
}
