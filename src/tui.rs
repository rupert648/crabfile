use colored::*;
use dialoguer::{Input, MultiSelect};
use figlet_rs::FIGfont;
use std::path::PathBuf;

use crate::{file_operations::is_valid_repo_path, secret::check_for_and_accept_secrets};

pub fn print_title_screen() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("dot-herder");
    println!("{}", figure.unwrap());
}

fn prompt_for_repo_path() -> Result<String, std::io::Error> {
    Input::new()
        .with_prompt("Enter the path for your .dotfiles repo".cyan().to_string())
        .interact_text()
}

pub fn get_repo_path() -> Result<String, Box<dyn std::error::Error>> {
    let mut path: String;
    loop {
        path = prompt_for_repo_path()?;
        match is_valid_repo_path(&path) {
            Ok(()) => break,
            Err(e) => {
                println!("Invalid path: {}", e);
                println!("Please supply another");
            }
        }
    }

    Ok(path)
}

pub fn select_dotfiles(dotfiles: &[PathBuf]) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let dotfiles_strings: Vec<String> = dotfiles
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect();

    let selections = MultiSelect::new()
        .with_prompt(
            "\nFound the following dotfiles, select which to include (space to select, enter to continue)"
                .green()
                .to_string(),
        )
        .items(&dotfiles_strings)
        .defaults(&vec![true; dotfiles.len()])
        .interact()?;

    let selections: Vec<PathBuf> = selections
        .into_iter()
        .map(|i| dotfiles[i].clone())
        .filter(|pb| check_for_and_accept_secrets(pb).is_ok_and(|f| f))
        .collect();

    Ok(selections)
}
