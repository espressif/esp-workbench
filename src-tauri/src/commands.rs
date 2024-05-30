use std::process::Command;
use std::sync::{Arc, Mutex};
use tauri::{State, Window};
use crate::Author;

pub struct HugoState {
    pub is_running: Arc<Mutex<bool>>,
}

#[tauri::command]
pub async fn launch_hugo(state: State<'_, HugoState>) -> Result<(), String> {
    let mut is_running = state.is_running.lock().unwrap();
    if *is_running {
        return Err("Hugo is already running".to_string());
    }

    Command::new("hugo")
        .args(&["server", "--source", "~/.espressif/devportal"])
        .spawn()
        .map_err(|e| e.to_string())?;

    *is_running = true;
    Ok(())
}

#[tauri::command]
pub async fn restart_hugo(state: State<'_, HugoState>) -> Result<(), String> {
    let mut is_running = state.is_running.lock().unwrap();
    if *is_running {
        // Send SIGTERM to Hugo process
        Command::new("pkill")
            .args(&["-f", "hugo server"])
            .output()
            .map_err(|e| e.to_string())?;

        // Relaunch Hugo
        Command::new("hugo")
            .args(&["server", "--source", "~/.espressif/devportal"])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
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
