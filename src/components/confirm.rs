use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Clear, Paragraph, Widget},
    Frame,
};

use super::{Action, EventState};

pub struct ConfirmComponent {
    pub pending_path: String,
}

impl ConfirmComponent {
    pub fn new(pending_path: String) -> Self {
        Self { pending_path }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Clear, area);
        Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(super::BORDER_STYLE)
            .title(" Delete Worktree ")
            .bold()
            .title_alignment(Alignment::Center)
            .render(area, frame.buffer_mut());

        let [_, label_area, _, path_area, _, hints_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .horizontal_margin(6)
        .areas(area);

        Paragraph::new("Delete this worktree?").render(label_area, frame.buffer_mut());
        Paragraph::new(self.pending_path.as_str())
            .style(Style::default().fg(Color::Red).bold())
            .render(path_area, frame.buffer_mut());
        Paragraph::new("[Enter] confirm  [Esc] cancel")
            .style(Style::default().dim())
            .render(hints_area, frame.buffer_mut());
    }

    pub fn handle_action(&mut self, _action: Action) -> EventState {
        EventState::NotConsumed
    }
}
