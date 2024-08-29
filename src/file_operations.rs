use crate::config::Dotfile;
use crate::secret::check_for_and_accept_secrets;
use colored::*;
use std::fs;
use std::path::{Path, PathBuf};
use symlink::symlink_file;
use thiserror::Error;

const DEFAULT_HOME_PATH: &str = "~";

fn create_dotfile_path(path: &str, home_path: Option<&str>) -> String {
    path.replace(
        "${HOME}",
        if home_path.is_some() {
            home_path.unwrap()
        } else {
            DEFAULT_HOME_PATH
        },
    )
}

pub fn find_dotfiles(dotfiles: &[Dotfile], home_path: Option<&str>) -> Vec<PathBuf> {
    dotfiles
        .iter()
        .filter_map(|dotfile| {
            let path = create_dotfile_path(&dotfile.name, home_path);

            let path = PathBuf::from(shellexpand::tilde(&path).to_string());
            if path.exists() {
                Some(path)
            } else {
                None
            }
        })
        .collect()
}

pub fn create_dotfiles_repo(
    repo_path: &str,
    dotfiles: &[PathBuf],
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(repo_path)?;

    for dotfile in dotfiles {
        if !check_for_and_accept_secrets(dotfile)? {
            continue;
        }

        let target = Path::new(repo_path).join(dotfile.file_name().unwrap());

        symlink_file(dotfile, &target)?;
        println!(
            "  {} {}",
            "Symlinked:".green(),
            dotfile.to_string_lossy().blue()
        );
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum RepoPathError {
    #[error("The path already exists")]
    PathAlreadyExists,
    #[error("The parent directory does not exist")]
    ParentDirectoryNotFound,
    #[error("The parent directory is not writable")]
    ParentDirectoryNotWritable,
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn is_valid_repo_path(path: &str) -> Result<(), RepoPathError> {
    let path = Path::new(path);

    // Check if the path already exists
    if path.exists() {
        return Err(RepoPathError::PathAlreadyExists);
    }

    // Check if the parent directory exists and is writable
    let parent = path
        .parent()
        .ok_or_else(|| RepoPathError::InvalidPath("Path has no parent directory".to_string()))?;

    if !parent.exists() {
        return Err(RepoPathError::ParentDirectoryNotFound);
    }

    if !parent.is_dir() {
        return Err(RepoPathError::InvalidPath(
            "Parent is not a directory".to_string(),
        ));
    }

    // Check if the parent directory is writable by attempting to create a temporary file
    let temp_file_path = parent.join(".tmp_write_test");
    match fs::File::create(&temp_file_path) {
        Ok(_) => {
            let _ = fs::remove_file(temp_file_path);
            Ok(())
        }
        Err(_) => Err(RepoPathError::ParentDirectoryNotWritable),
    }
}

pub fn print_file_tree(dir: &Path, level: usize) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.file_name().unwrap() == ".git" {
                continue;
            }

            let prefix = if level == 0 {
                "".to_string()
            } else {
                format!("{}└─ ", "   ".repeat(level - 1))
            };

            if path.is_dir() {
                println!(
                    "{}{}/",
                    prefix.blue(),
                    path.file_name().unwrap().to_string_lossy().cyan()
                );
                print_file_tree(&path, level + 1)?;
            } else {
                println!(
                    "{}{}",
                    prefix.blue(),
                    path.file_name().unwrap().to_string_lossy().green()
                );
            }
        }
    }
    Ok(())
}
