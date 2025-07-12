use std::path::{Path, PathBuf};

use clap::Parser;
use symlink::{symlink_dir, symlink_file};

use crate::args::HookArgs;
use crate::error::HookError;
use crate::utils::get_path;

pub fn run() -> Result<(), HookError> {
    let args = HookArgs::parse();

    if args.verbose {
        eprintln!("Args: {:#?}", args);
    }

    let source = get_path(&args.source.replace(r"\\", r"/").replace(r"\", "/"))
        .map_err(|err| HookError::ExecutionError(format!("Error getting source path: {}", err)))?;

    let destination = get_path(&args.destination.replace(r"\\", r"/").replace(r"\", "/"))
        .map_err(|err| HookError::ExecutionError(format!("Error getting source path: {}", err)))?;

    if args.verbose {
        eprintln!("Source: {}", source.display());
        eprintln!("Destination: {}", destination.display());

        if args.relative {
            eprintln!(
                "Relative destination: {}",
                pathdiff::diff_paths(&source, &destination)
                    .unwrap_or_else(|| PathBuf::from("N/A"))
                    .display()
            );
        }
    }

    if source.file_name() != destination.file_name() {
        return handle_different_base_names(source, destination, args.clone());
    }

    check_valid_paths_and_create_symlink(source, destination, args)
}

fn handle_different_base_names(
    source: PathBuf,
    destination: PathBuf,
    args: HookArgs,
) -> Result<(), HookError> {
    if args.force {
        return check_valid_paths_and_create_symlink(source, destination, args);
    }

    if !args.interactive {
        return Err(HookError::DifferentNames);
    }

    let expected_destination = destination.with_file_name(source.file_name().unwrap());

    eprintln!(
        "Possible name error: The destination path does not have the same name as the source path."
    );
    eprintln!("Source: {}", source.display());
    eprintln!("Destination: {}", destination.display());
    eprintln!("Expected destination: {}", expected_destination.display());
    eprintln!("Which path did you mean to use? (d/e) OR (n) to cancel.");

    let mut input = String::new();

    loop {
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim() {
                    "d" | "D" => {
                        // Using inputted destination
                        return check_valid_paths_and_create_symlink(source, destination, args);
                    }
                    "e" | "E" => {
                        // Using expected destination
                        return check_valid_paths_and_create_symlink(
                            source,
                            expected_destination,
                            args,
                        );
                    }
                    "n" | "N" => {
                        // Cancel
                        return Err(HookError::CancelledByUser);
                    }
                    _ => {
                        eprintln!("Invalid input. Please enter 'd', 'e', or 'n'.");
                        input.clear();
                    }
                }
            }
            Err(err) => {
                return Err(HookError::ExecutionError(format!(
                    "Error reading input: {}",
                    err
                )));
            }
        }
    }
}

fn check_valid_paths_and_create_symlink(
    source: PathBuf,
    destination: PathBuf,
    args: HookArgs,
) -> Result<(), HookError> {
    let source_is_file = source
        .exists()
        .then(|| source.metadata().unwrap().is_file());

    let destination_is_file = destination
        .exists()
        .then(|| destination.metadata().unwrap().is_file());

    match (source_is_file, destination_is_file) {
        (None, None) => Err(HookError::PathsDontExist),
        (Some(true), Some(false)) => Err(HookError::ExecutionError(
            "The destination path must be a file if the source path is a file.".to_string(),
        )),
        (Some(false), Some(true)) => Err(HookError::ExecutionError(
            "The destination path must be a directory if the source path is a directory."
                .to_string(),
        )),
        (Some(true), None) => create_symlink_file(&source, &destination, args),
        (None, Some(true)) => create_symlink_file(&source, &destination, args),
        (Some(false), None) => create_symlink_directory(&source, &destination, args),
        (None, Some(false)) => create_symlink_directory(&source, &destination, args),
        (Some(true), Some(true)) => create_symlink_file(&source, &destination, args),
        (Some(false), Some(false)) => create_symlink_directory(&source, &destination, args),
    }
}

fn create_symlink_file(source: &Path, destination: &Path, args: HookArgs) -> Result<(), HookError> {
    if destination.is_symlink() {
        handle_symlink_different_target(source, destination, args.clone())?;
    }

    if args.verbose {
        eprintln!(
            "Trying to create symlink file: {} -> {}",
            destination.display(),
            source.display(),
        );
    }

    match (destination.exists(), source.exists()) {
        (false, true) => create_symlink_file_op(source, destination, args.clone()),
        (true, false) => {
            move_file(destination, source, args.clone())?;
            create_symlink_file_op(source, destination, args.clone())
        }
        (true, true) => {
            if !args.force && !args.interactive {
                return Err(HookError::FilesAlreadyExists);
            }

            if args.interactive {
                eprintln!("The source and destination paths both exist.");
                eprintln!("Source: {}", source.display());
                eprintln!("Destination: {}", destination.display());
                eprintln!("Which do you wish to keep? (s/d) OR (n) to cancel.");

                let mut input = String::new();

                loop {
                    match std::io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            match input.trim() {
                                "s" | "S" => {
                                    // Keeping source
                                    remove_file(destination, args.clone())?;
                                    break;
                                }
                                "d" | "D" => {
                                    // Keeping destination
                                    remove_file(source, args.clone())?;
                                    move_file(destination, source, args.clone())?;
                                    break;
                                }
                                "n" | "N" => {
                                    // Cancel
                                    return Err(HookError::CancelledByUser);
                                }
                                _ => {
                                    eprintln!("Invalid input. Please enter 's', 'd', or 'n'.");
                                    input.clear();
                                }
                            }
                        }
                        Err(err) => {
                            return Err(HookError::ExecutionError(format!(
                                "Error reading input: {}",
                                err
                            )));
                        }
                    }
                }
            } else {
                // args.force is always true here
                remove_file(destination, args.clone())?;
            }

            create_symlink_file_op(source, destination, args)
        }
        (false, false) => {
            unreachable!(
                "Check occurs in {}",
                stringify!(check_valid_paths_and_create_symlink)
            );
        }
    }
}

