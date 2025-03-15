use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory where the new git worktrees will be stored
    #[arg(
        short = 'd',
        long = "git-worktrees-dir",
        value_name = "DIR",
        env = "DEVSPACE_GIT_WORKTREES_DIR"
    )]
    // TODO: list worktrees from the repositories directly instead of getting the worktrees_dir from user
    pub worktrees_dir: String,
    /// Directory of the git respositories
    #[arg(
        short = 'r',
        long = "git-repos-dir",
        value_name = "DIR",
        env = "DEVSPACE_GIT_REPOS_DIR"
    )]
    pub repos_dir: String,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }
}
