use tauri::AppHandle;
use crate::external_command::run_external_command_with_progress;
use std::path::PathBuf;

fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Some(home_dir) = dirs::home_dir() {
            return home_dir.join(path.trim_start_matches("~/"));
        }
    }
    PathBuf::from(path)
}

fn get_default_repo_dir() -> PathBuf {
    dirs::home_dir().unwrap().join(".espressif").join("developer-portal")
}

#[tauri::command]
pub async fn clone_devportal_repo(repo_url: String, repo_path: Option<String>, app: AppHandle) -> Result<String, String> {
    let repo_dir = repo_path.unwrap_or_else(|| get_default_repo_dir().to_str().unwrap().to_string());
    let expanded_repo_dir = expand_tilde(&repo_dir);
    let result = run_external_command_with_progress(
        app,
        "git",
        &["clone", &repo_url, expanded_repo_dir.to_str().unwrap()],
        "clone-progress",
    )
    .await;

    if result.is_err() {
        return Err("Failed to clone repository".to_string());
    }

    Ok("Repository cloned successfully".into())
}

#[tauri::command]
pub async fn check_devportal(repo_path: Option<String>) -> Result<bool, String> {
    let repo_dir = repo_path.unwrap_or_else(|| get_default_repo_dir().to_str().unwrap().to_string());
    let expanded_repo_dir = expand_tilde(&repo_dir);

    Ok(expanded_repo_dir.exists())
}
