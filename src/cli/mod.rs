use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory where the git worktrees are stored
    #[arg(
        short = 'd',
        long = "git-worktrees-dir",
        value_name = "DIR",
        env = "DEVSPACE_GIT_WORKTREES_DIR"
    )]
    pub worktrees_dir: String,
    /// Directory of the git respositories
    #[arg(
        short = 'r',
        long = "git-repos-dir",
        value_name = "DIR",
        env = "DEVSPACE_GIT_REPOS_DIR"
    )]
    pub repos_dirs: String,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }
}
