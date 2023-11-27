use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Idk", author = "Sandipsinh Rathod")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Starts the GraphQL server on the configured port
    Start {
        /// Path for the configuration files config files separated by spaces if more than one
        #[arg(required = true)]
        file_path: Vec<String>,
        #[arg(long)]
        log_level: Option<log::Level>,
    },

    /// Validate a composition spec
    Check {
        /// Path for the configuration files separated by spaces if more than one
        #[arg(required = true)]
        file_path: Vec<String>,
    },
}
