# Devspace

Rust CLI tool for creating and managing git worktrees across multiple repositories. Uses a ratatui TUI, git2 for git operations, and color-eyre for error handling.

## Build & Test

```bash
cargo build                      # build
cargo test                       # run all tests
cargo clippy -- -D warnings      # lint (must pass)
cargo fmt --check                # format check (must pass)
cargo fmt                        # auto-format
```

## Project Structure

```
src/
  main.rs                        # entry point, terminal setup/teardown, event loop
  app.rs                         # App struct: holds all components, routes key events, manages Focus
  cli.rs                         # clap CLI args (--repos-dir, --worktrees-dir, --run-fetch)
  lib.rs                         # re-exports
  logs.rs                        # tracing setup
  dirs.rs                        # directory resolution helpers
  git/
    mod.rs                       # public API: list_repositories, worktrees_of_repositories
    repository.rs                # Repository wrapper around git2::Repository; worktree creation, fetch
    worktree.rs                  # Worktree wrapper; delete_worktree
  components/
    mod.rs                       # EventState enum, shared SELECTED_STYLE constant
    worktrees.rs                 # WorktreesComponent — main list view (default focus)
    repositories.rs              # RepositoriesComponent — popup for repo selection
    create_worktree.rs           # CreateWorktreeComponent — popup text input for branch name
    list.rs                      # generic list widget used by worktrees/repositories components
    filter.rs                    # filter/search logic for lists
```

## Key Concepts

- **Focus** (`app.rs`): the app has three modes — `Worktrees`, `Repositories`, `CreateWorktree`. Only one has keyboard focus at a time.
- **EventState** (`components/mod.rs`): `Consumed`, `NotConsumed`, `Exit`. Components return this from `handle_key` to indicate whether they handled the event.
- **Worktrees** are stored under `DEVSPACE_WORKTREES_DIR/<repo-name>/<branch-name>/`.
- **Repositories** are discovered by recursively scanning `DEVSPACE_REPOS_DIR` for `.git` directories.
- **`has_remote_branch`** on `Worktree` indicates whether the local branch has a tracking upstream.

## Environment Variables

| Variable | CLI flag | Description |
|---|---|---|
| `DEVSPACE_REPOS_DIR` | `--repos-dir` | Directory containing git repositories |
| `DEVSPACE_WORKTREES_DIR` | `--worktrees-dir` | Directory where worktrees are created |

## Conventions

- Use `color-eyre` for error propagation: `eyre::Result`, `.wrap_err("...")`.
- Use `tracing` macros (`debug!`, `error!`) for logging — no `println!` in library code.
- SSH agent auth is used for git fetch (`Cred::ssh_key_from_agent`). HTTPS auth is not yet implemented (see TODO in `repository.rs`).
- New TUI components should implement `draw(&mut self, frame: &mut Frame, area: Rect)` and `handle_key(&mut self, key: KeyEvent) -> EventState`.
- Tests use `tempfile::tempdir()` for filesystem isolation.

