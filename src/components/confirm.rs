use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
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

        let outer_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(super::BORDER_STYLE)
            .title(Line::from(" Delete Worktree ").style(Style::new().fg(Color::Gray)))
            .title_bottom(keybinding_hint());

        let inner_area = outer_block.inner(area);
        outer_block.render(area, frame.buffer_mut());

        let [_, label_area, _, path_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .horizontal_margin(4)
        .areas(inner_area);

        Paragraph::new("Delete this worktree?").render(label_area, frame.buffer_mut());
        Paragraph::new(self.pending_path.as_str())
            .style(Style::default().fg(Color::Red).bold())
            .render(path_area, frame.buffer_mut());
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
