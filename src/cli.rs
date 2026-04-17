use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct RawArgs {
    /// Directory where the new git worktrees will be stored
    #[arg(
        short = 'd',
        long = "worktrees-dir",
        value_name = "DIR",
        env = "DEVSPACE_WORKTREES_DIR"
    )]
    // TODO: list worktrees from the repositories directly instead of getting the worktrees_dir from user
    worktrees_dir: Option<String>,
    /// Directory of the git repositories (colon-separated for multiple)
    #[arg(
        short = 'r',
        long = "repos-dir",
        value_name = "DIR",
        env = "DEVSPACE_REPOS_DIR",
        num_args = 1..,
        value_delimiter = ':'
    )]
    repos_dirs: Option<Vec<String>>,

    /// Whether to run git fetch for each repo. Default: false
    #[arg(short = 'f', long = "run-fetch", value_name = "BOOLEAN")]
    run_fetch: Option<bool>,
}

#[derive(Debug)]
pub struct Args {
    pub worktrees_dir: String,
    pub repos_dirs: Vec<String>,
    pub run_fetch: bool,
}

impl Args {
    pub fn new() -> Self {
        let raw = RawArgs::parse();
        let config = crate::config::Config::load().unwrap_or_default();

        let worktrees_dir_raw = raw.worktrees_dir.or(config.worktrees_dir).expect(
            "worktrees_dir must be set via --worktrees-dir, DEVSPACE_WORKTREES_DIR, or config file",
        );

        let repos_dirs_raw = raw
            .repos_dirs
            .or(config.repos_dirs)
            .expect("repos_dirs must be set via --repos-dir, DEVSPACE_REPOS_DIR, or config file");

        let run_fetch = raw.run_fetch.or(config.run_fetch).unwrap_or(false);

        let repos_dirs = repos_dirs_raw
            .iter()
            .map(|dir| {
                std::fs::canonicalize(
                    expand_tilde::expand_tilde(dir).expect("Could not expand the ~ in a repos_dir"),
                )
                .expect("Could not resolve repos_dir to an absolute path")
                .to_str()
                .expect("Could not convert the expanded repos_dir to a string")
                .to_string()
            })
            .collect();

        let worktrees_dir = std::fs::canonicalize(
            expand_tilde::expand_tilde(&worktrees_dir_raw)
                .expect("Could not expand the ~ in the worktrees_dir"),
        )
        .expect("Could not resolve worktrees_dir to an absolute path")
        .to_str()
        .expect("Could not convert the expanded worktrees_dir to a string")
        .to_string();

        Self {
            worktrees_dir,
            repos_dirs,
            run_fetch,
        }
    }
}
