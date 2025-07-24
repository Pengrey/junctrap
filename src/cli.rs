use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    // Turn debugging information on
    #[arg(short, long, help = "Enable debug logging")]
    pub debug: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates junction and spoofs .lnk
    Trap,

    /// Retrieves cookies from chrome
    Dump,

    /// Restores .lnk and deletes junction
    Restore,
}
