use clap::Parser;
use log::debug;

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
    // TODO: list worktrees from the repositories directly instead of getting the worktrees_dir from user
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
        let parsed_args = Self::parse();
        debug!("Worktrees dir: {}", parsed_args.worktrees_dir);
        debug!("Repositories dir: {}", parsed_args.repos_dirs);
        parsed_args
    }
}
