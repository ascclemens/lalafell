use ansi_term::Colour;
use fern;
use log::{LogLevel, LogLevelFilter};
use std::io;
use chrono;
use std::env::var;

use error::*;

fn colored_level(level: LogLevel) -> String {
  let color = match level {
    LogLevel::Trace => Colour::Fixed(8),
    LogLevel::Info => Colour::Blue,
    LogLevel::Warn => Colour::Yellow,
    LogLevel::Error => Colour::Red,
    _ => return level.to_string()
  };
  color.paint(level.to_string()).to_string()
}

fn colored_target(target: &str) -> String {
  let parts: Vec<&str> = target.split("::").collect();
  if parts.len() == 1 {
    return target.to_string();
  }
  let base = &parts[..parts.len() - 1];
  let target = &parts[parts.len() - 1];

  let separator = Colour::Fixed(8).paint("::").to_string();
  let mut colored = Vec::new();
  for part in base {
    colored.push(*part);
    colored.push(&separator);
  }
  colored.push(*target);
  colored.join("")
}

pub fn init_logger() -> Result<()> {
  fern::Dispatch::new()
    .format(|out, message, record| {
      out.finish(format_args!("[{}] [{}] {} – {}",
                              chrono::Local::now().format("%H:%M:%S"),
                              colored_level(record.level()),
                              colored_target(record.target()),
                              message))
    })
    .level(if var("LB_DEBUG").is_ok() { LogLevelFilter::Debug } else { LogLevelFilter::Info })
    .chain(io::stdout())
    .apply()
    .chain_err(|| "could not set up logger")
}
