use std::fmt::{Debug, Display};

use colored::Colorize;
use derive_setters::Setters;
use thiserror::Error;

#[derive(Debug, Error, Setters)]
pub struct CLIError {
  is_root: bool,
  #[setters(skip)]
  color: bool,
  message: String,
  #[setters(strip_option)]
  description: Option<String>,
  trace: Vec<String>,

  #[setters(skip)]
  caused_by: Vec<CLIError>,
}

impl CLIError {
  pub fn new(message: &str) -> Self {
    CLIError {
      is_root: true,
      color: false,
      message: message.to_string(),
      description: Default::default(),
      trace: Default::default(),
      caused_by: Default::default(),
    }
  }

  pub fn caused_by(mut self, error: Vec<CLIError>) -> Self {
    self.caused_by = error;

    for error in self.caused_by.iter_mut() {
      error.is_root = false;
    }

    self
  }

  fn colored<'a>(&'a self, str: &'a str, color: colored::Color) -> String {
    if self.color {
      str.color(color).to_string()
    } else {
      str.to_string()
    }
  }

  fn dimmed<'a>(&'a self, str: &'a str) -> String {
    if self.color {
      str.dimmed().to_string()
    } else {
      str.to_string()
    }
  }

  pub fn color(mut self, color: bool) -> Self {
    self.color = color;
    for inner in self.caused_by.iter_mut() {
      inner.color = color;
    }
    self
  }
}

fn margin(str: &str, margin: usize) -> String {
  let mut result = String::new();
  for line in str.split_inclusive('\n') {
    result.push_str(&format!("{}{}", " ".repeat(margin), line));
  }
  result
}

fn bullet(str: &str) -> String {
  let mut chars = margin(str, 2).chars().collect::<Vec<char>>();
  chars[0] = '•';
  chars[1] = ' ';
  chars.into_iter().collect::<String>()
}

impl Display for CLIError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let error_prefix = "Error: ";
    let default_padding = 2;
    let root_padding_size = if self.is_root {
      error_prefix.len()
    } else {
      default_padding
    };

    if self.is_root {
      f.write_str(self.colored(error_prefix, colored::Color::Red).as_str())?;
    }

    f.write_str(&self.message.to_string())?;

    if let Some(description) = &self.description {
      f.write_str("\n")?;
      let color = if self.is_root {
        colored::Color::Yellow
      } else {
        colored::Color::White
      };
      f.write_str(
        margin(
          &self.colored(format!("❯ {}", description).as_str(), color),
          root_padding_size,
        )
        .as_str(),
      )?;
    }

    if !self.trace.is_empty() {
      let mut buf = String::new();
      buf.push_str(" [at ");
      let len = self.trace.len();
      for (i, trace) in self.trace.iter().enumerate() {
        buf.push_str(&trace.to_string());
        if i < len - 1 {
          buf.push('.');
        }
      }
      buf.push(']');
      f.write_str(&self.colored(&buf, colored::Color::Cyan))?;
    }

    if !self.caused_by.is_empty() {
      f.write_str(self.dimmed("\nCaused by:\n").as_str())?;
      for (i, error) in self.caused_by.iter().enumerate() {
        let message = &error.to_string();
        f.write_str(&margin(bullet(message.as_str()).as_str(), default_padding))?;

        if i < self.caused_by.len() - 1 {
          f.write_str("\n")?;
        }
      }
    }

    Ok(())
  }
}

impl From<std::io::Error> for CLIError {
  fn from(error: std::io::Error) -> Self {
    let cli_error = CLIError::new("IO Error");
    let message = error.to_string();

    cli_error.description(message)
  }
}

impl From<Box<dyn std::error::Error>> for CLIError {
  fn from(value: Box<dyn std::error::Error>) -> Self {
    CLIError::new(value.to_string().as_str())
  }
}