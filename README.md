<h1 align="center">Shanti</h1>
<p align="center"><i>(Shanti: means peace of mind)</i></p>
<p align="center">
  CLI tool to create and manage git worktrees in multiple repositories.
</p>

<p align="center">
  <a href="#features">Features</a> •
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
- **Create worktrees from PR links** — paste a GitHub PR URL and shanti clones the repo and creates worktree from the PR branch (requires `gh` CLI or read-only `GITHUB_TOKEN`).
- **Vi-style navigation**

# Rationale

It simplifies working in multiple git repositories, and multiple PRs in each repository. Where each PR has a separate git worktree for ease of switching between the PRs.

The idea is to simplify context switching between open PRs by having all the git worktrees visible and manageable in single place.

`shanti` scans one or more repositories directories (`SHANTI_REPOS_DIR`) for git repos, and stores worktrees under a separate directory (`SHANTI_WORKTREES_DIR`).

```
.
├── work_repos_dir/           # work git repositories
│   ├── backend-repo/
│   └── frontend-repo/
├── personal_repos_dir/       # personal git repositories
│   └── side-project/
└── worktrees_dir/            # worktrees managed by shanti
```

Assume, there is a new feature to add a button in the UI, and that button requires a new endpoint in the backend. Worktrees can be created as below:
- In the `frontend-repo`, create new wortkree with a branch named `add-new-button-to-the-ui`.
- and, in the `backend-repo`, create new wortkree with a branch named `add-backend-api-for-the-new-button`.

When these worktrees are created in `shanti` they will be stored under the `worktrees_dir` as below:

```
└── worktrees_dir/
    ├── backend-repo/
    │   └── add-backend-api-for-the-new-button/   # checked-out worktree
    │       ├── src/
    │       └── ...
    └── frontend-repo/
        └── add-new-button-to-the-ui/             # checked-out worktree
            ├── src/
            └── ...
```

To switch between the worktrees, run `cd $(shanti)` to go the directory of the selected worktree.

# Installation

Download the binary from the releases or clone the repo and inside the root directory run:
`cargo install --path . --locked`

Typicall, the binary will be installed in `$HOME/.cargo/bin/shanti`.

# Usage

Run `cd $(shanti)` in `bash`/`zsh` or `cd (shanti)` in `fish` shell from any directory with the below CLI options, or define the environment variables and run it without any CLI option:
- `--repos-dir`: one or more directories where repositories are stored, colon-separated (or set `SHANTI_REPOS_DIR` env variable, e.g. `/path/a:/path/b`). Can be repeated: `--repos-dir /a --repos-dir /b`
- `--worktrees-dir`: the directory where the worktrees will be stored (or set `SHANTI_WORKTREES_DIR` env variable).

## Keybindings

`shanti` uses vi-style keybindings. Check them with `?`

# Roadmap

- [x] Create new worktrees.
- [x] Delete worktrees.
- [x] Show the status of worktrees (e.g. stale, active ...etc.).
- [x] Create worktrees from remote branches.
- [ ] Create PRs from worktrees.
- [ ] Add metadata to worktrees, e.g. JIRA links, PR links ...etc.
