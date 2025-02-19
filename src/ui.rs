use crate::app::{App, CurrentScreen};
use ratatui::layout::{Flex, Rect};
use ratatui::widgets::StatefulWidget;
use ratatui::Frame;
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{palette::tailwind::SLATE, Modifier, Style, Stylize},
    widgets::{Block, Borders, List, ListDirection, Paragraph},
};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub fn ui(frame: &mut Frame, app: &mut App) {
    let [header_area, main_area, footer_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(frame.area());

    let header = Paragraph::new("Devspaces").bold().centered();
    let footer = Paragraph::new("Use ↓↑ or j/k to move, g/G to go top/bottom.").centered();
    frame.render_widget(header, header_area);
    frame.render_widget(footer, footer_area);
    let block = Block::new().borders(Borders::all());
    let list = List::new(app.devspaces.items.clone())
        .block(block)
        .style(Style::new().white())
        .highlight_style(SELECTED_STYLE)
        .direction(ListDirection::TopToBottom);
    StatefulWidget::render(
        list,
        main_area,
        frame.buffer_mut(),
        &mut app.devspaces.state,
    );

    if let CurrentScreen::ListRepos = app.current_screen {
        let block = Block::bordered()
            .title("Repositories")
            .title_alignment(Alignment::Center)
            .style(Style::new().light_green());
        let list = List::new(app.repos.items.clone())
            .block(block)
            .style(Style::new().white())
            .highlight_style(SELECTED_STYLE)
            .direction(ListDirection::TopToBottom);
        let area = repos_list_popup(main_area, 50, 50);
        StatefulWidget::render(list, area, frame.buffer_mut(), &mut app.repos.state);
    }
}

fn repos_list_popup(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);

    area
}
