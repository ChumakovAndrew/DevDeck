use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::App;

pub fn render_projects_screen(frame: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .projects
        .iter()
        .enumerate()
        .map(|(idx, project)| {
            if idx == app.selected_project_idx {
                ListItem::new(format!("  ▶ {} ", project.name)).style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                ListItem::new(format!("    {} ", project.name))
            }
        })
        .collect();

    let projects_list = List::new(items).block(
        Block::default()
            .title(" SCREEN: Project list (↑/↓ select, Enter launch) ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green)),
    );

    frame.render_widget(projects_list, area);
}
