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

pub fn run_install_script(
    window: Window,
    app: tauri::AppHandle,
    file_path: String) -> Result<String, ()>
{
    let child_handle = thread::spawn(move || {
        // Launch the script
        let mut child = Command::new("bash")
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

pub async fn download_esp_idf(version: String, dest_path: String) -> Result<(), ()> {
    let url = format!("https://github.com/espressif/esp-idf/releases/download/v{}/esp-idf-v{}.zip", version, version);
    // let dest_path = Path::new(&dest_path);

    match download_file(&url, dest_path.as_str()).await {
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
