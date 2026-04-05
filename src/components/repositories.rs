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
        self.filter.draw(f, filter_area);
        let list = List::new(self.filtered_items())
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
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
        let mut filtered_repositories = self
            .repositories
            .iter()
            .filter(|repository| repository.name().contains(self.filter.value.as_str()))
            .collect::<Vec<&Repository>>();

        filtered_repositories.sort_by(|r1, r2| {
            let r2_name = r2.name();
            r1.name().cmp(&r2_name)
        });

        filtered_repositories
    }

    fn get_state(&mut self) -> &mut ListState {
        &mut self.state
    }

    fn update_selected_index(&mut self, index: usize) {
        self.selected_index = Some(index);
    }
}
