use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, GREEN, RED, SLATE},
        Style, Stylize,
    },
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Padding, Paragraph, Widget},
    Frame,
};

use super::{Action, EventState};

pub struct PrWorktreeComponent {
    character_index: usize,
    pub input: String,
    pub error: Option<String>,
    pub auto_clone: bool,
}

impl PrWorktreeComponent {
    pub fn new() -> Self {
        Self {
            character_index: 0,
            input: String::new(),
            error: None,
            auto_clone: false,
        }
    }

    pub fn current_url(&self) -> &str {
        &self.input
    }

    pub fn set_error(&mut self, msg: String) {
        self.error = Some(msg);
    }

    pub fn reset(&mut self) {
        self.input.clear();
        self.character_index = 0;
        self.error = None;
        self.auto_clone = false;
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Clear, area);

        let outer_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(super::POPUP_BORDER_STYLE)
            .title(Line::from(" Worktree from PR ").style(Style::new().fg(GREEN.c300).bold()))
            .title_bottom(keybinding_hint());

        let inner_area = outer_block.inner(area);
        outer_block.render(area, frame.buffer_mut());

        let [_, label_area, input_area, status_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .horizontal_margin(4)
        .areas(inner_area);

        Paragraph::new("GitHub PR URL:")
            .style(Style::new().fg(SLATE.c300))
            .render(label_area, frame.buffer_mut());

        Paragraph::new(self.input.as_str())
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(super::ACTIVE_BORDER_STYLE)
                    .padding(Padding::horizontal(1)),
            )
            .render(input_area, frame.buffer_mut());

        if let Some(err) = &self.error {
            Paragraph::new(err.as_str())
                .style(Style::new().fg(RED.c400))
                .render(status_area, frame.buffer_mut());
        }

        // input_area: border(1) + padding(1) = offset 2; y+1 skips top border row
        frame.set_cursor_position((
            input_area.x + 2 + self.character_index as u16,
            input_area.y + 1,
        ));
    }

    pub fn handle_action(&mut self, action: Action) -> EventState {
        match action {
            Action::InsertChar(c) => {
                self.enter_char(c);
                EventState::Consumed
            }
            Action::DeleteChar => {
                self.delete_char();
                EventState::Consumed
            }
            _ => EventState::NotConsumed,
        }
    }

    fn enter_char(&mut self, c: char) {
        let index = self.byte_index();
        self.input.insert(index, c);
        self.move_cursor_right();
        self.error = None;
    }

    fn delete_char(&mut self) {
        if self.character_index != 0 {
            let current_index = self.character_index;
            let before = self.input.chars().take(current_index - 1);
            let after = self.input.chars().skip(current_index);
            self.input = before.chain(after).collect();
            self.move_cursor_left();
            self.error = None;
        }
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn move_cursor_right(&mut self) {
        let moved = self.character_index.saturating_add(1);
        self.character_index = moved.clamp(0, self.input.chars().count());
    }

    fn move_cursor_left(&mut self) {
        let moved = self.character_index.saturating_sub(1);
        self.character_index = moved.clamp(0, self.input.chars().count());
    }
}

fn keybinding_hint() -> Line<'static> {
    Line::from(vec![
        Span::styled("[Enter] ", Style::new().fg(BLUE.c400).bold()),
        Span::styled("open", Style::new().fg(SLATE.c500)),
        Span::styled("  [Esc] ", Style::new().fg(BLUE.c400).bold()),
        Span::styled("cancel ", Style::new().fg(SLATE.c500)),
    ])
    .right_aligned()
}
