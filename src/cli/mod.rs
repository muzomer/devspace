use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory where the devspaces are stored
    #[arg(
        short = 'd',
        long = "git-worktrees-dir",
        value_name = "DIR",
        env = "DEVSPACE_GIT_WORKTREES_DIR"
    )]
    pub spaces_dir: String,
    /// Directory of the git respositories
    #[arg(
        short = 'r',
        long = "git-repos-dir",
        value_name = "DIR",
        env = "DEVPSACE_GIT_REPOS_DIR"
    )]
    pub repos_dirs: String,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }
}
