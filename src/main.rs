use clap::Parser;
use std::path::Path;
use symlink::{symlink_dir, symlink_file};

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
    let quiet = args.quiet;

    let source = Path::new(&source);
    let destination = Path::new(&destination);

    if source.file_name() != destination.file_name() {
        return Err(HookError::DifferentNames);
    }

    if source.extension().is_some() {
        if destination.extension().is_none() {
            return Err(HookError::ExecutionError("The destination path must be a file if the source path is a file.".to_string()));
        }
        
        create_symlink_file(source, destination, force, quiet)?;
    } else {
        if destination.extension().is_some() {
            return Err(HookError::ExecutionError("The destination path must be a directory if the source path is a directory.".to_string()));
        }

        create_symlink_directory(source, destination, force, quiet)?;
    }

    Ok(())
}

fn create_symlink_file(source: &Path, destination: &Path, force: bool, quiet: bool) -> Result<(), HookError> {
    if destination.is_symlink() {
        return Err(HookError::Skipping("The destination path is already a symlink.".to_string()));
    }
    
    assert!(source.extension().is_some());
    assert!(destination.extension().is_some());

    match (destination.exists(), source.exists()) {
        (false, true) => {
            create_symlink_file_op(source, destination, quiet)
        },
        (true, false) => {
            move_file(destination, source, quiet)?;
            create_symlink_file_op(source, destination, quiet)
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
                                remove_file(destination, quiet)?;
                                break;
                            },
                            "d" | "D" => { // Keeping destination
                                remove_file(source, quiet)?;
                                move_file(destination, source, quiet)?;
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
                        return Err(HookError::ExecutionError(format!("Error reading input: {}", err)));
                    },
                }
            }

            create_symlink_file_op(source, destination, quiet)
        },
        (false, false) => {
            create_file(source, quiet)?;
            create_symlink_file_op(source, destination, quiet)
        },
    }
}

fn create_symlink_directory(source: &Path, destination: &Path, force: bool, quiet: bool) -> Result<(), HookError> {
    if destination.is_symlink() {
        return Err(HookError::Skipping("The destination path is already a symlink.".to_string()));
    }

    assert!(source.extension().is_none());
    assert!(destination.extension().is_none());

    match (destination.exists(), source.exists()) {
        (false, true) => {
            create_symlink_directory_op(source, destination, quiet)
        },
        (true, false) => {
            move_directory(destination, source, quiet)?;
            create_symlink_directory_op(source, destination, quiet)
        },
        (true, true) => {
            if is_dir_empty(destination) {
                remove_directory(destination, quiet)?;
                return create_symlink_directory(source, destination, force, quiet);
            } else if is_dir_empty(source) {
                remove_directory(source, quiet)?;
                return create_symlink_directory(source, destination, force, quiet);
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
                                remove_directory(destination, quiet)?;
                                break;
                            },
                            "d" | "D" => { // Keeping destination
                                remove_directory(source, quiet)?;
                                move_directory(destination, source, quiet)?;
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

            create_symlink_directory_op(source, destination, quiet)
        },
        (false, false) => {
            create_directory(source, quiet)?;
            create_symlink_directory_op(source, destination, quiet)
        },
    }
}

fn remove_file(path: &Path, quiet: bool) -> Result<(), HookError> {
    if !quiet {
        println!("Removing file: {}", path.display());
    }

    std::fs::remove_file(path).map_err(|err| {
        HookError::ExecutionError(format!("Error removing file: {}", err))
    })
}

fn remove_directory(path: &Path, quiet: bool) -> Result<(), HookError> {
    if !quiet {
        println!("Removing directory: {}", path.display());
    }

    std::fs::remove_dir_all(path).map_err(|err| {
        HookError::ExecutionError(format!("Error removing directory: {}", err))
    })
}

fn create_file(path: &Path, quiet: bool) -> Result<(), HookError> {
    if !quiet {
        println!("Creating file: {}", path.display());
    }

    std::fs::File::create(path).map_err(|err| {
        HookError::ExecutionError(format!("Error creating file: {}", err))
    }).map(|_| ())
}

fn create_directory(path: &Path, quiet: bool) -> Result<(), HookError> {
    if !quiet {
        println!("Creating directory: {}", path.display());
    }

    std::fs::create_dir_all(path).map_err(|err| {
        HookError::ExecutionError(format!("Error creating directory: {}", err))
    })
}

fn move_file(from: &Path, to: &Path, quiet: bool) -> Result<(), HookError> {
    if !quiet {
        println!("Moving file: {} to {}", from.display(), to.display());
    }

    std::fs::rename(from, to).map_err(|err| {
        HookError::ExecutionError(format!("Error moving file: {}", err))
    })
}

fn move_directory(from: &Path, to: &Path, quiet: bool) -> Result<(), HookError> {
    if !quiet {
        println!("Moving directory: {} to {}", from.display(), to.display());
    }

    std::fs::rename(from, to).map_err(|err| {
        HookError::ExecutionError(format!("Error moving directory: {}", err))
    })
}

fn create_symlink_file_op(source: &Path, destination: &Path, quiet: bool) -> Result<(), HookError> {
    if !quiet {
        println!("Creating symlink: {} -> {}", source.display(), destination.display());
    }

    symlink_file(source, destination).map_err(|err| {
        HookError::SymlinkCreationError(err)
    })
}

fn create_symlink_directory_op(source: &Path, destination: &Path, quiet: bool) -> Result<(), HookError> {
    if !quiet {
        println!("Creating symlink: {} -> {}", source.display(), destination.display());
    }

    symlink_dir(source, destination).map_err(|err| {
        HookError::SymlinkCreationError(err)
    })
}

fn is_dir_empty(path: &Path) -> bool {
    match std::fs::read_dir(path) {
        Ok(mut dir) => dir.next().is_none(),
        Err(_) => true,
    }
}
