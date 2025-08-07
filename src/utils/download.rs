use anyhow::Result;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub async fn download_file(url: &str, path: &Path) -> Result<()> {
    println!("Downloading {} to {}", url, path.display());
    
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }
    
    let mut file = fs::File::create(path).await?;
    file.write_all(&bytes).await?;
    
    println!("Downloaded {} ({} bytes)", path.display(), bytes.len());
    Ok(())
}