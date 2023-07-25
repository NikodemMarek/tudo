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
    app::{App, Status, Task, Tasklist},
    timestamps::formatter,
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
                match x.status {
                    Status::Todo => Cell::from("☐"),
                    Status::Done => Cell::from("☑"),
                    Status::Unknown => Cell::from("?"),
                },
                {
                    let (str, color) = x
                        .due
                        .as_ref()
                        .map(match x.status {
                            Status::Todo => formatter::relative,
                            _ => formatter::absolute,
                        })
                        .unwrap_or((String::new(), Color::Reset));

                    Cell::from(str).style(Style::default().fg(color))
                },
                Cell::from(x.title.to_owned()),
                Cell::from(x.notes.to_owned().unwrap_or_default()),
            ])
        })
        .collect::<Vec<Row>>();

    Table::new(todos)
        .header(Row::new(vec!["", "Due", "Title", "Notes"]))
        .widths(&[
            Constraint::Length(1),
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(50),
        ])
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
        let tabs = tasklists::<B>(app.provider.get_tasklists()).select(app.active_tasklist);
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
