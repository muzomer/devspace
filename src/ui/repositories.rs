use ratatui::layout::{Alignment, Rect};
use ratatui::widgets::ListState;

use ratatui::Frame;
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, List, ListDirection, Paragraph, StatefulWidget},
};

use ratatui::style::{Style, Stylize};

use crate::model::Repository;

use super::SELECTED_STYLE;

#[derive(Debug, Clone)]
pub struct RepositoriesList {
    pub items: Vec<String>,
    pub state: ListState,
    pub filter: String,
    pub filter_character_index: usize,
    pub filtered_items: Vec<String>,
}

impl RepositoriesList {
    pub fn new(items: Vec<Repository>) -> Self {
        let dirs: Vec<String> = items.iter().map(|dir| dir.path.clone()).collect();
        let mut new = Self {
            items: dirs.clone(),
            state: ListState::default(),
            filter: String::new(),
            filter_character_index: 0,
            filtered_items: dirs.clone(),
        };
        new.state.select_first();
        new
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let vertical = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]);
        let [filter_area, repos_list_area] = vertical.areas(area);

        let input = Paragraph::new(self.filter.as_str()).block(Block::bordered().title("Filter"));
        frame.render_widget(input, filter_area);

        let block = Block::bordered()
            .title("Repositories")
            .title_alignment(Alignment::Center);

        let list = List::new(self.filtered_items.clone())
            .block(block)
            .style(Style::new().white())
            .highlight_style(SELECTED_STYLE)
            .direction(ListDirection::TopToBottom);

        StatefulWidget::render(list, repos_list_area, frame.buffer_mut(), &mut self.state);
    }
    pub fn select_next(&mut self) {
        if let Some(index) = self.state.selected() {
            if index == self.filtered_items.len() - 1 {
                self.state.select_first();
            } else {
                self.state.select_next();
            }
        }
    }
    pub fn select_previous(&mut self) {
        if let Some(index) = self.state.selected() {
            if index == 0 {
                self.state.select_last();
            } else {
                self.state.select_previous();
            }
        }
    }
    pub fn select_first(&mut self) {
        self.state.select_first();
    }

    pub fn select_last(&mut self) {
        self.state.select_last();
    }

    pub fn move_filter_cursor_right(&mut self) {
        let cursor_moved_right = self.filter_character_index.saturating_add(1);
        self.filter_character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.filter.chars().count())
    }

    pub fn update_filtered_items(&mut self) {
        self.filtered_items = self
            .items
            .iter()
            .filter(|repo| repo.contains(&self.filter))
            .cloned()
            .collect();
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.filter.insert(index, new_char);
        self.move_filter_cursor_right();
    }

    pub fn byte_index(&self) -> usize {
        self.filter
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.filter_character_index)
            .unwrap_or(self.filter.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.filter_character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.filter_character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.filter.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.filter.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.filter = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.filter_character_index.saturating_sub(1);
        self.filter_character_index = self.clamp_cursor(cursor_moved_left);
    }
}
