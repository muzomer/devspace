<h1 align="center">Devspace</h1>
<p align="center">
  CLI tool to create and manage git worktrees.
</p>

<p align="center">
  <a href="#workflow">Workflow</a> •
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#roadmap">Roadmap</a><br>
</p>

![Create New Git Worktrees](demos/create_new_worktrees.gif)

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
