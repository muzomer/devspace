<h1 align="center">Devspace</h1>
<p align="center">
  CLI tool to create and manage git worktrees.
</p>

<p align="center">
  <a href="#workflow">Workflow</a> •
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#development">Development</a> •
  <a href="#contribution">Contribution</a> •
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

Run `devspace` from any directory with the below CLI options, or define the environment variables and just run it without any CLI option:
- `--repos-dir`: the directory where the repositories are stored (or set `DEVSPACE_REPOS_DIR` env variable)
- `--worktrees-dir`: the directory where the worktrees will be stored (or set `DEVSPACE_WORKTREES_DIR` env variable).

## Keybindings

Each screen is treated as a separate mode. By default, `devspace` starts in the `Worktrees Mode`.

### Worktrees Mode

Lists the worktrees that exist under the worktrees directory.

| Keybinding | Description |
| ------------- | ------------- |
| `Ctrl + N` or `Down Arrow`  | Move down in the list |
| `Ctrl + P` or `Up Arrow`  | Move up in the list |
| `Ctrl + D` | Switch to `Repositories Mode` |
| `Ctrl + X` | Delete the selected worktree |
| `Enter` | Copy the full path of the selected worktree to the clipboard and exit |

### Repositories Mode

Lists the repositories that exist under the repositories directory.

| Keybinding | Description |
| ------------- | ------------- |
| `Ctrl + N` or `Down Arrow`  | Move down in the list |
| `Ctrl + P` or `Up Arrow`  | Move up in the list |
| `Enter` | Select the current repository and switch to the `New Worktree Mode`|

### New Worktree Mode

Creates new worktree in the selected repository.

| Keybinding | Description |
| ------------- | ------------- |
| `Enter` | Create new worktree with the provided branch name in the selected repository and switch to `Worktrees Mode` |

# Development

To enable debugging in this project, you need to set the `RUST_LOG` environment variable. The project uses the `tracing` library for logging, and the log level is configured via this environment variable.

### Set the `RUST_LOG` Environment Variable

You can set the `RUST_LOG` environment variable to control the verbosity of the logs. Here are a few examples:

- **To see all `info` level logs (the default):**
    ```bash
    export RUST_LOG=info
    ```

- **To enable `debug` level logging for all crates:**
    ```bash
    export RUST_LOG=debug
    ```

- **To enable `debug` level logging only for the `devspace` crate:**
    ```bash
    export RUST_LOG=devspace=debug
    ```

- **For the most verbose logging, you can use `trace`:**
    ```bash
    export RUST_LOG=trace
    ```

### Run the Application

After setting the environment variable, run the application as you normally would:

```bash
devspace
```

### View the Logs

The log output is written to a file named `devspace.log`. This file is located in the application's data directory. On Linux, this is typically `~/.local/share/devspace/devspace.log`.

You can view the logs in real-time by using the `tail` command:

```bash
tail -f ~/.local/share/devspace/devspace.log
```

By adjusting the `RUST_LOG` environment variable, you can control the level of detail in the logs, which is very helpful for debugging.

# Contribution

So far there are no specific rules for contributions. Pull requests are very welcome. Feel free to checkout the repo and submit new PRs for fixes or new feature requests.

# Roadmap

- [x] Create new worktrees.
- [x] Delete worktrees.
- [x] Show the status of worktrees (e.g. stale, active ...etc.).
- [ ] Create worktrees from remote branches.
- [ ] Add metadata to worktrees, e.g. JIRA links, PR links ...etc.

More features might be added in the future.
