use super::app::{App, Screen};
use crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::crossterm::event::{KeyCode, KeyEventKind};

pub enum HandleEventResult {
    Continue,
    Stop,
    NewScreen(Screen),
}

pub fn handle_event(key_event: KeyEvent, app: &mut App) -> HandleEventResult {
    if key_event.kind == KeyEventKind::Release {
        HandleEventResult::Continue
    } else if key_event.modifiers == KeyModifiers::CONTROL && key_event.code == KeyCode::Char('c') {
        HandleEventResult::Stop
    } else {
        let handle_event_result = match &app.current_screen {
            Screen::ListWorktrees => app.worktrees.handle_event(&key_event),
            Screen::ListRepos => app.repos.handle_event(&key_event),
            Screen::CreateWorktree(selected_repository) => app
                .new_worktree
                .handle_event(&key_event, selected_repository.clone()),
        };
        handle_event_result
    }
}
