mod updater;
mod ui;
mod utils;

use std::env;
use std::fs;
use std::path::PathBuf;

const ZED_REPO: &str = "zed-industries/zed";
const WIN_INSTALLER_PATTERN: &str = "zed-windows-x86_64-installer.exe";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the current executable's directory
    let exe_dir = env::current_exe()?
        .parent()
        .ok_or("Failed to get executable directory")?
        .to_path_buf();
    
    println!("Zed Auto Updater starting...");
    
    // Read the current version from version file or use a default
    let current_version = read_current_version(&exe_dir).unwrap_or_else(|_| "0.0.0".to_string());
    println!("Current version: {}", current_version);
    
    // Create updater instance
    let updater = updater::Updater::new(current_version);
    
    // Check for updates and install if available
    match updater.check_for_updates(ZED_REPO, WIN_INSTALLER_PATTERN).await {
        Ok(update_status) => {
            match update_status {
                updater::UpdateStatus::UpToDate => {
                    ui::show_success_message("Zed is already up to date!");
                }
                updater::UpdateStatus::UpdatedSuccessfully(new_version) => {
                    // Save the new version
                    if let Err(e) = save_current_version(&exe_dir, &new_version) {
                        ui::show_error_message(&format!("Failed to save version: {}", e));
                    }
                    ui::show_success_message(&format!("Zed has been updated to version {}!", new_version));
                }
                updater::UpdateStatus::UpdateError(err) => {
                    ui::show_error_message(&format!("Update failed: {}", err));
                }
            }
        }
        Err(e) => {
            ui::show_error_message(&format!("Error checking for updates: {}", e));
        }
    }
    
    Ok(())
}

fn read_current_version(dir: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let version_file = dir.join("zed-version.txt");
    Ok(fs::read_to_string(version_file)?.trim().to_string())
}

fn save_current_version(dir: &PathBuf, version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let version_file = dir.join("zed-version.txt");
    fs::write(version_file, version)?;
    Ok(())
}