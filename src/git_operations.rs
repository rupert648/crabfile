use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

pub fn init_git_repo(repo_path: &str) -> Result<(), std::io::Error> {
    Command::new("git")
        .arg("init")
        .current_dir(repo_path)
        .output()?;
    Ok(())
}

pub fn add_pre_commit_hook(repo_path: &str) -> Result<(), std::io::Error> {
    let hook_contents = fs::read_to_string("sym-link-pre-commit.sh")?;

    let hook_path = Path::new(repo_path)
        .join(".git")
        .join("hooks")
        .join("pre-commit");

    if let Some(parent) = hook_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(&hook_path)?;
    file.write_all(hook_contents.as_bytes())?;

    // Make the file executable (rwxr-xr-x = 0o755)
    let mut perms = file.metadata()?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&hook_path, perms)?;

    Ok(())
}
