use crate::git::{self, RemoteStatus};
use color_eyre::eyre;
use nucleo_matcher::{
    pattern::{CaseMatching, Normalization, Pattern},
    Config, Matcher, Utf32Str,
};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{
        Block, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState,
        StatefulWidget,
    },
    Frame,
};

use super::{filter::FilterComponent, Action, EventState};
use super::{
    list::{Focus, ItemOrder, ListComponent},
    SELECTED_STYLE,
};
use crate::keymap::InputMode;

pub struct WorktreesComponent {
    worktrees: Vec<git::Worktree>,
    filter: FilterComponent,
    state: ListState,
    focus: Focus,
    selected_index: Option<usize>,
    pub last_error: Option<String>,
    worktrees_dir: String,
}

impl WorktreesComponent {
    pub fn new(worktrees: Vec<git::Worktree>, worktrees_dir: String) -> WorktreesComponent {
        let selected_index = if worktrees.is_empty() { None } else { Some(0) };
        Self {
            filter: FilterComponent::new(" Filter Worktrees ".to_string()),
            state: ListState::default().with_selected(selected_index),
            focus: Focus::Filter,
            selected_index,
            worktrees_dir: worktrees_dir.trim_end_matches('/').to_string(),
            worktrees,
            last_error: None,
        }
    }

