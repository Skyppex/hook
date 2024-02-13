use clap::Parser;
use std::path::Path;
use symlink::symlink_auto;

use args::HookArgs;
use error::HookError;

mod args;
mod error;

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

fn create_symlink_file(source: &Path, destination: &Path, force: bool) -> Result<(), HookError> {
    if destination.is_symlink() {
        return Err(HookError::Skipping("The destination path is already a symlink.".to_string()));
    }
    
    assert!(source.extension().is_some());
    assert!(destination.extension().is_some());

    match (destination.exists(), source.exists()) {
        (false, true) => {
            create_symlink(source, destination)
        },
        (true, false) => {
            move_file(destination, source)?;
            create_symlink(source, destination)
        },
        (true, true) => {
            if !force {
                return Err(HookError::FilesAlreadyExists);
            }

            println!("The source and destination paths both exist.");
            println!("Source: {}", source.display());
            println!("Destination: {}", destination.display());
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
            create_file(source)?;
            create_symlink(source, destination)
        },
    }
}

fn create_symlink_directory(source: &Path, destination: &Path, force: bool) -> Result<(), HookError> {
    if destination.is_symlink() {
        return Err(HookError::Skipping("The destination path is already a symlink.".to_string()));
    }

    assert!(source.extension().is_none());
    assert!(destination.extension().is_none());

    match (destination.exists(), source.exists()) {
        (false, true) => {
            create_symlink(source, destination)
        },
        (true, false) => {
            move_directory(destination, source)?;
            create_symlink(source, destination)
        },
        (true, true) => {
            if is_dir_empty(destination) {
                remove_directory(destination)?;
                return create_symlink_directory(source, destination, force);
            } else if is_dir_empty(source) {
                remove_directory(source)?;
                return create_symlink_directory(source, destination, force);
            }

            if !force {
                return Err(HookError::FilesAlreadyExists);
            }

            println!("The source and destination paths both exist and have files in them.");
            println!("Source: {}", source.display());
            println!("Destination: {}", destination.display());
            println!("Which do you wish to keep? (s/d) OR (n) to cancel.");

            let mut input = String::new();
            loop {
                match std::io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        match input.trim() {
                            "s" | "S" => { // Keeping source
                                remove_directory(destination)?;
                                break;
                            },
                            "d" | "D" => { // Keeping destination
                                remove_directory(source)?;
                                move_directory(destination, source)?;
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
            create_directory(source)?;
            create_symlink(source, destination)
        },
    }
}

fn remove_file(path: &Path) -> Result<(), HookError> {
    println!("Removing file: {}", path.display());

    std::fs::remove_file(path).map_err(|err| {
        HookError::ExecutionError(format!("Error removing file: {}", err))
    })
}

fn remove_directory(path: &Path) -> Result<(), HookError> {
    println!("Removing directory: {}", path.display());

    std::fs::remove_dir_all(path).map_err(|err| {
        HookError::ExecutionError(format!("Error removing directory: {}", err))
    })
}

fn create_file(path: &Path) -> Result<(), HookError> {
    println!("Creating file: {}", path.display());

    std::fs::File::create(path).map_err(|err| {
        HookError::ExecutionError(format!("Error creating file: {}", err))
    }).map(|_| ())
}

fn create_directory(path: &Path) -> Result<(), HookError> {
    println!("Creating directory: {}", path.display());

    std::fs::create_dir_all(path).map_err(|err| {
        HookError::ExecutionError(format!("Error creating directory: {}", err))
    })
}

fn move_file(from: &Path, to: &Path) -> Result<(), HookError> {
    println!("Moving file: {} to {}", from.display(), to.display());

    std::fs::rename(from, to).map_err(|err| {
        HookError::ExecutionError(format!("Error moving file: {}", err))
    })
}

fn move_directory(from: &Path, to: &Path) -> Result<(), HookError> {
    println!("Moving directory: {} to {}", from.display(), to.display());

    std::fs::rename(from, to).map_err(|err| {
        HookError::ExecutionError(format!("Error moving directory: {}", err))
    })
}

fn create_symlink(source: &Path, destination: &Path) -> Result<(), HookError> {
    println!("Creating symlink: {} to {}", source.display(), destination.display());

    symlink_auto(source, destination).map_err(|err| {
        HookError::SymlinkCreationError(err)
    })
}

fn is_dir_empty(path: &Path) -> bool {
    match std::fs::read_dir(path) {
        Ok(mut dir) => dir.next().is_none(),
        Err(_) => true,
    }
}