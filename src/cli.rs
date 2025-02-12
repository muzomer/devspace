use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory where the devspaces are stored
    #[arg(short = 'd', long, value_name = "DIR")]
    pub spaces_dir: String,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }
}
