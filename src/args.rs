use clap::{ArgGroup, Parser};

/// Create symlinks quickly and easily even if there are files there already.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
#[command(group(ArgGroup::new("overwrite").multiple(false)))]
#[command(group(ArgGroup::new("logging").multiple(false)))]
pub struct HookArgs {
    /// The file path where you wish the real files to be.
    #[arg(short, long, required = true)]
    pub source: String,

    /// The file path where you wish the symlink files to be.
    #[arg(short, long, required = true)]
    pub destination: String,

    /// Set symlinks as relative to the destination file.
    #[arg(short, long)]
    pub relative: bool,

    /// When there is the possibility for data loss, ask the user for confirmation.
    #[arg(short, long, group = "overwrite")]
    pub interactive: bool,

    /// Overwrite the destination files without asking.
    #[arg(short, long, group = "overwrite")]
    pub force: bool,

    /// Do not print any output except errors and required prompts.
    #[arg(short, long, group = "logging")]
    pub quiet: bool,

    /// Print more information about the operation.
    #[arg(short, long, group = "logging")]
    pub verbose: bool,

    /// Don't create symlinks, just print what would be done.
    #[arg(long)]
    pub dry_run: bool,
}
