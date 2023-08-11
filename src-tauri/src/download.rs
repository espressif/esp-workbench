use std::path::Path;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

use tauri::{Window, Manager};

use std::sync::Mutex;
use crate::app_state::{AppState, BuilderState};
use log::info;

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

    if let Some(parent) = dest_path.parent() {
        std::fs::create_dir_all(parent)?; // Ensure the directory exists
    }

    let mut dest = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&dest_path)
        .await?;

    let mut downloaded: u64 = 0;

    while let Some(chunk) = response.chunk().await? {
        dest.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        let percentage = downloaded as f64 / total_size as f64 * 100.0;
        info!("Download progress: {:.2}%", percentage);
        if is_abort_state(app.clone()) {
            info!("Download aborted at: {:.2}%", percentage);
            break;
        }
    }

    Ok(())
}
