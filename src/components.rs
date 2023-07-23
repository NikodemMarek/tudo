use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols::line,
    text::{Spans, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs},
    Frame,
};

use crate::{
    app::{Task, Tasklist},
    App,
};

fn tasklists<'a, B: Backend>(list_names: &[Tasklist]) -> Tabs<'a> {
    let tabs = list_names
        .iter()
        .map(|x| Spans::from(x.title.clone()))
        .collect();

    Tabs::new(tabs)
        .block(Block::default().borders(Borders::BOTTOM))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Blue))
        .divider(line::VERTICAL)
}

fn todos_component<'a, B: Backend>(todos: &[Task]) -> Table<'a> {
    let todos = todos
        .iter()
        .map(|x| {
            Row::new(vec![
                // TODO: Format date like in the original
                Cell::from(
                    x.due
                        .map(|x| x.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or("".to_string()),
                )
                .style(Style::default().fg(Color::Red)),
                Cell::from(x.title.clone()),
            ])
        })
        .collect::<Vec<Row>>();

    Table::new(todos)
        .header(Row::new(vec!["Due", "Title"]))
        .widths(&[Constraint::Length(20), Constraint::Length(20)])
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        )
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(2), Constraint::Percentage(100)].as_ref())
        .split(f.size());

    if let Some(tasklist) = app.active_tasklist() {
        let tabs = tasklists::<B>(&app.tasklists).select(app.active_tasklist);
        f.render_widget(tabs, chunks[0]);

        match tasklist.is_empty() {
            false => f.render_stateful_widget(
                todos_component::<B>(&tasklist.tasks),
                chunks[1],
                &mut app.tasks_state,
            ),
            true => f.render_widget(
                Paragraph::new(Text::from("No todos in this list!"))
                    .style(Style::default().fg(Color::Green)),
                chunks[1],
            ),
        };
    } else {
        f.render_widget(
            Paragraph::new(Text::from("No tasklists")).style(Style::default().fg(Color::Yellow)),
            chunks[0],
        )
    }
}
