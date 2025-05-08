use async_std::sync::{Arc, Mutex};
use tauri::{AppHandle, State};
use crate::external_command::run_external_command_with_progress;
use std::path::PathBuf;

pub struct HugoState {
    pub is_running: Arc<Mutex<bool>>,
}

const HUGO_COMMAND: &str = "hugo";
const HUGO_ARGS: &[&str] = &["server", "--source"];

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
