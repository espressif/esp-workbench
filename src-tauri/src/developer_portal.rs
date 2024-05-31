use async_std::sync::{Arc, Mutex};
use tauri::{AppHandle, State};
use crate::Author;
use crate::external_command::run_external_command_with_progress;
use std::path::PathBuf;
use regex::Regex;
use deunicode::deunicode;

pub struct HugoState {
    pub is_running: Arc<Mutex<bool>>,
}

const REPO_NAME: &str = "developer-portal";
const BASE_DIR: &str = ".espressif";
const AUTHORS_DIR: &str = "data/authors";
const CONTENT_DIR: &str = "content/authors";
const HUGO_COMMAND: &str = "hugo";
const HUGO_ARGS: &[&str] = &["server", "--source"];

// Helper function to expand ~ to the user's home directory
fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Some(home_dir) = dirs::home_dir() {
            return home_dir.join(path.trim_start_matches("~/"));
        }
    }
    PathBuf::from(path)
}

// Helper function to get default repo directory
fn get_default_repo_dir() -> PathBuf {
    dirs::home_dir().unwrap().join(BASE_DIR).join(REPO_NAME)
}

#[tauri::command]
pub async fn launch_hugo(state: State<'_, HugoState>, app: AppHandle, repo_path: Option<String>) -> Result<String, String> {
    let repo_dir = repo_path.unwrap_or_else(|| get_default_repo_dir().to_str().unwrap().to_string());
    let expanded_repo_dir = expand_tilde(&repo_dir);

    {
        let mut is_running = state.is_running.lock().await;
        if *is_running {
            return Err("Hugo is already running".to_string());
        }
        *is_running = true;
    }

    let result = run_external_command_with_progress(app.clone(), HUGO_COMMAND, &[HUGO_ARGS[0], expanded_repo_dir.to_str().unwrap()], "hugo-progress").await;

    if result.is_err() {
        let mut is_running = state.is_running.lock().await;
        *is_running = false;
        return Err("Failed to start Hugo server".to_string());
    }

    Ok("Hugo server started".into())
}

#[tauri::command]
pub async fn restart_hugo(state: State<'_, HugoState>, app: AppHandle, repo_path: Option<String>) -> Result<String, String> {
    let repo_dir = repo_path.unwrap_or_else(|| get_default_repo_dir().to_str().unwrap().to_string());
    let expanded_repo_dir = expand_tilde(&repo_dir);

    let relaunch: bool;
    {
        let mut is_running = state.is_running.lock().await;
        relaunch = *is_running;
        if relaunch {
            run_external_command_with_progress(app.clone(), "pkill", &["-f", "hugo server"], "hugo-progress").await.ok();
            *is_running = false;
        }
    }

    if relaunch {
        let result = run_external_command_with_progress(app.clone(), HUGO_COMMAND, &[HUGO_ARGS[0], expanded_repo_dir.to_str().unwrap()], "hugo-progress").await;

        if result.is_err() {
            let mut is_running = state.is_running.lock().await;
            *is_running = false;
            return Err("Failed to restart Hugo server".to_string());
        }

        let mut is_running = state.is_running.lock().await;
        *is_running = true;
    }

    Ok("Hugo server restarted".to_string())
}

// Helper function to normalize and create file name
fn create_file_name(name: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9-]").unwrap();
    let normalized_name = deunicode(name).to_lowercase().replace(" ", "-");
    re.replace_all(&normalized_name, "").to_string()
}

#[tauri::command]
pub async fn save_author(author: Author, original_file_name: Option<String>, repo_path: Option<String>) -> Result<(), String> {
    let repo_dir = repo_path.unwrap_or_else(|| get_default_repo_dir().to_str().unwrap().to_string());
    let expanded_repo_dir = expand_tilde(&repo_dir);
    let authors_dir = expanded_repo_dir.join(AUTHORS_DIR);
    let content_dir = expanded_repo_dir.join(CONTENT_DIR);

    let file_name = original_file_name.unwrap_or_else(|| create_file_name(&author.name));
    let file_path = authors_dir.join(format!("{}.json", file_name));
    let index_path = content_dir.join(file_name).join("_index.md");

    std::fs::write(file_path, serde_json::to_string_pretty(&author).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;

    std::fs::create_dir_all(index_path.parent().unwrap()).map_err(|e| e.to_string())?;
    std::fs::write(index_path, &author.name).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_authors(repo_path: Option<String>) -> Result<Vec<Author>, String> {
    let repo_dir = repo_path.unwrap_or_else(|| get_default_repo_dir().to_str().unwrap().to_string());
    let expanded_repo_dir = expand_tilde(&repo_dir);
    let authors_path = expanded_repo_dir.join(AUTHORS_DIR);
    let mut authors = Vec::new();
    for entry in std::fs::read_dir(authors_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() {
            let author: Author = serde_json::from_reader(std::fs::File::open(path).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
            authors.push(author);
        }
    }
    Ok(authors)
}

#[tauri::command]
pub async fn delete_author(file_name: String, repo_path: Option<String>) -> Result<(), String> {
    let repo_dir = repo_path.unwrap_or_else(|| get_default_repo_dir().to_str().unwrap().to_string());
    let expanded_repo_dir = expand_tilde(&repo_dir);
    let authors_dir = expanded_repo_dir.join(AUTHORS_DIR);
    let content_dir = expanded_repo_dir.join(CONTENT_DIR);
    let file_path = authors_dir.join(&file_name);
    let index_path = content_dir.join(file_name.replace(".json", "")).join("_index.md");

    std::fs::remove_file(file_path).map_err(|e| e.to_string())?;

    std::fs::remove_file(index_path.clone()).map_err(|e| e.to_string())?;
    let content_dir = index_path.parent().unwrap();
    if std::fs::read_dir(content_dir).map_err(|e| e.to_string())?.next().is_none() {
        std::fs::remove_dir(content_dir).map_err(|e| e.to_string())?;
    }

    Ok(())
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