    pub fn draw(&mut self, f: &mut Frame, rect: Rect, mode: InputMode, is_active: bool) {
        let worktrees_dir = self.worktrees_dir.clone();

        // Collect (remote_status, is_dirty, path) as owned data so the mutable borrow from
        // filtered_items() fully ends before we access other fields of self.
        let display_data: Vec<(RemoteStatus, bool, String)> = {
            let filtered = self.filtered_items();
            filtered
                .iter()
                .map(|wt| (wt.remote_status, wt.is_dirty, wt.path().to_string()))
                .collect()
        };
        let total = display_data.len();
        let items: Vec<ListItem<'static>> = display_data
            .iter()
            .map(|(remote_status, is_dirty, path)| {
                worktree_to_list_item(*remote_status, *is_dirty, path, &worktrees_dir)
            })
            .collect();

        let current = self.selected_index.map(|i| i + 1).unwrap_or(0);

        let mode_indicator = match mode {
            InputMode::Normal => Line::from(" NORMAL ").cyan().bold(),
            InputMode::Insert => Line::from(" INSERT ").yellow().bold(),
        };

        let bottom_left = match &self.last_error {
            Some(err) => Line::from(format!(" {} ", err)).red().bold(),
            None => mode_indicator,
        };

        let mut block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(super::BORDER_STYLE)
            .title(format!(" Worktrees ({}/{}) ", current, total))
            .title_bottom(bottom_left);

        if matches!(mode, InputMode::Normal) {
            block = block.title_bottom(Line::from(" ? help ").dark_gray().right_aligned());
        }

        let [filter_area, list_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(rect);

        self.filter.draw(
            f,
            filter_area,
            is_active && matches!(mode, InputMode::Insert) && matches!(self.focus, Focus::Filter),
        );

        let list = List::new(items)
            .block(block)
            .style(Style::new().white())
            .highlight_style(SELECTED_STYLE)
            .direction(ratatui::widgets::ListDirection::TopToBottom);

        StatefulWidget::render(list, list_area, f.buffer_mut(), &mut self.state);

        let mut scroll_state = ScrollbarState::new(total).position(self.state.offset());
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None);
        f.render_stateful_widget(scrollbar, list_area, &mut scroll_state);
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
            Action::Select => {
                if self.selected_worktree_path().is_some() {
                    EventState::Exit
                } else {
                    EventState::Consumed
                }
            }
            Action::InsertChar(c) => {
                self.filter.enter_char(c);
                EventState::Consumed
            }
            Action::DeleteChar => {
                self.filter.delete_char();
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

    /// Clears any active filter, finds the worktree matching the given branch name,
    /// and selects it. Returns `true` if found, `false` otherwise.
    pub fn select_worktree_by_branch(&mut self, branch: &str) -> bool {
        let exists = self.worktrees.iter().any(|wt| wt.name() == branch);
        if !exists {
            return false;
        }
        self.filter.clear();
        let index = self
            .filtered_items()
            .iter()
            .position(|wt| wt.name() == branch);
        if let Some(idx) = index {
            self.selected_index = Some(idx);
            self.state.select(Some(idx));
            true
        } else {
            false
        }
    }

    pub fn add(&mut self, new_worktree: git::Worktree) {
        let new_worktree_path = new_worktree.path().to_string();
        self.worktrees.push(new_worktree);
        let new_worktree_index = self
            .filtered_items()
            .iter()
            .position(|wt| wt.path().to_string().eq(&new_worktree_path));

        self.state.select(new_worktree_index);
        self.selected_index = new_worktree_index;
    }

    pub fn delete_selected_worktree(&mut self) -> eyre::Result<()> {
        if let Some(path) = self.selected_worktree_path() {
            if let Some(index) = self.worktrees.iter().position(|w| w.path() == path) {
                let result = git::delete_worktree(&self.worktrees[index]);
                self.worktrees.remove(index);
                result?;
            }
        }
        Ok(())
    }

    pub fn selected_worktree_path(&mut self) -> Option<String> {
        self.selected_index.and_then(|index| {
            self.filtered_items()
                .get(index)
                .map(|wt| wt.path().to_string())
        })
    }

}

fn worktree_to_list_item(
    remote_status: RemoteStatus,
    is_dirty: bool,
    path: &str,
    worktrees_dir: &str,
) -> ListItem<'static> {
    let (remote_indicator, indicator_color) = match remote_status {
        RemoteStatus::Exists => ("✔", Color::Green),
        RemoteStatus::Gone => ("✘", Color::Red),
        RemoteStatus::NeverPushed => ("⬆", Color::Yellow),
    };

    let indicator_span = Span::styled(
        format!("{} ", remote_indicator),
        Style::default()
            .fg(indicator_color)
            .add_modifier(Modifier::BOLD),
    );

    let path = path.trim_end_matches('/');
    let relative = path
        .strip_prefix(worktrees_dir)
        .unwrap_or(path)
        .trim_start_matches('/');

    let line = if let Some(sep) = relative.find('/') {
        let repo = &relative[..sep];
        let branch = relative[sep + 1..].trim_end_matches('/');
        let repo_span = Span::styled(repo.to_string(), Style::default().fg(Color::DarkGray));
        let sep_span = Span::styled(" / ", Style::default().fg(Color::DarkGray));
        let branch_span = Span::styled(
            branch.to_string(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );
        if is_dirty {
            let dirty_span = Span::styled("*", Style::default().fg(Color::Yellow));
            Line::from(vec![
                indicator_span,
                repo_span,
                sep_span,
                branch_span,
                dirty_span,
            ])
        } else {
            Line::from(vec![indicator_span, repo_span, sep_span, branch_span])
        }
    } else {
        let path_span = Span::from(relative.to_string());
        if is_dirty {
            let dirty_span = Span::styled("*", Style::default().fg(Color::Yellow));
            Line::from(vec![indicator_span, path_span, dirty_span])
        } else {
            Line::from(vec![indicator_span, path_span])
        }
    };

    ListItem::new(line)
}

impl ListComponent<git::Worktree> for WorktreesComponent {
    fn filtered_items(&mut self) -> Vec<&git::Worktree> {
        let query = self.filter.value.as_str();
        if query.is_empty() {
            let mut items: Vec<&git::Worktree> = self.worktrees.iter().collect();
            items.sort_by(|a, b| a.path().cmp(b.path()));
            return items;
        }
        let worktrees_dir = self.worktrees_dir.as_str();
        let mut matcher = Matcher::new(Config::DEFAULT);
        // Pair each word with its per-word minimum score threshold.
        // Short words (1-2 chars) have low scores due to gap penalties on
        // longer haystacks, so we accept any match for them.
        let patterns: Vec<(Pattern, u32)> = query
            .split_whitespace()
            .map(|w| {
                let min = if w.len() >= 3 { 70 } else { 1 };
                (
                    Pattern::parse(w, CaseMatching::Ignore, Normalization::Smart),
                    min,
                )
            })
            .collect();
        let mut buf = Vec::new();
        let mut scored: Vec<(&git::Worktree, u32)> = self
            .worktrees
            .iter()
            .filter_map(|wt| {
                let path = wt.path().trim_end_matches('/');
                let display = path
                    .strip_prefix(worktrees_dir)
                    .unwrap_or(path)
                    .trim_start_matches('/');
                let mut total = 0u32;
                for (pattern, min_score) in &patterns {
                    match pattern.score(Utf32Str::new(display, &mut buf), &mut matcher) {
                        Some(s) if s >= *min_score => total += s,
                        _ => return None,
                    }
                }
                Some((wt, total))
            })
            .collect();
        scored.sort_by(|a, b| b.1.cmp(&a.1));
        scored.into_iter().map(|(wt, _)| wt).collect()
    }

    fn get_state(&mut self) -> &mut ListState {
        &mut self.state
    }

    fn update_selected_index(&mut self, index: usize) {
        self.selected_index = Some(index);
    }
}
