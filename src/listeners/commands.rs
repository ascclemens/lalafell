use bot::Bot;
use listeners::ReceivesEvents;
use commands::*;
use discord::model::{Event, Message, ReactionEmoji};

use std::collections::HashMap;
use std::sync::Arc;

pub struct CommandListener<'a> {
  bot: Arc<Bot + Send + Sync>,
  commands: HashMap<Vec<String>, Box<Command<'a> + Send + Sync>>
}

impl<'a> CommandListener<'a> {
  pub fn new(bot: Arc<Bot + Send + Sync>) -> CommandListener<'a> {
    CommandListener {
      bot: bot,
      commands: Default::default()
    }
  }

  pub fn add_command<T: AsRef<str>>(&mut self, names: &[T], command: Box<Command<'a> + Send + Sync>) {
    self.commands.insert(names.iter().map(|t| t.as_ref().to_string()).collect(), command);
  }

  fn check_command(&self, message: &Message) {
    let parts: Vec<&str> = message.content.split_whitespace().collect();
    if parts.is_empty() {
      return;
    }
    let first = parts[0];
    if !first.starts_with('!') {
      return;
    }
    let command_name = first[1..].to_lowercase();
    let params = &parts[1..];
    let (_, command) = match self.commands.iter().find(|&(names, _)| names.contains(&command_name)) {
      Some(c) => c,
      None => return
    };
    debug!("running command: {}", command_name);
    let run_result = command.run(message, params);
    match run_result {
      Ok(info) => match info.message {
        Some(embed) => { self.bot.discord().send_embed(message.channel_id, "", |e| embed(e).color(0x196358)).ok(); },
        None => { self.bot.discord().add_reaction(message.channel_id, message.id, ReactionEmoji::Unicode("\u{2705}".to_string())).ok(); }
      },
      Err(CommandFailure::Internal(info)) => {
        self.bot.discord().send_embed(message.channel_id, "",
                                    |e| e.description("An internal error happened while processing this command.")).ok();
        for err in info.error.iter() {
          error!("error: {:#?}", err);
        }
      },
      Err(CommandFailure::External(info)) => match info.message {
        Some(embed) => { self.bot.discord().send_embed(message.channel_id, "", |e| embed(e).color(0x63191b)).ok(); },
        None => { self.bot.discord().add_reaction(message.channel_id, message.id, ReactionEmoji::Unicode("\u{274c}".to_string())).ok(); }
      }
    }
  }
}

impl<'a> ReceivesEvents for CommandListener<'a> {
  fn receive(&self, event: &Event) {
    let message = match *event {
      Event::MessageCreate(ref m) => m,
      _ => return
    };
    self.check_command(message);
  }
}
