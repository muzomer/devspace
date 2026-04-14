pub struct FilterComponent {
    pub value: String,
    character_index: usize,
}

impl FilterComponent {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            character_index: 0,
        }
    }

    /// Current cursor position within the filter text (in chars).
    pub fn cursor_pos(&self) -> usize {
        self.character_index
    }

    pub fn clear(&mut self) {
        self.value.clear();
        self.character_index = 0;
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.value.insert(index, new_char);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        if self.character_index != 0 {
            let current_index = self.character_index;
            let before = self.value.chars().take(current_index - 1);
            let after = self.value.chars().skip(current_index);
            self.value = before.chain(after).collect();
            self.move_cursor_left();
        }
    }

    fn move_cursor_right(&mut self) {
        let moved = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(moved);
    }

    fn move_cursor_left(&mut self) {
        let moved = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(moved);
    }

    fn clamp_cursor(&self, pos: usize) -> usize {
        pos.clamp(0, self.value.chars().count())
    }

    fn byte_index(&self) -> usize {
        self.value
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.value.len())
    }
}
