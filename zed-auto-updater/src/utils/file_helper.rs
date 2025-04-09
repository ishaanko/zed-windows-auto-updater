use std::fs;
use std::io;

pub async fn download_file(url: &str, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    fs::write(destination, bytes)?;
    Ok(())
}

pub fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn delete_file(path: &str) -> Result<(), io::Error> {
    fs::remove_file(path)
}

pub fn create_directory(path: &str) -> Result<(), io::Error> {
    fs::create_dir_all(path)
}

pub fn get_file_size(path: &str) -> Result<u64, io::Error> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}