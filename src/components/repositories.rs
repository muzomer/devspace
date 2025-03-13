use super::list::ItemOrder;
use crate::git::Repository;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Clear, List, ListDirection, ListItem, ListState, Paragraph, StatefulWidget},
    Frame,
};

use super::{
    filter::FilterComponent,
    list::{Focus, ListComponent},
    EventState, SELECTED_STYLE,
};

pub struct RepositoriesComponent {
    repositories: Vec<Repository>,
    filter: FilterComponent,

    state: ListState,
    focus: Focus,
}

impl RepositoriesComponent {
    pub fn new(repositories: Vec<Repository>) -> Self {
        Self {
            repositories,
            filter: FilterComponent::default(),
            state: ListState::default().with_selected(Some(0)),
            focus: Focus::Filter,
        }
    }
    pub fn draw(&mut self, f: &mut Frame, rect: Rect) {
        f.render_widget(Clear, rect);

        let [filter_area, repos_list_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).areas(rect);

        self.filter.draw(f, filter_area);

        let list = List::new(self.filtered_items())
            .block(
                Block::bordered()
                    .title("Repositories")
                    .title_alignment(Alignment::Center),
            )
            .style(Style::new().white())
            .highlight_style(SELECTED_STYLE)
            .direction(ListDirection::TopToBottom);

        StatefulWidget::render(list, repos_list_area, f.buffer_mut(), &mut self.state);
    }
    pub fn handle_key(&mut self, key: KeyEvent) -> EventState {
        match self.focus {
            Focus::Filter => {
                let result = self.filter.handle_key(key);
                if result == EventState::Consumed {
                    self.state.select_first();
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

impl From<&Repository> for ListItem<'_> {
    fn from(value: &Repository) -> Self {
        ListItem::new(
            value
                .path
                .clone()
                .split('/')
                .last()
                .unwrap_or("")
                .to_string(),
        )
    }
}

impl ListComponent<Repository> for RepositoriesComponent {
    fn filtered_items(&mut self) -> Vec<&Repository> {
        self.repositories
            .iter()
            .filter(|worktree| worktree.path.contains(self.filter.value.as_str()))
            .collect()
    }

    fn get_state(&mut self) -> &mut ListState {
        &mut self.state
    }

    fn update_selected_index(&mut self, index: usize) {
        todo!()
    }
}
