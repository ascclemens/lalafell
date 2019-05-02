use commands::*;

use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;

use quoted_strings::QuotedParts;

use std::collections::HashMap;

pub struct CommandListener<'a> {
  prefix: String,
  commands: HashMap<Vec<String>, Box<Command<'a> + Send + Sync>>
}

impl<'a> CommandListener<'a> {
  pub fn new(prefix: &str) -> Self {
    CommandListener {
      prefix: prefix.to_string(),
      commands: HashMap::default()
    }
  }

  pub fn add_command<T: AsRef<str>>(&mut self, names: &[T], command: Box<Command<'a> + Send + Sync>) {
    self.commands.insert(names.iter().map(|t| t.as_ref().to_string()).collect(), command);
  }
}

impl<'a> EventHandler for CommandListener<'a> {
  fn message(&self, context: Context, message: Message) {
    let strs = QuotedParts::all(&message.content);
    let parts: Vec<&str> = strs.iter().map(|x| x.as_str()).collect();
    if parts.is_empty() {
      return;
    }
    let first = parts[0];
    if !first.starts_with(&self.prefix) {
      return;
    }
    let command_name = first[1..].to_lowercase();
    let params = &parts[1..];
    let (_, command) = match self.commands.iter().find(|&(names, _)| names.contains(&command_name)) {
      Some(c) => c,
      None => return
    };
    debug!("running command: {}", command_name);
    let run_result = command.run(&context, &message, params);
    match run_result {
      Ok(info) => match info.message {
        Some(embed) => { message.channel_id.send_message(&context, |c| c.embed(|e| embed(e).color(0x196358))).ok(); },
        None => { message.react(&context, "\u{2705}").ok(); }
      },
      Err(CommandFailure::Internal(info)) => {
        message.channel_id.send_message(&context, |c| c.embed(|e| e.description("An internal error happened while processing this command."))).ok();
        error!("error during command {} (message {})", command_name, message.id);
        error!("params: {:?}", params);
        for err in info.error.iter() {
          error!("error: {:#?}", err);
        }
      },
      Err(CommandFailure::External(info)) => match info.message {
        Some(embed) => { message.channel_id.send_message(&context, |c| c.embed(|e| embed(e).color(0x63191b))).ok(); },
        None => { message.react(&context, "\u{274c}").ok(); }
      }
    }
  }
}
