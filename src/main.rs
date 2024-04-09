mod args;
mod error;
mod program;

use std::{io::ErrorKind, path::PathBuf};

fn main() {
    let result = program::run();

    if let Err(err) = result {
        eprintln!("{}", err);
    }
}

pub fn get_path(path: &str) -> Result<PathBuf, std::io::Error> {
    let path = match path {
        p if p.starts_with("~") => {
            dirs::home_dir().ok_or(std::io::Error::from(ErrorKind::NotFound))?.join(&p[2..])
        },
        p if p.starts_with(".") => {
            std::env::current_dir()?.join(&p[2..])
        },
        // 
        p => std::env::current_dir()?.join(&p),
    };

    Ok(path)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_get_full_path() {
        let path = "~\\.cargo".to_string();
        let binding = get_path(&path).unwrap();
        let full_path = binding.to_str().unwrap();
        let binding = dirs::home_dir().unwrap().join(".cargo");
        let expected = binding.to_str().unwrap();
        assert_eq!(full_path, expected);
    }

    #[test]
    fn test_get_full_path_absolute() {
        let path = "C:\\Users\\user\\.cargo".to_string();
        let binding = get_path(&path).unwrap();
        let full_path = binding.to_str().unwrap();
        let binding = Path::new("C:\\Users\\user\\.cargo").to_path_buf();
        let expected = binding.to_str().unwrap();
        assert_eq!(full_path, expected);
    }
}