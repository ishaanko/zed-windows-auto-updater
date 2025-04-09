pub mod progress;

#[cfg(windows)]
use winapi::um::winuser::{MessageBoxA, MB_OK, MB_ICONINFORMATION, MB_ICONERROR};
#[cfg(windows)]
use std::ffi::CString;
#[cfg(windows)]
use std::ptr::null_mut;

pub fn show_update_notification() {
    println!("\n=================================");
    println!("| New Zed update is available! |");
    println!("| Downloading update...        |");
    println!("=================================\n");
    
    #[cfg(windows)]
    unsafe {
        let title = CString::new("Zed Auto Updater").unwrap();
        let text = CString::new("A new version of Zed is available. Downloading update...").unwrap();
        MessageBoxA(null_mut(), text.as_ptr(), title.as_ptr(), MB_OK | MB_ICONINFORMATION);
    }
}

pub fn show_error_message(message: &str) {
    eprintln!("\n=================================");
    eprintln!("| ERROR: {}   ", message);
    eprintln!("=================================\n");
    
    #[cfg(windows)]
    unsafe {
        let title = CString::new("Zed Auto Updater - Error").unwrap();
        let text = CString::new(message).unwrap();
        MessageBoxA(null_mut(), text.as_ptr(), title.as_ptr(), MB_OK | MB_ICONERROR);
    }
}

pub fn show_success_message(message: &str) {
    println!("\n=================================");
    println!("| SUCCESS: {}   ", message);
    println!("=================================\n");
    
    #[cfg(windows)]
    unsafe {
        let title = CString::new("Zed Auto Updater").unwrap();
        let text = CString::new(message).unwrap();
        MessageBoxA(null_mut(), text.as_ptr(), title.as_ptr(), MB_OK | MB_ICONINFORMATION);
    }
}