use std::{fmt::Display, path::PathBuf};

pub enum HookError {
    ExecutionError(String),
    SymlinkCreationError(std::io::Error),
    Skipping(String),
    FilesAlreadyExists,
    PathsDontExist,
    DifferentNames,
    CancelledByUser,
    PathDiff {
        source: PathBuf,
        destination: PathBuf,
    },

    #[allow(dead_code)]
    Debug(String),
}

impl Display for HookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HookError::ExecutionError(message) => write!(f, "Execution error: {}", message),
            HookError::SymlinkCreationError(message) => write!(f, "Symlink creation error: {}", message),
            HookError::Skipping(message) => write!(f, "Skipping: {}", message),
            HookError::FilesAlreadyExists => write!(f, "The source and destination paths already have files and --force or --interactive is not passed."),
            HookError::PathsDontExist => write!(f, "The source and destination paths don't exist."),
            HookError::DifferentNames => write!(f, "The source and destination paths have different base names."),
            HookError::CancelledByUser => write!(f, "The operation was cancelled by the user."),
            HookError::PathDiff { source, destination } => write!(f, "Couldn't compute difference between {} and {}", source.display(), destination.display()),
            HookError::Debug(message) => write!(f, "Debug: {}", message),
        }
    }
}
