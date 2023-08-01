use std::path::Path;
use reqwest::StatusCode;
use futures::StreamExt;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt; // Add this line

use tauri::{Window};

const PROGRESS_EVENT: &str = "progress";

#[derive(Clone, serde::Serialize)]
struct Payload {
    pct: String,
}

pub async fn download_file(window: Window, url: &str, dest_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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
        
    }

    Ok(())
}
