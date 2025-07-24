use std::fs;
use std::path::PathBuf;

pub fn create_junc(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let junc_path: PathBuf = PathBuf::from(format!("C:\\temp\\{}", username));
    let chrome_data_path: PathBuf = PathBuf::from(&format!("C:\\Users\\{}\\AppData\\Local\\Google\\Chrome\\User Data", username));

    if !junction::exists(&junc_path)? {
        junction::create(chrome_data_path, &junc_path)?;
    }

    Ok(())
}

pub fn remove_junc(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let junc_path: PathBuf = PathBuf::from(format!("C:\\temp\\{}", username));

    if junction::exists(&junc_path)? {
        junction::delete(&junc_path)?;
        fs::remove_dir(&junc_path)?;
    }

    Ok(())
}
