mod scanner;

use std::path::PathBuf;

use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm};
use scanner::{ScanResult, Scanner};
use std::error::Error;

fn check_file_for_secrets(path: &PathBuf) -> Result<Vec<ScanResult>, Box<dyn Error>> {
    let content = std::fs::read_to_string(path)?;

    let scanner = Scanner::new();
    Ok(scanner.scan(&content))
}

pub(crate) fn check_for_and_accept_secrets(path: &PathBuf) -> Result<bool, Box<dyn Error>> {
    let path_string = path.to_str().unwrap();
    println!(
        "{} {} {}",
        "Checking".bright_blue(),
        path_string.bright_blue().bold(),
        "for secrets".bright_blue()
    );

    if path.is_dir() {
        return Ok(true);
    }

    let scan_results = check_file_for_secrets(path)?;
    if scan_results.is_empty() {
        return Ok(true);
    }
    println!(
        "{} {}:",
        "Potential secrets found in:".yellow().bold(),
        path_string
    );
    for result in &scan_results {
        result.print_formatted();
    }
    let continue_with_file = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue with this file?")
        .default(false)
        .interact()?;

    Ok(continue_with_file)
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::check_file_for_secrets;

    const TEST_DATA_DIR: &str = "src/secret/testdata";

    fn get_test_data_path(file_name: &str) -> PathBuf {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        PathBuf::from(manifest_dir)
            .join(TEST_DATA_DIR)
            .join(file_name)
    }

    #[test]
    fn contains_api_key() {
        let path = get_test_data_path("contains-api-key.txt");
        let results = check_file_for_secrets(&path).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].culprit_line.to_lowercase().contains("api_key"));
    }
}
