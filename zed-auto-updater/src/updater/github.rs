// This file contains functions to interact with the GitHub API, fetching the latest release information and download links for the Zed application.

use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
}

pub async fn fetch_latest_release(repo: &str) -> Result<Release, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    
    let response = client
        .get(&url)
        .header("User-Agent", "zed-auto-updater")
        .send()
        .await?
        .json::<Release>()
        .await?;
    
    Ok(response)
}

pub fn get_download_link(release: &Release, asset_name: &str) -> Option<String> {
    release.assets.iter()
        .find(|asset| asset.name == asset_name)
        .map(|asset| asset.browser_download_url.clone())
}

pub fn get_download_link_by_pattern(release: &Release, pattern: &str) -> Option<String> {
    release.assets.iter()
        .find(|asset| asset.name.contains(pattern))
        .map(|asset| asset.browser_download_url.clone())
}