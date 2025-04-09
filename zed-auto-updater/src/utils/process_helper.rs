use std::process::Command;

#[cfg(windows)]
pub fn is_process_running(process_name: &str) -> bool {
    let output = Command::new("tasklist")
        .output()
        .expect("Failed to execute tasklist command");
    
    let processes = String::from_utf8_lossy(&output.stdout);
    processes.contains(process_name)
}

#[cfg(windows)]
pub fn kill_process(process_name: &str) -> Result<(), String> {
    let output = Command::new("taskkill")
        .args(&["/F", "/IM", process_name])
        .output();
    
    match output {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to terminate process: {}", e)),
    }
}

#[cfg(not(windows))]
pub fn is_process_running(_process_name: &str) -> bool {
    false // Not implemented for non-Windows platforms
}

#[cfg(not(windows))]
pub fn kill_process(_process_name: &str) -> Result<(), String> {
    Ok(()) // Not implemented for non-Windows platforms
}