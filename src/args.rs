use clap::Parser;

/// Create symlinks quickly and easily even if there are files there already.
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct HookArgs {
    /// The file path where you wish the real files to be.
    #[arg(short, long)]
    pub source: String,

    /// The file path where you wish the symlink files to be.
    #[arg(short, long)]
    pub destination: String,

    /// Move files from the destination path to the source path and overwrite if they exist in the source directory.
    #[arg(short, long)]
    pub force: bool,
}