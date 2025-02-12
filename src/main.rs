mod cli;
use std::io;

mod devspace;

fn main() -> io::Result<()> {
    let args = cli::Args::new();
    let devspaces = devspace::list(&args.spaces_dir)?;
    println!("{:#?}", devspaces);
    Ok(())
}
