use nucleo_matcher::{
    pattern::{CaseMatching, Normalization, Pattern},
    Config, Matcher, Utf32Str,
};

use super::list::ItemOrder;
use crate::git::Repository;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Clear, List, ListDirection, ListItem, ListState, StatefulWidget},
    Frame,
};

use super::{
    filter::FilterComponent,
    list::{Focus, ListComponent},
    Action, EventState, SELECTED_STYLE,
};

pub struct RepositoriesComponent {
    repositories: Vec<Repository>,
    filter: FilterComponent,
    state: ListState,
    selected_index: Option<usize>,
    focus: Focus,
}

impl RepositoriesComponent {
    pub fn new(repositories: Vec<Repository>) -> Self {
        Self {
            repositories,
            filter: FilterComponent::new(" Filter Repositories ".to_string()),
            state: ListState::default().with_selected(Some(0)),
            selected_index: Some(0),
            focus: Focus::Filter,
        }
    }

    pub fn draw(&mut self, f: &mut Frame, rect: Rect) {
        f.render_widget(Clear, rect);
        let [filter_area, repos_list_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).areas(rect);
        self.filter
            .draw(f, filter_area, matches!(self.focus, Focus::Filter));
        let list = List::new(self.filtered_items())
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(super::BORDER_STYLE)
                    .title_alignment(Alignment::Center),
            )
            .style(Style::new().white())
            .highlight_style(SELECTED_STYLE)
            .direction(ListDirection::TopToBottom);
        StatefulWidget::render(list, repos_list_area, f.buffer_mut(), &mut self.state);
    }

    pub fn handle_action(&mut self, action: Action) -> EventState {
        match action {
            Action::MoveDown => {
                self.select(ItemOrder::Next);
                EventState::Consumed
            }
            Action::MoveUp => {
                self.select(ItemOrder::Previous);
                EventState::Consumed
            }
            Action::GoFirst => {
                self.select(ItemOrder::First);
                EventState::Consumed
            }
            Action::GoLast => {
                self.select(ItemOrder::Last);
                EventState::Consumed
            }
            Action::InsertChar(c) => {
                self.filter.enter_char(c);
                self.select(ItemOrder::First);
                EventState::Consumed
            }
            Action::DeleteChar => {
                self.filter.delete_char();
                self.select(ItemOrder::First);
                EventState::Consumed
            }
            _ => EventState::NotConsumed,
        }
    }

    pub fn focus_filter(&mut self) {
        self.focus = Focus::Filter;
    }

    pub fn focus_list(&mut self) {
        self.focus = Focus::List;
    }

    pub fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Filter => Focus::List,
            Focus::List => Focus::Filter,
        };
    }

    pub fn is_filter_focused(&self) -> bool {
        matches!(self.focus, Focus::Filter)
    }

    pub fn selected_repository(&mut self) -> Option<&Repository> {
        match self.selected_index {
            Some(index) => {
                let filtered_repositories = self.filtered_items();
                match filtered_repositories.get(index) {
                    Some(selected_repository) => Some(selected_repository),
                    None => None,
                }
            }
            None => None,
        }
    }
}

impl From<&Repository> for ListItem<'_> {
    fn from(value: &Repository) -> Self {
        ListItem::new(value.name())
    }
}

impl ListComponent<Repository> for RepositoriesComponent {
    fn filtered_items(&mut self) -> Vec<&Repository> {
        let query = self.filter.value.as_str();
        if query.is_empty() {
            let mut items: Vec<&Repository> = self.repositories.iter().collect();
            items.sort_by_key(|a| a.name());
            return items;
        }
        let mut matcher = Matcher::new(Config::DEFAULT);
        let pattern = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart);
        let mut buf = Vec::new();
        let min_score: u32 = 70;
        let mut scored: Vec<(&Repository, u32)> = self
            .repositories
            .iter()
            .filter_map(|r| {
                pattern
                    .score(Utf32Str::new(&r.name(), &mut buf), &mut matcher)
                    .filter(|&s| s >= min_score)
                    .map(|s| (r, s))
            })
            .collect();
        scored.sort_by(|a, b| b.1.cmp(&a.1));
        scored.into_iter().map(|(r, _)| r).collect()
    }

    fn get_state(&mut self) -> &mut ListState {
        &mut self.state
    }

    fn update_selected_index(&mut self, index: usize) {
        self.selected_index = Some(index);
    }
}
