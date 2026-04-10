use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

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

    pub fn draw(&mut self, f: &mut Frame, rect: Rect, is_active: bool) {
        let border_style = if is_active {
            Style::new().fg(Color::Cyan)
        } else {
            super::BORDER_STYLE
        };
        let input = Paragraph::new(self.value.as_str()).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(border_style)
                .title(Line::from(self.title.as_str()).style(Style::new().fg(Color::Gray)))
                .style(Style::new().white()),
        );
        f.render_widget(input, rect);
    }

    pub fn clear(&mut self) {
        self.value.clear();
        self.character_index = 0;
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.value.insert(index, new_char);
        self.move_filter_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;
            let before_char_to_delete = self.value.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.value.chars().skip(current_index);
            self.value = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn move_filter_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.value.chars().count())
    }

    fn byte_index(&self) -> usize {
        self.value
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.value.len())
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }
}
