use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{SLATE, VIOLET},
        Color, Style, Stylize,
    },
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph, Widget},
    Frame,
};

use super::{Action, EventState};

pub struct ConfirmComponent {
    pub title: String,
    pub label: String,
    pub detail: String,
}

impl ConfirmComponent {
    pub fn new(title: String, label: String, detail: String) -> Self {
        Self {
            title,
            label,
            detail,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Clear, area);

        let outer_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(super::POPUP_BORDER_STYLE)
            .title(
                Line::from(format!(" {} ", self.title)).style(Style::new().fg(VIOLET.c300).bold()),
            )
            .title_bottom(keybinding_hint());

        let inner_area = outer_block.inner(area);
        outer_block.render(area, frame.buffer_mut());

        let [_, label_area, _, detail_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .horizontal_margin(4)
        .areas(inner_area);

        Paragraph::new(self.label.as_str())
            .style(Style::new().fg(SLATE.c300))
            .render(label_area, frame.buffer_mut());
        Paragraph::new(self.detail.as_str())
            .style(Style::default().fg(Color::Red).bold())
            .render(detail_area, frame.buffer_mut());
    }

    pub fn handle_action(&mut self, _action: Action) -> EventState {
        EventState::NotConsumed
    }
}

fn keybinding_hint() -> Line<'static> {
    Line::from(vec![
        Span::styled("[Enter] ", Style::new().white().bold()),
        Span::styled("confirm", Style::new().dark_gray()),
        Span::styled("  [Esc] ", Style::new().white().bold()),
        Span::styled("cancel ", Style::new().dark_gray()),
    ])
    .right_aligned()
}
