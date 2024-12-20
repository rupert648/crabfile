mod cli;
mod config;
mod file_operations;
mod git_operations;
mod secret;
mod tui;

use clap::Parser;
use cli::Args;
use colored::*;
use config::Config;
use file_operations::{create_dotfiles_repo, find_dotfiles, print_file_tree};
use git_operations::init_git_repo;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;
use std::time::Duration;
use tui::{get_repo_path, print_title_screen, select_dotfiles};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    dbg!(&args.config);
    let config = fs::read_to_string(args.config)?;
    let config: Config = serde_yaml::from_str(&config)?;

    print_title_screen();

    let repo_path = get_repo_path()?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Searching for dotfiles...");
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {msg}")?,
    );
    spinner.enable_steady_tick(Duration::from_millis(100));

    let found_dotfiles = find_dotfiles(&config.dotfiles, &args.home);
    dbg!(&found_dotfiles);
    spinner.finish_and_clear();

    let selected_dotfiles = select_dotfiles(&found_dotfiles)?;
    dbg!(&selected_dotfiles);

    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Creating dotfiles repo...");
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {msg}")?,
    );
    spinner.enable_steady_tick(Duration::from_millis(100));

    create_dotfiles_repo(&repo_path, &selected_dotfiles)?;
    spinner.finish_and_clear();

    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Initialising git...");
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {msg}")?,
    );
    spinner.enable_steady_tick(Duration::from_millis(100));

    init_git_repo(&repo_path)?;
    spinner.finish_and_clear();

    println!("{}", "Dotfiles repo created successfully!".green().bold());
    println!("Repo location: {}\n", repo_path.cyan());

    print_file_tree(Path::new(&repo_path), 0)?;

    Ok(())
}