fn create_symlink_directory(
    source: &Path,
    destination: &Path,
    args: HookArgs,
) -> Result<(), HookError> {
    if destination.is_symlink() {
        handle_symlink_different_target(source, destination, args.clone())?;
    }

    if args.verbose {
        eprintln!(
            "Trying to create symlink directory: {} -> {}",
            destination.display(),
            source.display(),
        );
    }

    assert!(source.extension().is_none());
    assert!(destination.extension().is_none());

    match (destination.exists(), source.exists()) {
        (false, true) => create_symlink_directory_op(source, destination, args),
        (true, false) => {
            move_directory(destination, source, args.clone())?;
            create_symlink_directory_op(source, destination, args)
        }
        (true, true) => {
            if is_dir_empty(destination) {
                remove_directory(destination, args.clone())?;
                return create_symlink_directory(source, destination, args);
            } else if is_dir_empty(source) {
                remove_directory(source, args.clone())?;
                return create_symlink_directory(source, destination, args);
            }

            if !args.force && !args.interactive {
                return Err(HookError::FilesAlreadyExists);
            }

            if args.interactive {
                eprintln!("The source and destination paths both exist and have files in them.");
                eprintln!("Source: {}", source.display());
                eprintln!("Destination: {}", destination.display());
                eprintln!("Which do you wish to keep? (s/d) OR (n) to cancel.");

                let mut input = String::new();
                loop {
                    match std::io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            match input.trim() {
                                "s" | "S" => {
                                    // Keeping source
                                    remove_directory(destination, args.clone())?;
                                    break;
                                }
                                "d" | "D" => {
                                    // Keeping destination
                                    remove_directory(source, args.clone())?;
                                    move_directory(destination, source, args.clone())?;
                                    break;
                                }
                                "n" | "N" => {
                                    // Cancel
                                    return Err(HookError::CancelledByUser);
                                }
                                _ => {
                                    eprintln!("Invalid input. Please enter 's', 'd', or 'n'.");
                                    input.clear();
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("Error reading input: {}", err);
                        }
                    }
                }
            } else {
                // args.force is always true here
                remove_directory(destination, args.clone())?;
            }

            create_symlink_directory_op(source, destination, args)
        }
        (false, false) => {
            create_directory(source, args.clone())?;
            create_symlink_directory_op(source, destination, args)
        }
    }
}

fn remove_file(path: &Path, args: HookArgs) -> Result<(), HookError> {
    if !args.quiet {
        eprintln!("Removing file: {}", path.display());
    }

    std::fs::remove_file(path)
        .map_err(|err| HookError::ExecutionError(format!("Error removing file: {}", err)))
}

