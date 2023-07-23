use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols::line,
    text::{Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};

use crate::{
    app::{Task, Tasklist},
    App,
};

fn lists<'a, B: Backend>(list_names: &[Tasklist]) -> Tabs<'a> {
    let tabs = list_names
        .iter()
        .map(|x| Spans::from(x.title.clone()))
        .collect();

    Tabs::new(tabs)
        .block(Block::default().borders(Borders::BOTTOM))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(line::VERTICAL)
}

fn todos_component<'a, B: Backend>(todos: &[Task]) -> List<'a> {
    let todos = todos
        .iter()
        .map(|x| ListItem::new(x.title.clone()))
        .collect::<Vec<ListItem>>();

    List::new(todos)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(2), Constraint::Percentage(100)].as_ref())
        .split(f.size());

    if let Some(tasklist) = app.active_tasklist() {
        let tabs = lists::<B>(&app.tasklists).select(app.active_tasklist);
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
