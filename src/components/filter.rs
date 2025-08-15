use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use super::EventState;

pub struct FilterComponent {
    pub value: String,
    title: String,
    character_index: usize,
}

impl FilterComponent {
    pub fn new(title: String) -> Self {
        Self {
            value: String::new(),
            character_index: 0,
            title,
        }
    }

    pub fn draw(&mut self, f: &mut Frame, rect: Rect) {
        let input = Paragraph::new(self.value.as_str()).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(self.title.as_str())
                .style(Style::new().white().bold()),
        );
        f.render_widget(input, rect);
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> EventState {
        if key.modifiers == KeyModifiers::CONTROL {
            return EventState::NotConsumed;
        }
        match key.code {
            KeyCode::Backspace => {
                self.delete_char();
            }
            KeyCode::Char(to_insert) => {
                self.enter_char(to_insert);
            }
            _ => return EventState::NotConsumed,
        }
        EventState::Consumed
    }

    fn move_filter_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.value.chars().count())
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.value.insert(index, new_char);
        self.move_filter_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.value
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.value.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.value.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.value.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.value = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }
}
