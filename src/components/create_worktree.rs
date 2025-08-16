use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, BorderType, Clear, Padding, Paragraph, Widget},
    Frame,
};

use super::EventState;

enum Focus {
    TextField,
    Buttons,
}

pub struct CreateWorktreeComponent {
    pub character_index: usize,
    pub new_worktree_name: String,
    focus: Focus,
}

impl CreateWorktreeComponent {
    pub fn new() -> Self {
        Self {
            character_index: 0,
            new_worktree_name: String::new(),
            focus: Focus::TextField,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Clear, area);
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title(" New Worktree ")
            .bold()
            .title_alignment(Alignment::Center)
            .render(area, frame.buffer_mut());

        let [_, label_area, input_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(3),
        ])
        .horizontal_margin(6)
        .areas(area);

        Paragraph::new("New worktree branch name:").render(label_area, frame.buffer_mut());
        Paragraph::new(self.new_worktree_name.as_str())
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .padding(Padding::horizontal(1)),
            )
            .render(input_area, frame.buffer_mut());
    }

    pub fn move_filter_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.new_worktree_name.chars().count())
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.new_worktree_name.insert(index, new_char);
        self.move_filter_cursor_right();
    }

    pub fn byte_index(&self) -> usize {
        self.new_worktree_name
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.new_worktree_name.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self
                .new_worktree_name
                .chars()
                .take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.new_worktree_name.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.new_worktree_name = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> EventState {
        match self.focus {
            Focus::TextField => match key.code {
                KeyCode::Tab => self.focus = Focus::Buttons,
                KeyCode::Char(to_insert) => {
                    self.enter_char(to_insert);
                }
                KeyCode::Backspace => {
                    self.delete_char();
                }
                _ => return EventState::NotConsumed,
            },
            Focus::Buttons => return EventState::NotConsumed,
        }
        EventState::Consumed
    }
}
