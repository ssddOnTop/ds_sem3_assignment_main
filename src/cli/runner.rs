use anyhow::{Result};
use clap::Parser;
use log::Level;
use crate::cli::command::{Cli, Command};
use crate::computation::compute::compute_conf;
// use crate::computation::compute::compute;

pub fn run() -> Result<()>{
    let cli = Cli::parse();
    match cli.command {
        Command::Start { file_path, log_level } => {
            env_logger::Builder::new()
                .filter_level(log_level.unwrap_or(Level::Info).to_level_filter())
                .init();
            let config = compute_conf(file_path.iter())?;
            let ans = config.compute();
            for c in ans {
                println!("{} ",config.get(&c));
            }
        }
        Command::Check { file_path } => {

        }
    }
    Ok(())
}