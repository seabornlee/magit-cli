use tui::backend::Backend;
use tui::Frame;
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, List, ListItem};

use crate::app::App;
use crate::git_helper;
use tui::layout::{Layout, Direction, Constraint};

pub fn draw<B: Backend>(f: &mut Frame<B>, _app: &mut App) {
    let unstaged_list = to_list(git_helper::get_unstaged().unwrap(), "UnStaged");
    let staged_list = to_list(git_helper::get_staged().unwrap(), "Staged");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref()).split(f.size());

    f.render_widget(unstaged_list, chunks[0]);
    f.render_widget(staged_list, chunks[1]);
}

fn to_list(file_names: Vec<String>, title: &'static str) -> List<'static> {
    let items: Vec<ListItem> = file_names.iter().map(|path| {
        ListItem::new(Span::raw(String::from("  ".to_owned() + path)))
            .style(Style::default().fg(Color::White))
    }).collect();

    let file_list = List::new(items)
        .block(Block::default().borders(Borders::ALL)
            .title(Span::styled(title, Style::default().add_modifier(Modifier::BOLD))))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    file_list
}