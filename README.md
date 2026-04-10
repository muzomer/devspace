<h1 align="center">Devspace</h1>
<p align="center">
  CLI tool to create and manage git worktrees.
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#workflow">Workflow</a> •
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#roadmap">Roadmap</a><br>
</p>

![Features demo](demos/features.gif)

# Features

- **Worktree management** — create, delete, and navigate git worktrees across all your repositories from a single TUI.
- **Live status indicators** — each worktree shows its remote branch state at a glance:
  - `✔` remote branch exists (green)
  - `✘` branch merged or deleted remotely (red)
  - `⬆` never pushed to remote (yellow)
  - `*` dirty working tree — uncommitted changes (yellow)
- **Smart branch base** — when creating a worktree, devspace automatically bases it on a matching remote branch if one exists, or falls back to the default branch (`main`/`master`).
- **Fuzzy search** — filter worktrees and repositories in real time with a fuzzy matcher (`i` or `/` to enter filter mode).
- **GitHub PR integration** — press `p`, paste a GitHub PR URL, and devspace fetches the branch name, warns you if the PR is already merged, and opens the creation form pre-filled.
- **Auto-clone** — if the repository for a PR URL is not found locally, devspace offers to clone it via SSH before creating the worktree.
- **Copy path to clipboard** — press `Enter` on any worktree to copy its full path to the clipboard and exit, ready to `cd` into it.
- **Context-sensitive help** — press `?` from any screen to see all available keybindings for the current context.
- **Vi-style navigation** — `j`/`k`, `g`/`G`, `Tab` between filter and list; familiar and fast.

# Workflow

It simplifies working in multiple git repositories, and multiple PRs in each repository. Where each PR has a separate git worktree for ease of switching between the PRs.

The idea is to simplify context switching between open PRs by having all the git worktrees visible and manageable in single place.

Devspace assumes repositories are stored under a single directory (`DEVSPACE_REPOS_DIR`), and there is a separate directory (`DEVSPACE_WORKTREES_DIR`) for storing the worktrees.

```
.
├── repositories_dir
│   ├── backend-repo
│   ├── frontend-repo
│   └── infra-repo
└── worktrees_dir
```

The repositories directories should contain the updated trunk branch (could be `main` or `master`, or `staging`). When a new worktree is created in `devspace`, it will be based on the trunk branch checked out in the git repository directory.

Assume, there is a new feature to add a button in the UI, and that button requires a new endpoint in the backend. Worktrees can be created as below:
- In the `frontend-repo`, create new wortkree with a branch named `add-new-button-to-the-ui`.
- and, in the `backend-repo`, create new wortkree with a branch named `add-backend-api-for-the-new-button`.

When these worktrees are created by `devspace` they will be stored under the `worktrees_dir` as below:

```
└── worktrees_dir
    ├── backend-repo
    │   └── add-backend-api-for-the-new-button
    └── frontend-repo
        └── add-new-button-to-the-ui
```

To switch between the worktrees, from `devspace` select the worktree, and the full path for the worktree will be copied to the clipboard.

# Installation

Clone the repo and inside the root directory run:
`cargo install --path . --locked`

This command should install the binary in `$HOME/.cargo/bin/devspace`.

# Usage

Run `devspace` from any directory with the below CLI options, or define the environment variables and run it without any CLI option:
- `--repos-dir`: the directory where the repositories are stored (or set `DEVSPACE_REPOS_DIR` env variable)
- `--worktrees-dir`: the directory where the worktrees will be stored (or set `DEVSPACE_WORKTREES_DIR` env variable).

## Keybindings

`devspace` uses vi-style keybindings. Press `?` in the Worktrees or Repositories view to show the full keybindings reference. Press `q` or `Ctrl+C` to quit.

# Roadmap

- [x] Create new worktrees.
- [x] Delete worktrees.
- [x] Show the status of worktrees (e.g. stale, active ...etc.).
- [x] Create worktrees from remote branches.
- [ ] Add metadata to worktrees, e.g. JIRA links, PR links ...etc.
