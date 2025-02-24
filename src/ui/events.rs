use super::app::{App, CurrentScreen, ListingScreenMode};
use crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::crossterm::event::{KeyCode, KeyEventKind};

pub enum HandleEventResult {
    Continue,
    Stop,
}

pub fn handle_event(key_event: KeyEvent, app: &mut App) -> HandleEventResult {
    if key_event.kind == KeyEventKind::Release {
        HandleEventResult::Continue
    } else if key_event.modifiers == KeyModifiers::CONTROL && key_event.code == KeyCode::Char('c') {
        HandleEventResult::Stop
    } else {
        match app.current_screen {
            CurrentScreen::ListWorktrees(screen_mode) => {
                list_worktrees_screen(&key_event, &screen_mode, app)
            }
            CurrentScreen::ListRepos(screen_mode) => {
                list_repos_screen(&key_event, &screen_mode, app)
            }
        }
    }
}

fn list_worktrees_screen(
    key_event: &KeyEvent,
    screen_mode: &ListingScreenMode,
    app: &mut App,
) -> HandleEventResult {
    match screen_mode {
        ListingScreenMode::Navigating => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                match key_event.code {
                    KeyCode::Char('f') => {
                        app.current_screen =
                            CurrentScreen::ListWorktrees(ListingScreenMode::Filtering)
                    }
                    KeyCode::Char('n') => {
                        app.worktrees.select_next();
                    }
                    KeyCode::Char('p') => {
                        app.worktrees.select_previous();
                    }
                    _ => {}
                }
            } else {
                match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc => return HandleEventResult::Stop,
                    KeyCode::Char('j') | KeyCode::Down => app.worktrees.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => app.worktrees.select_previous(),
                    KeyCode::Char('g') | KeyCode::Home => app.worktrees.select_first(),
                    KeyCode::Char('G') | KeyCode::End => app.worktrees.select_last(),
                    KeyCode::Char('n') => {
                        app.current_screen = CurrentScreen::ListRepos(ListingScreenMode::Filtering)
                    }
                    KeyCode::Enter => {
                        app.go_to_worktree();
                    }
                    KeyCode::Tab => app.worktrees.select_next(),
                    _ => {}
                }
            }
        }
        ListingScreenMode::Filtering => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                match key_event.code {
                    KeyCode::Char('d') => {
                        app.current_screen = CurrentScreen::ListRepos(ListingScreenMode::Filtering)
                    }
                    KeyCode::Char('n') => {
                        app.worktrees.select_next();
                    }
                    KeyCode::Char('p') => {
                        app.worktrees.select_previous();
                    }
                    _ => {}
                }
            } else {
                match key_event.code {
                    KeyCode::Esc => return HandleEventResult::Stop,
                    KeyCode::Tab => {
                        app.worktrees.select_next();
                    }
                    KeyCode::Backspace => {
                        app.worktrees.delete_char();
                        app.worktrees.update_filtered_items();
                        app.worktrees.select_first();
                    }
                    KeyCode::Enter => {
                        app.current_screen =
                            CurrentScreen::ListWorktrees(ListingScreenMode::Navigating);
                        app.go_to_worktree();
                        return HandleEventResult::Stop;
                    }
                    KeyCode::Char(to_insert) => {
                        app.worktrees.enter_char(to_insert);
                        app.worktrees.update_filtered_items();
                        app.worktrees.select_first();
                    }
                    _ => {}
                }
            }
        }
    }

    HandleEventResult::Continue
}

fn list_repos_screen(
    key_event: &KeyEvent,
    screen_mode: &ListingScreenMode,
    app: &mut App,
) -> HandleEventResult {
    if key_event.modifiers == KeyModifiers::CONTROL {
        match key_event.code {
            KeyCode::Char('n') => {
                app.repos.select_next();
            }
            KeyCode::Char('p') => {
                app.repos.select_previous();
            }
            _ => {}
        }
    } else {
        match screen_mode {
            ListingScreenMode::Filtering => match key_event.code {
                KeyCode::Esc => {
                    app.current_screen = CurrentScreen::ListWorktrees(ListingScreenMode::Filtering)
                }
                KeyCode::Tab => {
                    app.repos.select_next();
                    app.current_screen = CurrentScreen::ListRepos(ListingScreenMode::Navigating)
                }
                KeyCode::Char(to_insert) => {
                    app.repos.enter_char(to_insert);
                    app.repos.update_filtered_items();
                    app.repos.select_first();
                }
                KeyCode::Backspace => {
                    app.repos.delete_char();
                    app.repos.update_filtered_items();
                    app.repos.select_first();
                }
                _ => {}
            },
            ListingScreenMode::Navigating => match key_event.code {
                KeyCode::Tab => app.repos.select_next(),
                KeyCode::Char('q') | KeyCode::Esc => {
                    app.current_screen = CurrentScreen::ListWorktrees(ListingScreenMode::Filtering)
                }
                KeyCode::Char('j') | KeyCode::Down => app.repos.select_next(),
                KeyCode::Char('k') | KeyCode::Up => app.repos.select_previous(),
                KeyCode::Char('g') | KeyCode::Home => app.repos.select_first(),
                KeyCode::Char('G') | KeyCode::End => app.repos.select_last(),
                _ => {}
            },
        }
    }
    HandleEventResult::Continue
}
