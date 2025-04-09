// This file handles the installation process of the downloaded updates, including verifying the integrity of the files and executing the installation.

use std::fs;
use std::process::Command;
use std::path::Path;

pub struct Installer {
    pub download_path: String,
    pub install_path: String,
}

impl Installer {
    pub fn new(download_path: String, install_path: String) -> Self {
        Installer {
            download_path,
            install_path,
        }
    }

    pub fn verify_integrity(&self) -> Result<bool, String> {
        // Basic verification: just check if the file exists and has a non-zero size
        match fs::metadata(&self.download_path) {
            Ok(metadata) => {
                if metadata.len() == 0 {
                    return Err("Downloaded file is empty".to_string());
                }
                Ok(true)
            },
            Err(e) => Err(format!("Failed to verify file: {}", e)),
        }
    }

    pub fn install(&self) -> Result<(), String> {
        if let Err(e) = self.verify_integrity() {
            return Err(format!("Integrity check failed: {}", e));
        }
        
        // Check if Zed is running and prompt to close it
        if crate::utils::process_helper::is_process_running("zed.exe") {
            println!("Zed is currently running. Please close it before continuing.");
            
            #[cfg(windows)]
            unsafe {
                use std::ffi::CString;
                use std::ptr::null_mut;
                use winapi::um::winuser::{MessageBoxA, MB_YESNO, MB_ICONQUESTION, IDYES};
                
                let title = CString::new("Zed Auto Updater").unwrap();
                let text = CString::new("Zed is currently running. Close it automatically?").unwrap();
                let response = MessageBoxA(null_mut(), text.as_ptr(), title.as_ptr(), MB_YESNO | MB_ICONQUESTION);
                
                if response == IDYES {
                    if let Err(e) = crate::utils::process_helper::kill_process("zed.exe") {
                        return Err(format!("Failed to close Zed: {}", e));
                    }
                    // Wait for process to fully terminate
                    std::thread::sleep(std::time::Duration::from_secs(1));
                } else {
                    return Err("Update canceled. Please close Zed and try again.".to_string());
                }
            }
        }
        
        println!("Starting installation process...");
        
        let installer_path = Path::new(&self.download_path);
        if installer_path.exists() {
            // Execute the installer with silent install flag
            println!("Launching installer in silent mode...");
            
            match Command::new(installer_path)
                .arg("/S") // Silent install for NSIS installers
                .status() {
                Ok(status) => {
                    if status.success() {
                        println!("Installation completed successfully");
                        
                        // Verify installation by checking if key files exist
                        let install_dir = Path::new(&self.install_path);
                        let exe_path = install_dir.join("zed.exe");
                        if !exe_path.exists() {
                            return Err("Installation verification failed: zed.exe not found".to_string());
                        }
                        
                        Ok(())
                    } else {
                        Err(format!("Installer exited with non-zero status: {}", status))
                    }
                },
                Err(e) => Err(format!("Failed to execute installer: {}", e)),
            }
        } else {
            Err("Installer not found".to_string())
        }
    }
    
    pub fn cleanup(&self) -> Result<(), String> {
        // Optionally remove the downloaded installer after installation
        match fs::remove_file(&self.download_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to remove installer: {}", e)),
        }
    }
}