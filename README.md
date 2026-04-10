<h1 align="center">Devspace</h1>
<p align="center">
  CLI tool to create and manage git worktrees in multiple repositories.
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

- **Manage worktrees** — create, delete, and navigate git worktrees across different repositories.
- **Worktree status indicators** — each worktree shows its remote branch state:
  - `✔` remote branch exists (green)
  - `✘` branch merged or deleted remotely (red)
  - `⬆` never pushed to remote
  - `*` dirty working tree — uncommitted changes
- **Create worktrees from PR links** — paste a GitHub PR URL and devspace clones the repo and creates worktree from the PR branch (requires `gh` CLI or read-only `GITHUB_TOKEN`).
- **Vi-style navigation**

# Rationale

It simplifies working in multiple git repositories, and multiple PRs in each repository. Where each PR has a separate git worktree for ease of switching between the PRs.

The idea is to simplify context switching between open PRs by having all the git worktrees visible and manageable in single place.

`devspace` assumes repositories are stored under a single directory (`DEVSPACE_REPOS_DIR`), and there is a separate directory (`DEVSPACE_WORKTREES_DIR`) for storing the worktrees.

```
.
├── repositories_dir
│   ├── backend-repo
│   ├── frontend-repo
│   └── infra-repo
└── worktrees_dir
```

Assume, there is a new feature to add a button in the UI, and that button requires a new endpoint in the backend. Worktrees can be created as below:
- In the `frontend-repo`, create new wortkree with a branch named `add-new-button-to-the-ui`.
- and, in the `backend-repo`, create new wortkree with a branch named `add-backend-api-for-the-new-button`.

When these worktrees are created in `devspace` they will be stored under the `worktrees_dir` as below:

```
└── worktrees_dir
    ├── backend-repo
    │   └── add-backend-api-for-the-new-button
    └── frontend-repo
        └── add-new-button-to-the-ui
```

To switch between the worktrees, run `cd $(devspace)` to go the directory of the selected worktree.

# Installation

Download the binary from the releases or clone the repo and inside the root directory run:
`cargo install --path . --locked`

Typicall, the binary will be installed in `$HOME/.cargo/bin/devspace`.

# Usage

Run `cd $(devspace)` in `bash`/`zsh` or `cd (devspace)` in `fish` shell from any directory with the below CLI options, or define the environment variables and run it without any CLI option:
- `--repos-dir`: the directory where the repositories are stored (or set `DEVSPACE_REPOS_DIR` env variable)
- `--worktrees-dir`: the directory where the worktrees will be stored (or set `DEVSPACE_WORKTREES_DIR` env variable).

## Keybindings

`devspace` uses vi-style keybindings. Check them with `?`

# Roadmap

- [x] Create new worktrees.
- [x] Delete worktrees.
- [x] Show the status of worktrees (e.g. stale, active ...etc.).
- [x] Create worktrees from remote branches.
- [ ] Create PRs from worktrees.
- [ ] Add metadata to worktrees, e.g. JIRA links, PR links ...etc.
