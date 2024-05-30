use tauri::{State, Window};
use std::sync::Mutex;

use crate::app_state::{AppState, BuilderState};
use crate::Author;

#[tauri::command]
pub async fn abort_build(state_mutex: State<'_, Mutex<AppState>>) -> Result<String, ()> {
    let mut state = state_mutex.lock().unwrap();
    state.builder = BuilderState::Abort;
    Ok("ok".to_string())
}

// Other command functions...

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