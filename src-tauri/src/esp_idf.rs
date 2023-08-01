use std::process::{Command, Child, Stdio};
use std::io::{BufReader, BufRead};
use std::time::Duration;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use tauri::{Window, Manager};

use crate::app_state::{AppState, BuilderState};
use std::sync::{Mutex};

use std::path::Path;
use crate::download::download_file;

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
const SHELL_NAME: &str = "bash";
#[cfg(unix)]
const INSTALL_SCRIPT_NAME: &str = "install.sh";

#[cfg(windows)]
const SHELL_NAME: &str = "cmd";
#[cfg(windows)]
const INSTALL_SCRIPT_NAME: &str = "install.cmd";


pub fn run_install_script(
    window: Window,
    app: tauri::AppHandle,
    esp_idf_path: String) -> Result<String, ()>
{
    let file_path = Path::new(&esp_idf_path).join(INSTALL_SCRIPT_NAME);
    let child_handle = thread::spawn(move || {
        // Launch the script
        let mut child = Command::new(SHELL_NAME)
            .arg(file_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to launch script");

        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);

        // Read the script's output line by line
        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            println!("{}", line);
            let payload = Payload {
                pct: line,
            };
            window.emit(PROGRESS_EVENT, payload).unwrap();

            // If is_abort_state is true, kill the script
            if is_abort_state(app.clone()) {
                child.kill().expect("Failed to kill script");
                break;
            }
        }

        let payload = Payload {
            pct: "Done".to_string(),
        };
        window.emit(PROGRESS_EVENT, payload).unwrap();

        // Wait for the child to exit completely
        child.wait().expect("Failed to wait on child");
    });

    // Wait for the child process to finish
    child_handle.join().unwrap();

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