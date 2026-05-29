use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::App;

pub fn render_settings_screen(frame: &mut Frame, area: Rect, app: &App) {
    if app.is_input_active() {
        render_project_input(frame, area, app);
        return;
    }

    let selected_project = app
        .projects
        .get(app.selected_project_idx)
        .map(|project| project.name.as_str())
        .unwrap_or("no projects");

    let options = [
        format!("Current IDE: {}", app.config.selected_ide),
        format!("Key layout: {}", app.key_layout_label()),
        "Add project".to_string(),
        "Add all projects from directory".to_string(),
        format!("Delete selected project: {}", selected_project),
    ];

    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(idx, option)| {
            if idx == app.selected_settings_idx {
                ListItem::new(format!("  ▶ {} ", option)).style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                ListItem::new(format!("    {} ", option))
            }
        })
        .collect();

    let settings_list = List::new(items).block(
        Block::default()
            .title(" SCREEN: Application settings ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green)),
    );

    frame.render_widget(settings_list, area);
}

fn render_project_input(frame: &mut Frame, area: Rect, app: &App) {
    let input_text = format!(
        "\n  {}\n\n  {}\n\n  Enter - continue/save\n  Esc - cancel",
        app.input_label(),
        app.input_value()
    );

    let input_paragraph = Paragraph::new(input_text).block(
        Block::default()
            .title(" Add project ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow)),
    );

    frame.render_widget(input_paragraph, area);
}
