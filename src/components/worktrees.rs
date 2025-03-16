use arboard::Clipboard;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget},
    Frame,
};
use tracing::{debug, error};

use crate::git::Worktree;

use super::{filter::FilterComponent, EventState};
use super::{
    list::{Focus, ItemOrder, ListComponent},
    SELECTED_STYLE,
};

pub struct WorktreesComponent {
    worktrees: Vec<Worktree>,
    filter: FilterComponent,
    state: ListState,
    focus: Focus,
    selected_index: Option<usize>,
}

impl WorktreesComponent {
    pub fn new(worktrees: Vec<Worktree>) -> WorktreesComponent {
        let selected_index = if worktrees.is_empty() { None } else { Some(0) };
        debug!("selected_index: {:#?}", selected_index);
        Self {
            filter: FilterComponent::default(),
            state: ListState::default().with_selected(selected_index),
            focus: Focus::Filter,
            selected_index,
            worktrees,
        }
    }

    pub fn draw(&mut self, f: &mut Frame, rect: Rect) {
        let block = Block::new().borders(Borders::all());
        let [filter_area, list_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(rect);

        self.filter.draw(f, filter_area);

        let list = List::new(self.filtered_items())
            .block(block)
            .style(Style::new().white())
            .highlight_style(SELECTED_STYLE)
            .direction(ratatui::widgets::ListDirection::TopToBottom);

        StatefulWidget::render(list, list_area, f.buffer_mut(), &mut self.state);
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> EventState {
        match self.focus {
            Focus::Filter => {
                let result = self.filter.handle_key(key);
                if result == EventState::Consumed {
                    result
                } else {
                    if key.modifiers == KeyModifiers::CONTROL {
                        match key.code {
                            KeyCode::Char('n') => {
                                self.select(ItemOrder::Next);
                            }
                            KeyCode::Char('p') => {
                                self.select(ItemOrder::Previous);
                            }
                            _ => return EventState::NotConsumed,
                        }
                    } else {
                        match key.code {
                            KeyCode::Tab => self.focus = Focus::List,
                            KeyCode::Enter => {
                                self.copy_path_of_selected_worktree();
                                return EventState::NotConsumed;
                            }
                            _ => return EventState::NotConsumed,
                        }
                    }
                    EventState::Consumed
                }
            }
            Focus::List => {
                match key.code {
                    KeyCode::Char('j') | KeyCode::Down => self.select(ItemOrder::Next),
                    KeyCode::Char('k') | KeyCode::Up => self.select(ItemOrder::Previous),
                    KeyCode::Char('g') | KeyCode::Home => self.select(ItemOrder::First),
                    KeyCode::Char('G') | KeyCode::End => self.select(ItemOrder::Last),
                    KeyCode::Tab => self.focus = Focus::Filter,
                    KeyCode::Enter => {
                        self.copy_path_of_selected_worktree();
                        return EventState::NotConsumed;
                    }
                    _ => return EventState::NotConsumed,
                }
                EventState::Consumed
            }
        }
    }

    pub fn add(&mut self, new_worktree: Worktree) {
        let new_worktree_path = new_worktree.path().to_string();
        self.worktrees.push(new_worktree);
        let new_worktree_index = self
            .filtered_items()
            .iter()
            .position(|wt| wt.path().to_string().eq(&new_worktree_path));

        self.state.select(new_worktree_index);
        self.selected_index = new_worktree_index;
    }

    pub fn selected_worktree(&mut self) -> Option<&Worktree> {
        match self.selected_index {
            Some(index) => Some(self.filtered_items().get(index).unwrap()),
            None => None,
        }
    }

    fn copy_path_of_selected_worktree(&mut self) {
        match Clipboard::new() {
            Ok(mut clipboard) => match self.selected_worktree() {
                Some(selected_worktree) => {
                    match clipboard.set_text(selected_worktree.path().to_string()) {
                        Ok(_) => {
                            debug!("Copied the path {} to clipboard", selected_worktree.path())
                        }
                        Err(error) => error!(
                            "Could not copy the path {} to clipboard. Error: {}",
                            selected_worktree.path(),
                            error
                        ),
                    }
                }
                None => debug!("No worktree was selected. Nothing to copy to clipboard"),
            },
            Err(error) => error!("Could access the clipboard. Error: {}", error),
        }
    }
}

impl From<&Worktree> for ListItem<'_> {
    fn from(value: &Worktree) -> Self {
        // TODO: only display the worktree name and repository
        ListItem::new(value.path().to_string())
    }
}

impl ListComponent<Worktree> for WorktreesComponent {
    fn filtered_items(&mut self) -> Vec<&Worktree> {
        let mut filtered_worktrees = self
            .worktrees
            .iter()
            .filter(|worktree| worktree.path().contains(self.filter.value.as_str()))
            .collect::<Vec<&Worktree>>();

        filtered_worktrees.sort_by(|w1, w2| w1.path().cmp(w2.path()));
        filtered_worktrees
    }

    fn get_state(&mut self) -> &mut ListState {
        &mut self.state
    }

    fn update_selected_index(&mut self, index: usize) {
        self.selected_index = Some(index);
    }
}
