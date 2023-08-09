use std::path::Path;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt; // Add this line

use tauri::{Window, Manager};

use std::sync::Mutex;
use crate::app_state::{AppState, BuilderState};

const PROGRESS_EVENT: &str = "progress";

#[derive(Clone, serde::Serialize)]
struct Payload {
    pct: String,
}

fn is_abort_state(app: tauri::AppHandle) -> bool {
    let state_mutex = app.state::<Mutex<AppState>>();
    let mut state = state_mutex.lock().unwrap();
    match state.builder {
        BuilderState::Abort => true,
        _ => false
    }
}

pub async fn download_file(window: Window, app: tauri::AppHandle, url: &str, dest_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let total_size = {
        let resp = reqwest::get(url).await?;
        resp.content_length().ok_or("unable to get content length")?
    };

    let request = reqwest::get(url);
    let mut response = request.await?;

    let mut dest = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&dest_path)
        .await?;

    let mut downloaded: u64 = 0;

    while let Some(chunk) = response.chunk().await? {
        dest.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        let percentage = downloaded as f64 / total_size as f64 * 100.0;
        let payload = Payload {
            pct: format!("Download progress: {:.2}%", percentage).to_string(),
        };
        window.emit(PROGRESS_EVENT, payload).unwrap();
        if is_abort_state(app.clone()) {
            let payload = Payload {
                pct: format!("Download aborted at: {:.2}%", percentage).to_string(),
            };
            window.emit(PROGRESS_EVENT, payload).unwrap();
            break;
        }
    }

    Ok(())
}
