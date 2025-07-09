use std::{fmt::Display, path::StripPrefixError};

pub enum HookError {
    ExecutionError(String),
    SymlinkCreationError(std::io::Error),
    Skipping(String),
    FilesAlreadyExists,
    PathsDontExist,
    DifferentNames,
    CancelledByUser,
    StripPrefixError {
        inner: StripPrefixError,
        prefix: String,
        full_path: String,
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
            HookError::StripPrefixError { inner, prefix, full_path } => write!(f, "{} is not a prefix for {} | {}", prefix, full_path, inner),
            HookError::Debug(message) => write!(f, "Debug: {}", message),
        }
    }
}
