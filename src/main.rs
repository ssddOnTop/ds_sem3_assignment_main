use crate::cli::CLIError;
use anyhow::Result;

mod cli;
mod computation;
mod config;
mod coordinate_parser;

fn main() -> Result<()> {
    let result = cli::run();
    match result {
        Ok(_) => {}
        Err(error) => {
            // Ensure all errors are converted to CLIErrors before being printed.
            let cli_error = match error.downcast::<CLIError>() {
                Ok(cli_error) => cli_error,
                Err(error) => {
                    let sources = error
                        .source()
                        .map(|error| vec![CLIError::new(error.to_string().as_str())])
                        .unwrap_or_default();

                    CLIError::new(&error.to_string()).caused_by(sources)
                }
            };
            eprintln!("{}", cli_error.color(true));
            std::process::exit(1);
        }
    }
    Ok(())
}
