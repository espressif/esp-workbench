// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use dirs;

mod app_state;
use app_state::{AppState, BuilderState};

mod download;

mod esp_idf;
use esp_idf::{run_install_script};

mod zip_archiver;
use zip_archiver::{zip_dir, unzip};

use serde::Serialize;
use thiserror;
use tauri::{State, Window};

// Create a custom Error that we can return in Results
#[derive(Debug, thiserror::Error)]
enum Error {
    // Implement std::io::Error for our Error enum
    #[error(transparent)]
    Io(#[from] std::io::Error),
    // Add a PoisonError, but we implement it manually later
    #[error("the mutex was poisoned")]
    PoisonError(String),
}

#[tauri::command]
async fn abort_build(state_mutex: State<'_, Mutex<AppState>>) -> Result<String, ()> {
    let mut state = state_mutex.lock().unwrap();
    state.builder = BuilderState::Abort;
    Ok("ok".to_string())
}

// Command to copress directories into a archive file.
#[tauri::command]
async fn compress(window: Window, app: tauri::AppHandle, state_mutex: State<'_, Mutex<AppState>>, source_path: String, target_path:String) -> Result<String, ()> {
    let method = zip::CompressionMethod::Deflated;

    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Running;
    }

    let result = zip_dir(window,app.clone(), source_path.as_str(), target_path.as_str(), method);
    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Idle;
    }

    match result {
        Ok(_) => Ok("Success".to_string()),
        Err(_) => Err(())
    }
}

// Command to decompress a archive file into a directory.
#[tauri::command]
async fn decompress(window: Window, app: tauri::AppHandle, state_mutex: State<'_, Mutex<AppState>>, source_path: String, target_path:String) -> Result<String, ()> {
    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Running;
    }

    let result = unzip(window,app.clone(), source_path, target_path);
    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Idle;
    }

    match result {
        Ok(_) => Ok("Success".to_string()),
        Err(_) => Err(())
    }
}

// Command to run install shell script of ESP-IDF
#[tauri::command]
async fn run_esp_idf_install_script(window: Window, app: tauri::AppHandle, state_mutex: State<'_, Mutex<AppState>>, target_path:String) -> Result<String, ()> {
    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Running;
    }

    let result = run_install_script(window,app.clone(), target_path);
    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Idle;
    }

    match result {
        Ok(_) => Ok("Success".to_string()),
        Err(_) => Err(())
    }
}


// Command to download ESP-IDF to ZIP file
#[tauri::command]
async fn download_esp_idf(window: Window, app: tauri::AppHandle, state_mutex: State<'_, Mutex<AppState>>, version: String, target_path:String) -> Result<String, ()> {
    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Running;
    }

    let download_handle = tokio::spawn(esp_idf::download_esp_idf(window, app, version, target_path));

    let result = download_handle.await;

    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Idle;
    }

    match result {
        Ok(result) => match result {
            Ok(_) => Ok("Download finished successfully".to_string()),
            Err(_) => Ok("Download failed".to_string()),
        },
        Err(err) => Ok("Download task panicked".to_string().to_string()),
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

// Command to get ESP-IDF Tools directory which is specific for each operating system.
#[tauri::command]
async fn get_esp_idf_tools_dir() -> Result<String, ()> {
    #[cfg(unix)]
    match dirs::home_dir() {
        Some(path) => Ok(format!("{}/{}", path.to_str().unwrap(), ".espressif")),
        None => Err(())
    }

    #[cfg(windows)]
    Ok("C:\\Espressif");
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![compress, decompress, download_esp_idf, get_user_home, get_esp_idf_tools_dir, abort_build, run_esp_idf_install_script])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
