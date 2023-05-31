use log::debug;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{
    Block, BorderType, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table,
};
use tui::Frame;
use tui_logger::TuiLoggerWidget;

use super::actions::Actions;
use super::state::{AppData, AppState};
use crate::app::App;
use crate::models::key::Key;

pub fn draw<B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let size = rect.size();
    check_size(&size);

    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
                Constraint::Length(12),
            ]
            .as_ref(),
        )
        .split(size);

    // Title
    let title = draw_title();
    rect.render_widget(title, chunks[0]);

    // Body & Help
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunks[1]);

    let body = draw_body(app.is_loading(), app.state(), &app.data);
    rect.render_widget(body, body_chunks[1]);

    draw_keys(&mut app.data, body_chunks[0], rect);
    // rect.render_stateful_widget(keys, body_chunks[0], &mut app.data.keys.state);
    // rect.render_widget(keys, body_chunks[0]);

    // Logs
    let logs = draw_logs();
    rect.render_widget(logs, chunks[3]);

    if app.state.is_help() {
        debug!("actions: {:?}", app.actions());
        let help = draw_help(app.actions());
        let area = centered_rect(80, 80, size);
        rect.render_widget(Clear, area); //this clears out the background
        rect.render_widget(help, area);
    }

    // if app.state.is_creation_popup() {
    //     let popup = draw_creation_form();
    //     let area = centered_rect(80, 80, size);
    //     rect.render_widget(Clear, area); //this clears out the background
    //     rect.render_widget(popup, area);
    // }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Plop with TUI")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}

fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}

fn draw_body<'a>(_loading: bool, state: &AppState, data: &'a AppData) -> Paragraph<'a> {
    // let initialized_text = if state.is_initialized() {
    //     "Initialized"
    // } else {
    //     "Not Initialized !"
    // };
    // let loading_text = if loading { "Loading..." } else { "" };
    let tick_text = if let Some(ticks) = state.count_tick() {
        format!("Tick count: {}", ticks)
    } else {
        String::default()
    };

    let selected_key = data.keys.state.selected();
    match selected_key {
        Some(idx) => {
            data.keys
                .items
                .get(idx)
                .map(|key| {
                    // key.value().to_owned()

                    Paragraph::new(vec![
                        Spans::from(Span::raw(key.name().to_string())),
                        Spans::from(Span::raw(key.value())),
                        Spans::from(Span::raw(key.created_at())),
                        Spans::from(Span::raw(key.updated_at())),
                        Spans::from(Span::raw(tick_text)),
                        // Spans::from(Span::raw(format!("Selected: {:?}", selected))),
                    ])
                    .style(Style::default().fg(Color::LightCyan))
                    .alignment(Alignment::Left)
                    .block(
                        Block::default()
                            .title("Body")
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::White))
                            .border_type(BorderType::Plain),
                    )
                })
                .unwrap_or_else(|| Paragraph::new(vec![Spans::from(Span::raw(""))]))
            // debug!("selected_key: {:?}", e);
        }
        None => {
            // debug!("selected_key: None");
            Paragraph::new(vec![
                Spans::from(Span::raw("")),
                // Spans::from(Span::raw(loading_text)),
                Spans::from(Span::raw(tick_text)),
                // Spans::from(Span::raw(format!("Selected: {:?}", selected))),
            ])
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .title("Body")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Plain),
            )
        }
    }

    // Paragraph::new(vec![
    //     Spans::from(Span::raw(initialized_text)),
    //     Spans::from(Span::raw(loading_text)),
    //     Spans::from(Span::raw(tick_text)),
    //     Spans::from(Span::raw(format!("Selected: {:?}", selected))),
    // ])
}

fn draw_keys<B: Backend>(data: &mut AppData, body_chunk: Rect, rect: &mut Frame<B>) -> () {
    // debug!("state: {:?}", data.keys.state);
    // debug!("items: {:?}", data.keys.items);

    let key_style = Style::default().fg(Color::LightCyan);
    // let value_style = Style::default().fg(Color::Gray);

    let items: Vec<ListItem> = data
        .keys
        .items
        .iter()
        .map(|i: &Key| {
            // let mut lines = vec![Line::from(i.name())];
            //     for _ in 0..i.1 {
            //         lines.push(Line::from(Span::styled(
            //             "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
            //             Style::default().add_modifier(Modifier::ITALIC),
            //         )));
            //     }
            ListItem::new(Span::from(i.name())).style(key_style)
            // .style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        );
    // .highlight_symbol(">> ");

    rect.render_stateful_widget(items, body_chunk, &mut data.keys.state);
}

// fn draw_creation_form() {

// }

fn draw_help(actions: &Actions) -> Table {
    let key_style = Style::default().fg(Color::LightCyan);
    let help_style = Style::default().fg(Color::Gray);

    let mut rows = vec![];
    for action in actions.actions().iter() {
        let mut first = true;
        for key in action.keys() {
            let help = if first {
                first = false;
                action.to_string()
            } else {
                String::from("")
            };
            let row = Row::new(vec![
                Cell::from(Span::styled(key.to_string(), key_style)),
                Cell::from(Span::styled(help, help_style)),
            ]);
            rows.push(row);
        }
    }

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("Help"),
        )
        .widths(&[Constraint::Length(11), Constraint::Min(20)])
        .column_spacing(1)
}

fn draw_logs<'a>() -> TuiLoggerWidget<'a> {
    TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .title("Logs")
                .border_style(Style::default().fg(Color::White).bg(Color::Black))
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
}
