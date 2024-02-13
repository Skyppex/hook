use std::fmt::Display;



pub enum HookError {
    ExecutionError(String),
    SymlinkCreationError(std::io::Error),
    Skipping(String),
}

impl Display for HookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HookError::ExecutionError(message) => write!(f, "Execution error: {}", message),
            HookError::SymlinkCreationError(message) => write!(f, "Symlink creation error: {}", message),
            HookError::Skipping(message) => write!(f, "Skipping: {}", message),
        }
    }
}