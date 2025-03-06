use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Rect,
    style::{palette::tailwind::SLATE, Modifier, Style, Stylize},
    widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget},
    Frame,
};

use crate::git::Worktree;

use super::list::ListComponent;
use super::{filter::FilterComponent, EventState};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub struct WorktreesComponent {
    worktrees: Vec<Worktree>,
    filter: FilterComponent,
    state: ListState,
}

impl WorktreesComponent {
    pub fn new(worktrees: Vec<Worktree>) -> Self {
        Self {
            worktrees,
            filter: FilterComponent::default(),
            state: ListState::default(),
        }
    }

    pub fn draw(&mut self, f: &mut Frame, rect: Rect) {
        let block = Block::new().borders(Borders::all());
        let list = List::new(self.filtered_items())
            .block(block)
            .style(Style::new().white())
            .highlight_style(SELECTED_STYLE)
            .direction(ratatui::widgets::ListDirection::TopToBottom);
        StatefulWidget::render(list, rect, f.buffer_mut(), &mut self.state);
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> EventState {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            KeyCode::Tab => self.select_next(),
            _ => return EventState::NotConsumed,
        }

        EventState::Consumed
    }
}

impl From<&Worktree> for ListItem<'_> {
    fn from(value: &Worktree) -> Self {
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
}
