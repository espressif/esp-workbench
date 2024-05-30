use async_std::sync::{Arc, Mutex};
use tauri::{State, AppHandle};
use crate::Author;
use crate::external_command::run_external_command_with_progress;
use std::path::PathBuf;

pub struct HugoState {
    pub is_running: Arc<Mutex<bool>>,
}

#[tauri::command]
pub async fn launch_hugo(state: State<'_, HugoState>, app: AppHandle) -> Result<String, String> {
    {
        let mut is_running = state.is_running.lock().await;
        if *is_running {
            return Err("Hugo is already running".to_string());
        }

        // Set Hugo state to running
        *is_running = true;
    } // MutexGuard is dropped here

    // Execute Hugo command and stream output to frontend
    let result = run_external_command_with_progress(app.clone(), "hugo", &["server", "--source", "~/.espressif/devportal"], "hugo-progress").await;

    if result.is_err() {
        let mut is_running = state.is_running.lock().await;
        *is_running = false;
        return Err("Failed to start Hugo server".to_string());
    }

    Ok("Hugo server started".into())
}

#[tauri::command]
pub async fn restart_hugo(state: State<'_, HugoState>, app: AppHandle) -> Result<String, String> {
    let relaunch: bool;
    {
        let mut is_running = state.is_running.lock().await;
        relaunch = *is_running;
        if relaunch {
            // Send SIGTERM to Hugo process
            run_external_command_with_progress(app.clone(), "pkill", &["-f", "hugo server"], "hugo-progress").await.ok();
            *is_running = false;
        }
    } // MutexGuard is dropped here

    if relaunch {
        // Relaunch Hugo
        let result = run_external_command_with_progress(app.clone(), "hugo", &["server", "--source", "~/.espressif/devportal"], "hugo-progress").await;

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

#[tauri::command]
pub async fn save_author(author: Author, file_name: String) -> Result<(), String> {
    let authors_dir = dirs::home_dir().unwrap().join(".espressif/devportal/data/authors");
    let content_dir = dirs::home_dir().unwrap().join(".espressif/devportal/content/authors");
    let file_path = authors_dir.join(&file_name);
    let index_path = content_dir.join(file_name.replace(".json", "")).join("_index.md");

    // Create author JSON file
    std::fs::write(file_path, serde_json::to_string_pretty(&author).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;

    // Create _index.md file
    std::fs::create_dir_all(index_path.parent().unwrap()).map_err(|e| e.to_string())?;
    std::fs::write(index_path, &author.name).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_authors() -> Result<Vec<Author>, String> {
    let authors_path = dirs::home_dir().unwrap().join(".espressif/devportal/data/authors");
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
pub async fn delete_author(file_name: String) -> Result<(), String> {
    let authors_dir = dirs::home_dir().unwrap().join(".espressif/devportal/data/authors");
    let content_dir = dirs::home_dir().unwrap().join(".espressif/devportal/content/authors");
    let file_path = authors_dir.join(&file_name);
    let index_path = content_dir.join(file_name.replace(".json", "")).join("_index.md");

    // Remove author JSON file
    std::fs::remove_file(file_path).map_err(|e| e.to_string())?;

    // Remove _index.md file and the directory if empty
    std::fs::remove_file(index_path.clone()).map_err(|e| e.to_string())?;
    let content_dir = index_path.parent().unwrap();
    if std::fs::read_dir(content_dir).map_err(|e| e.to_string())?.next().is_none() {
        std::fs::remove_dir(content_dir).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn clone_devportal_repo(app: AppHandle) -> Result<String, String> {
    // Expand the tilde to the full path
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let target_dir = home_dir.join(".espressif/devportal");

    // Execute git clone command and stream output to frontend
    let result = run_external_command_with_progress(app.clone(), "git", &["clone", "git@github.com:espressif/developer-portal.git", target_dir.to_str().unwrap()], "git-progress").await;

    if result.is_err() {
        return Err("Failed to clone repository".to_string());
    }

    Ok("Repository cloned successfully".into())
}
