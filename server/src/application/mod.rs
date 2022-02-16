mod commands;

use crate::prelude::*;
use clap::{
    AppSettings,
    Parser,
    Subcommand,
};
pub use commands::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
pub struct App {
    #[clap(subcommand)]
    command: Commands,
    /// Configuration file to load
    #[clap(short, long)]
    config_file: Option<PathBuf>,
}

impl App {
    #[track_caller]
    pub fn init() -> Result<Self> {
        color_eyre::install()?;
        dotenv::dotenv().ok();
        let app = Self::parse();
        Ok(app)
    }

    pub async fn run(self) -> Result<Unit> {
        Ok(())
    }
}
