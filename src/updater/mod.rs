pub mod github;
pub mod installer;
pub mod version;

use crate::ui;
use crate::utils::file_helper;
use std::env;
use std::path::PathBuf;

pub enum UpdateStatus {
    UpToDate,
    UpdatedSuccessfully(String),
    UpdateError(String),
}

pub struct Updater {
    current_version: String,
}

impl Updater {
    pub fn new(current_version: String) -> Self {
        Updater { current_version }
    }

    pub async fn check_for_updates(
        &self,
        repo: &str,
        asset_pattern: &str,
    ) -> Result<UpdateStatus, Box<dyn std::error::Error>> {
        // Fetch the latest release from GitHub
        let release = match github::fetch_latest_release(repo).await {
            Ok(release) => release,
            Err(e) => return Err(format!("Failed to fetch release: {}", e).into()),
        };

        // Extract the version from the tag name
        let tag_name = release.tag_name.trim_start_matches('v').to_string();
        let latest_version = match version::Version::from_str(&tag_name) {
            Some(v) => v,
            None => return Err(format!("Invalid version format: {}", tag_name).into()),
        };

        // Parse the current version
        let current = match version::Version::from_str(&self.current_version) {
            Some(v) => v,
            None => return Err(format!("Invalid current version: {}", self.current_version).into()),
        };

        // Check if update is needed
        if !latest_version.is_newer_than(&current) {
            return Ok(UpdateStatus::UpToDate);
        }

        // Find the Windows installer asset
        let download_url = match github::get_download_link_by_pattern(&release, asset_pattern) {
            Some(url) => url,
            None => return Err(format!("Windows installer not found in release assets").into()),
        };

        // Show update notification
        ui::show_update_notification();
        println!("New version found: {}", latest_version.to_string());
        println!("Downloading from: {}", download_url);

        // Download the installer
        let temp_dir = env::temp_dir();
        let installer_path = temp_dir.join("zed-installer.exe");
        let download_path = installer_path.to_str().unwrap();

        // Download with progress
        match download_with_progress(&download_url, download_path).await {
            Ok(_) => (),
            Err(e) => return Ok(UpdateStatus::UpdateError(format!("Download failed: {}", e))),
        }

        // Install the update
        let installer = installer::Installer::new(
            download_path.to_string(),
            download_path.to_string(), // Same path for installing
        );

        match installer.install() {
            Ok(_) => Ok(UpdateStatus::UpdatedSuccessfully(
                latest_version.to_string(),
            )),
            Err(e) => Ok(UpdateStatus::UpdateError(e)),
        }
    }
}

async fn download_with_progress(
    url: &str,
    destination: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a progress bar
    let client = reqwest::Client::new();

    // Send GET request to download file
    let response = client.get(url).send().await?;

    // Create file
    let mut file = tokio::fs::File::create(destination).await?;

    // Stream the response body to the file
    let bytes = response.bytes().await?;
    tokio::io::copy(&mut &bytes[..], &mut file).await?;

    Ok(())
}
