use clap::{ArgGroup, Parser};

/// Create symlinks quickly and easily even if there are files there already.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
#[command(group(ArgGroup::new("overwrite").multiple(false)))]
pub struct HookArgs {
    /// The file path where you wish the real files to be.
    #[arg(short, long, required=true)]
    pub source: String,

    /// The file path where you wish the symlink files to be.
    #[arg(short, long, required=true)]
    pub destination: String,

    /// When there is the possibility for data loss, ask the user for confirmation.
    #[arg(short, long, group="overwrite")]
    pub interactive: bool,

    /// Overwrite the destination files without asking.
    #[arg(short, long, group="overwrite")]
    pub force: bool,

    /// Do not print any output except errors and required prompts.
    #[arg(short, long)]
    pub quiet: bool
}