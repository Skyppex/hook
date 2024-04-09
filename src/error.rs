use std::fmt::Display;

pub enum HookError {
    ExecutionError(String),
    SymlinkCreationError(std::io::Error),
    Skipping(String),
    FilesAlreadyExists,
    DifferentNames,
    CancelledByUser,
}

impl Display for HookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HookError::ExecutionError(message) => write!(f, "Execution error: {}", message),
            HookError::SymlinkCreationError(message) => write!(f, "Symlink creation error: {}", message),
            HookError::Skipping(message) => write!(f, "Skipping: {}", message),
            HookError::FilesAlreadyExists => write!(f, "The source and destination paths already have files and --force or --interactive is not passed."),
            HookError::DifferentNames => write!(f, "The source and destination paths have different names."),
            HookError::CancelledByUser => write!(f, "The operation was cancelled by the user."),
        }
    }
}