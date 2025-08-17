use clap::Parser;
use expand_tilde;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory where the new git worktrees will be stored
    #[arg(
        short = 'd',
        long = "worktrees-dir",
        value_name = "DIR",
        env = "DEVSPACE_WORKTREES_DIR"
    )]
    // TODO: list worktrees from the repositories directly instead of getting the worktrees_dir from user
    pub worktrees_dir: String,
    /// Directory of the git respositories
    #[arg(
        short = 'r',
        long = "repos-dir",
        value_name = "DIR",
        env = "DEVSPACE_REPOS_DIR"
    )]
    pub repos_dir: String,
}

impl Args {
    pub fn new() -> Self {
        let mut args = Self::parse();
        args.repos_dir = expand_tilde::expand_tilde(&args.repos_dir)
            .expect("Could not expand the ~ in the repos_dir")
            .to_str()
            .expect("Could not convert the expanded repos_dir to a string")
            .to_string();
        args.worktrees_dir = expand_tilde::expand_tilde(&args.worktrees_dir)
            .expect("Could not expand the ~ in the worktrees_dir")
            .to_str()
            .expect("Could not convert the expanded worktrees_dir to a string")
            .to_string();
        args
    }
}
