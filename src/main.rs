use std::path::Path;
use error::HookError;
use symlink::symlink_auto;

use args::HookArgs;
use clap::Parser;

mod args;
mod error;

// Rules:
// A symlink should no be created if the file already is a symlink in the destination path.
// A symlink should be created at the destination path if there are no files there, but there are files at the source path.
// A symlink should be created at the destination path if there are files there, there are no files at the source path.
//     This will move files.
// A symlink should be created at the destination path if there are files there, there are files at the source path.
//     This will error unless the force flag is set.
// If the source path is a file, the destination path should be a file.
// If the source path is a directory, the destination path should be a directory.
//     This will follow the above rules for each file in the immediate directory.
//     If the recursive flag is set, this will follow the above rules for each file in the directory and its subdirectories.

fn main() {
    let result = run_program();

    if let Err(err) = result {
        eprintln!("{}", err);
    }
}

fn run_program() -> Result<(), HookError> {
    let args = HookArgs::parse();
    let source = args.source;
    let destination = args.destination;
    let recursive = args.recursive;
    let force = args.force;

    let source = Path::new(&source);
    let destination = Path::new(&destination);

    match source.is_file() {
        true => {
            if !destination.is_file() {
                return Err(HookError::ExecutionError("The destination path must be a file if the source path is a file.".to_string()));
            }
        },
        false => {
            if destination.is_file() {
                return Err(HookError::ExecutionError("The destination path must be a directory if the source path is a directory.".to_string()));
            }
        },
    };

    

    Ok(())
}

fn create_symlink(source: &Path, destination: &Path, recursive: bool, force: bool) -> Result<(), HookError> {
    if destination.is_symlink() {
        return Err(HookError::ExecutionError("The destination path is already a symlink.".to_string()));
    }

    let symlink = symlink_auto(source, destination).map_err(|err| {
        HookError::SymlinkCreationError(err)
    })?;


    Ok(())
}
