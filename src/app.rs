use ratatui::widgets::ListState;

use crate::cli;
use crate::devspace;

#[derive(Debug)]
pub enum ListingScreenMode {
    Filtering,
    Navigating,
}

#[derive(Debug)]
pub enum CurrentScreen {
    ListDevspaces(ListingScreenMode),
    ListRepos(ListingScreenMode),
}

#[derive(Debug)]
pub struct App {
    pub devspaces: DevspaceList,
    pub repos: RepositoriesList,
    pub exit: bool,
    pub selected_space: String,
    pub current_screen: CurrentScreen,
}

impl Default for App {
    fn default() -> Self {
        let args = cli::Args::new();
        let devspaces = devspace::list(&args.spaces_dir).unwrap_or_default();
        let repos = devspace::list(&args.repos_dirs).unwrap_or_default();

        Self {
            devspaces: DevspaceList::new(devspaces),
            repos: RepositoriesList::new(repos),
            exit: false,
            selected_space: String::new(),
            current_screen: CurrentScreen::ListDevspaces(ListingScreenMode::Filtering),
        }
    }
}

impl App {
    pub fn go_to_devspace(&mut self) {
        if let Some(selected_index) = self.devspaces.state.selected() {
            let selected_space = &self.devspaces.items[selected_index];
            self.selected_space = selected_space.clone();
        }
        self.exit();
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn print_devspace_dir(&self) {
        println!("{}", self.selected_space);
    }
}

#[derive(Debug, Clone)]
pub struct DevspaceList {
    pub items: Vec<String>,
    pub state: ListState,
    pub filter: String,
    pub filter_character_index: usize,
    pub filtered_items: Vec<String>,
}

impl DevspaceList {
    pub fn new(items: Vec<String>) -> Self {
        Self {
            items: items.clone(),
            state: ListState::default(),
            filter: String::new(),
            filter_character_index: 0,
            filtered_items: items.clone(),
        }
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
            .filter(|devspace| devspace.contains(&self.filter))
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

#[derive(Debug, Clone)]
pub struct RepositoriesList {
    pub items: Vec<String>,
    pub state: ListState,
    pub filter: String,
    pub filter_character_index: usize,
    pub filtered_items: Vec<String>,
}

impl RepositoriesList {
    pub fn new(items: Vec<String>) -> Self {
        Self {
            items: items.clone(),
            state: ListState::default(),
            filter: String::new(),
            filter_character_index: 0,
            filtered_items: items.clone(),
        }
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
            .filter(|devspace| devspace.contains(&self.filter))
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
