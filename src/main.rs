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
//     This will create a symlink directory.

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
    let force = args.force;

    let source = Path::new(&source);
    let destination = Path::new(&destination);

    if source.file_name() != destination.file_name() {
        return Err(HookError::DifferentNames);
    }

    if source.extension().is_some() {
        if destination.extension().is_none() {
            return Err(HookError::ExecutionError("The destination path must be a file if the source path is a file.".to_string()));
        }
        
        create_symlink_file(source, destination, force)?;
    } else {
        if destination.extension().is_some() {
            return Err(HookError::ExecutionError("The destination path must be a directory if the source path is a directory.".to_string()));
        }

        create_symlink_directory(source, destination, force)?;
    }

    Ok(())
}

// A symlink should not be created if the file already is a symlink in the destination path.
// A symlink should be created at the destination path if there are no files there, but there are files at the source path.
// A symlink should be created at the destination path if there are files there and there are no files at the source path.
//     This will move files.
// A symlink should be created at the destination path if there are files there, there are files at the source path.
//     This will error unless the force flag is set.
fn create_symlink_file(source: &Path, destination: &Path, force: bool) -> Result<(), HookError> {
    // A symlink should not be created if the file already is a symlink in the destination path.
    if destination.is_symlink() {
        return Err(HookError::Skipping("The destination path is already a symlink.".to_string()));
    }
    
    assert!(source.extension().is_some());
    assert!(destination.extension().is_some());

    match (destination.exists(), source.exists()) {
        // A symlink should be created at the destination path if there isn't a file there, but there is a file at the source path.
        (false, true) => {
            create_symlink(source, destination)
        },
        // A symlink should be created at the destination path if there is a file there and there isn't a file at the source path.
        //     This will move the file.
        (true, false) => {
            move_file(destination, source)?;
            create_symlink(source, destination)
        },
        // A symlink should be created at the destination path if there is a file there and there is a file at the source path.
        //     This will error unless the force flag is set.
        (true, true) => {
            if !force {
                return Err(HookError::FilesAlreadyExists);
            }

            println!("The source and destination paths both exist.");
            println!("Which do you wish to keep? (s/d) OR (n) to cancel.");

            let mut input = String::new();
            loop {
                match std::io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        match input.trim() {
                            "s" | "S" => { // Keeping source
                                remove_file(destination)?;
                                break;
                            },
                            "d" | "D" => { // Keeping destination
                                remove_file(source)?;
                                move_file(destination, source)?;
                                break;
                            },
                            "n" | "N" => { // Cancel
                                return Err(HookError::CancelledByUser);
                            },
                            _ => {
                                println!("Invalid input. Please enter 's', 'd', or 'n'.");
                                input.clear();
                            },
                        }
                    
                    },
                    Err(err) => {
                        println!("Error reading input: {}", err);
                    },
                }
            }

            create_symlink(source, destination)
        },
        (false, false) => {
            // This should never happen.
            return Err(HookError::ExecutionError("The source and destination paths do not exist.".to_string()));
        },
    }
}

fn create_symlink_directory(source: &Path, destination: &Path, force: bool) -> Result<(), HookError> {
    // A symlink should not be created if the file already is a symlink in the destination path.
    if destination.is_symlink() {
        return Err(HookError::Skipping("The destination path is already a symlink.".to_string()));
    }

    assert!(source.is_dir());
    assert!(destination.is_dir());

    match (destination.exists(), source.exists()) {
        // A symlink should be created at the destination path if there isn't a file there, but there is a file at the source path.
        (false, true) => {
            create_symlink(source, destination)
        },
        // A symlink should be created at the destination path if there is a file there and there isn't a file at the source path.
        //     This will move the file.
        (true, false) => {
            move_file(destination, source)?;
            create_symlink(source, destination)
        },
        // A symlink should be created at the destination path if there is a file there and there is a file at the source path.
        //     This will error unless the force flag is set.
        (true, true) => {
            if !force {
                return Err(HookError::FilesAlreadyExists);
            }

            println!("The source and destination paths both exist.");
            println!("Which do you wish to keep? (s/d) OR (n) to cancel.");

            let mut input = String::new();
            loop {
                match std::io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        match input.trim() {
                            "s" | "S" => { // Keeping source
                                remove_file(destination)?;
                                break;
                            },
                            "d" | "D" => { // Keeping destination
                                remove_file(source)?;
                                move_file(destination, source)?;
                                break;
                            },
                            "n" | "N" => { // Cancel
                                return Err(HookError::CancelledByUser);
                            },
                            _ => {
                                println!("Invalid input. Please enter 's', 'd', or 'n'.");
                                input.clear();
                            },
                        }
                    
                    },
                    Err(err) => {
                        println!("Error reading input: {}", err);
                    },
                }
            }

            create_symlink(source, destination)
        },
        (false, false) => {
            // This should never happen.
            return Err(HookError::ExecutionError("The source and destination paths do not exist.".to_string()));
        },
    }
}

fn remove_file(path: &Path) -> Result<(), HookError> {
    println!("Removing file: {:?}", path);

    std::fs::remove_file(path).map_err(|err| {
        HookError::ExecutionError(format!("Error removing file: {}", err))
    })
}

fn move_file(from: &Path, to: &Path) -> Result<(), HookError> {
    println!("Moving file: {:?} to {:?}", from, to);

    std::fs::rename(from, to).map_err(|err| {
        HookError::ExecutionError(format!("Error moving file: {}", err))
    })
}

fn create_symlink(source: &Path, destination: &Path) -> Result<(), HookError> {
    println!("Creating symlink: {:?} to {:?}", source, destination);

    symlink_auto(source, destination).map_err(|err| {
        HookError::SymlinkCreationError(err)
    })
}