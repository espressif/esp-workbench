
use log::info;

use tauri::{Window, Manager};

use crate::app_state::{AppState, BuilderState};
use std::sync::Mutex;

use std::path::Path;
use crate::download::download_file;

use crate::external_command::run_external_command_with_progress;

#[derive(Clone, serde::Serialize)]
struct Payload {
    pct: String,
}

const PROGRESS_EVENT: &str = "progress";


fn is_abort_state(app: tauri::AppHandle) -> bool {
    let state_mutex = app.state::<Mutex<AppState>>();
    let mut state = state_mutex.lock().unwrap();
    match state.builder {
        BuilderState::Abort => true,
        _ => false
    }
}

#[cfg(unix)]
const INSTALL_SCRIPT_NAME: &str = "install.sh";

#[cfg(windows)]
const INSTALL_SCRIPT_NAME: &str = "install.bat";


pub fn run_install_script(
    window: Window,
    app: tauri::AppHandle,
    esp_idf_path: String) -> Result<String, ()>
{
    let file_path = Path::new(&esp_idf_path).join(INSTALL_SCRIPT_NAME);
    println!("Running install script: {:?}", file_path);

    #[cfg(unix)]
    {
        let args = vec![file_path.to_str().unwrap()];
        run_external_command_with_progress(window.clone(), app.clone(), "bash", &args, PROGRESS_EVENT);
    }

    #[cfg(windows)]
    {
        let args = vec!["/c", file_path.to_str().unwrap()];
        run_external_command_with_progress(window.clone(), app.clone(), "cmd", &args, PROGRESS_EVENT);
    }

    Ok("Success".to_string())
}

pub async fn download_esp_idf(window: Window,
    app: tauri::AppHandle,
    version: String,
    dest_path: String) -> Result<(), ()> {
    let url = format!("https://github.com/espressif/esp-idf/releases/download/{}/esp-idf-{}.zip", version, version);
    println!("Downloading ESP-IDF from {}", url);
    let dest_path = Path::new(&dest_path);

    // If the file exists, check if it is not corrupted
    if dest_path.exists() {
        let is_file_corrupted = {
            match check_zip(&dest_path) {
                Ok(()) => {
                    println!("ESP-IDF already downloaded and the file is not corrupted");
                    return Ok(());
                },
                Err(err) => {
                    eprintln!("The file is corrupted: {}", err);
                    true
                }
            }
        };

        if is_file_corrupted {
            tokio::fs::remove_file(&dest_path).await.unwrap();
        }
    }

    // Ensure parent directory exists
    if let Some(parent_path) = dest_path.parent() {
        tokio::fs::create_dir_all(parent_path).await.unwrap();
    }

    match download_file(window, app, &url, dest_path).await {
        Ok(_) => {
            println!("ESP-IDF downloaded successfully");
            Ok(())
        }
        Err(err) => {
            eprintln!("Failed to download ESP-IDF: {}", err);
            Err(())
        }
    }
}



fn check_zip(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let reader = std::fs::File::open(path)?;
    let mut archive = zip::ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
    }

    Ok(())
}