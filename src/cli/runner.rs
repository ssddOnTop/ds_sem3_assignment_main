use std::sync::Arc;
use crate::cli::command::{Cli, Command};
use crate::computation::compute::compute_conf;
use anyhow::Result;
use clap::Parser;
use log::Level;
use tokio::sync::RwLock;
use crate::web::handeler;

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Start {
            file_path,
            log_level,
        } => {
            env_logger::Builder::new()
                .filter_level(log_level.unwrap_or(Level::Info).to_level_filter())
                .init();
            let config = compute_conf(file_path.iter())?;
            log::info!("Started server at port 8080");
            handeler::init(Arc::new(RwLock::new(config))).await?;
        }
        Command::Check { file_path: _ } => {}
    }
    Ok(())
}
