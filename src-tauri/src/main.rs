// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use dirs;

mod zip_archiver;
use zip_archiver::zip_dir;

use serde::Serialize;

use tauri::{State, Window};

#[derive(Serialize, Clone)]
pub(crate) struct BuilderState {
    compression_state: String,
}

impl Default for BuilderState {
    fn default() -> Self {
        Self {
            compression_state: "idle".to_string(),
        }
    }
}

#[tauri::command]
async fn set_builder_state(state_mutex: State<'_, Mutex<BuilderState>>) -> Result<BuilderState, Error> {
    let mut state = state_mutex.lock()?;
    state.compression_state = "running".to_string();
    Ok(state.clone())
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn compress(window: Window, source_path: String, target_path:String) -> Result<String, ()> {
    // format!("Source: {}\nTarget: {}", source_path, target_path);
    let method = zip::CompressionMethod::Deflated;
    // window.emit("PROGRESS", payload).unwrap();
    let result = zip_dir(window, source_path.as_str(), target_path.as_str(), method);
    match result {
        Ok(_) => Ok("Success".to_string()),
        Err(_) => Err(())
    }
}

// Comand to get the current user home
#[tauri::command]
async fn get_user_home() -> Result<String, ()> {
    match dirs::home_dir() {
        Some(path) => Ok(path.to_str().unwrap().to_string()),
        None => Err(())
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(BuilderState::default()))
        .invoke_handler(tauri::generate_handler![compress, get_user_home])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
