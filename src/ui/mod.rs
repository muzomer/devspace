use ratatui::layout::{Flex, Rect};
use ratatui::Frame;
use ratatui::{
    layout::{Constraint, Layout},
    style::{palette::tailwind::SLATE, Modifier, Style, Stylize},
    widgets::Paragraph,
};

mod app;
mod create_worktree;
mod events;
mod repositories;
mod worktrees;

pub use app::{App, Screen};
pub use events::{handle_event, HandleEventResult};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub fn draw(frame: &mut Frame, app: &mut App) {
    let [header_area, main_area, footer_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(frame.area());

    let header = Paragraph::new("Git Worktrees").bold().centered();
    let footer = Paragraph::new("Use ↓↑ or j/k to move, g/G to go top/bottom.").centered();
    frame.render_widget(header, header_area);
    frame.render_widget(footer, footer_area);
    app.worktrees.draw(frame, main_area);

    if let Screen::ListRepos = app.current_screen {
        let popup_area = popup_area(main_area, 50, 50);
        app.repos.draw(frame, popup_area);
    }

    if let Screen::CreateWorktree(_) = app.current_screen {
        let popup_area = popup_area(main_area, 50, 20);
        app.new_worktree.draw(frame, popup_area);
    }
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
