use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use reqwest::Url;

pub async fn download_file(url: &str, dest_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(url)?;
    let resp = reqwest::get(url).await?;

    let mut out = File::create(dest_path).await?;
    let content = resp.bytes().await?;
    out.write_all(&content).await?;

    println!("Downloaded file to {}", dest_path);
    Ok(())
}
