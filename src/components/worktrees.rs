use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget},
    Frame,
};

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

    pub selected_index: Option<usize>,
}

impl WorktreesComponent {
    pub fn new(worktrees: Vec<Worktree>) -> Self {
        Self {
            worktrees,
            filter: FilterComponent::default(),
            state: ListState::default().with_selected(Some(0)),
            focus: Focus::Filter,
            selected_index: None,
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
                    _ => return EventState::NotConsumed,
                }
                EventState::Consumed
            }
        }
    }
}

impl From<&Worktree> for ListItem<'_> {
    fn from(value: &Worktree) -> Self {
        // TODO: only display the worktree name and repository
        ListItem::new(value.path.clone().to_string())
    }
}

impl ListComponent<Worktree> for WorktreesComponent {
    fn filtered_items(&mut self) -> Vec<&Worktree> {
        self.worktrees
            .iter()
            .filter(|worktree| worktree.path.contains(self.filter.value.as_str()))
            .collect()
    }

    fn get_state(&mut self) -> &mut ListState {
        &mut self.state
    }

    fn update_selected_index(&mut self, index: usize) {
        self.selected_index = Some(index);
    }
}