fn remove_directory(path: &Path, args: HookArgs) -> Result<(), HookError> {
    if !args.quiet {
        eprintln!("Removing directory: {}", path.display());
    }

    std::fs::remove_dir_all(path)
        .map_err(|err| HookError::ExecutionError(format!("Error removing directory: {}", err)))
}

fn create_directory(path: &Path, args: HookArgs) -> Result<(), HookError> {
    if !args.quiet {
        eprintln!("Creating directory: {}", path.display());
    }

    std::fs::create_dir_all(path)
        .map_err(|err| HookError::ExecutionError(format!("Error creating directory: {}", err)))
}

fn move_file(from: &Path, to: &Path, args: HookArgs) -> Result<(), HookError> {
    if !args.quiet {
        eprintln!("Moving file: {} to {}", from.display(), to.display());
    }

    std::fs::rename(from, to)
        .map_err(|err| HookError::ExecutionError(format!("Error moving file: {}", err)))
}

fn move_directory(from: &Path, to: &Path, args: HookArgs) -> Result<(), HookError> {
    if !args.quiet {
        eprintln!("Moving directory: {} to {}", from.display(), to.display());
    }

    std::fs::rename(from, to)
        .map_err(|err| HookError::ExecutionError(format!("Error moving directory: {}", err)))
}

fn create_symlink_file_op(
    source: &Path,
    destination: &Path,
    args: HookArgs,
) -> Result<(), HookError> {
    let source = if args.relative {
        let parent = destination.parent().ok_or_else(|| {
            HookError::ExecutionError("Destination path has no parent".to_string())
        })?;

        &pathdiff::diff_paths(source, parent).ok_or_else(|| HookError::PathDiff {
            source: source.to_path_buf(),
            destination: destination.to_path_buf(),
        })?
    } else {
        source
    };

    if !args.quiet {
        eprintln!(
            "Creating symlink: {} -> {}",
            destination.display(),
            source.display(),
        );
    }

    if args.dry_run {
        return Ok(());
    }

    symlink_file(source, destination).map_err(HookError::SymlinkCreationError)
}

fn create_symlink_directory_op(
    source: &Path,
    destination: &Path,
    args: HookArgs,
) -> Result<(), HookError> {
    let source = if args.relative {
        let parent = destination.parent().ok_or_else(|| {
            HookError::ExecutionError("Destination path has no parent".to_string())
        })?;

        &pathdiff::diff_paths(source, parent).ok_or_else(|| HookError::PathDiff {
            source: source.to_path_buf(),
            destination: destination.to_path_buf(),
        })?
    } else {
        source
    };

    if !args.quiet {
        eprintln!(
            "Creating symlink: {} -> {}",
            destination.display(),
            source.display(),
        );
    }

    if args.dry_run {
        return Ok(());
    }

    symlink_dir(source, destination).map_err(HookError::SymlinkCreationError)
}

fn is_dir_empty(path: &Path) -> bool {
    match std::fs::read_dir(path) {
        Ok(mut dir) => dir.next().is_none(),
        Err(_) => true,
    }
}

fn handle_symlink_different_target(
    source: &Path,
    destination: &Path,
    args: HookArgs,
) -> Result<(), HookError> {
    let target = destination
        .read_link()
        .map_err(|err| HookError::ExecutionError(format!("Error reading symlink: {}", err)))?;

    if target == source {
        return Err(HookError::Skipping(format!("The destination path is already a symlink to the source path | Source: {} | Destination: {}", source.display(), destination.display())));
    }

    if !args.force && !args.interactive {
        return Err(HookError::FilesAlreadyExists);
    }

    if args.interactive {
        eprintln!("The destination path is already a symlink, but with a different target.");
        eprintln!("Source: {}", source.display());
        eprintln!("Destination: {}", destination.display());
        eprintln!("Do you wish to overwrite the symlink target? (y/n)");

        let mut input = String::new();

        loop {
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {
                    match input.trim() {
                        "y" | "Y" => {
                            // Overwrite
                            remove_file(destination, args)?;
                            break;
                        }
                        "n" | "N" => {
                            // Cancel
                            return Err(HookError::CancelledByUser);
                        }
                        _ => {
                            eprintln!("Invalid input. Please enter 'y' or 'n'.");
                            input.clear();
                        }
                    }
                }
                Err(err) => {
                    return Err(HookError::ExecutionError(format!(
                        "Error reading input: {}",
                        err
                    )));
                }
            }
        }
    } else {
        // args.force is always true here
        remove_file(destination, args)?;
    }

    Ok(())
}
