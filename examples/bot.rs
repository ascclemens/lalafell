extern crate discord;
extern crate lalafell;

use discord::{Discord, State};
use lalafell::bot::Bot;
use lalafell::listeners::{ReceivesEvents, CommandListener};

use std::sync::{Arc, RwLock};
use std::env::var;

struct MyBot {
  discord: Discord,
  state: RwLock<Option<State>>,
  listeners: RwLock<Vec<Box<ReceivesEvents + Send + Sync>>>
}

impl Bot for MyBot {
  fn discord(&self) -> &Discord {
    &self.discord
  }

  fn discord_mut(&mut self) -> &mut Discord {
    &mut self.discord
  }

  fn state(&self) -> &RwLock<Option<State>> {
    &self.state
  }

  fn listeners(&self) -> &RwLock<Vec<Box<ReceivesEvents + Send + Sync>>> {
    &self.listeners
  }
}

fn main() {
  let token = var("LALAFELL_TOKEN").unwrap();

  // Bot setup
  let discord = Discord::from_bot_token(&token).unwrap();
  let bot = Arc::new(MyBot { discord: discord, state: RwLock::default(), listeners: RwLock::default() });

  // Command setup
  let command_listener = CommandListener::new(bot.clone());
  // add some commands
  {
    let mut listeners = bot.listeners.write().unwrap();
    listeners.push(Box::new(command_listener));
  }

  // Connection
  let (mut connection, ready) = bot.discord.connect().unwrap();
  let state = State::new(ready);
  *bot.state.write().unwrap() = Some(state);

  // Event loop
  loop {
    let event = connection.recv_event().unwrap();
    {
      let mut state_option = bot.state.write().unwrap();
      let mut state = state_option.as_mut().unwrap();
      state.update(&event);
    }

    {
      let listeners = bot.listeners.read().unwrap();
      for listener in listeners.iter() {
        listener.receive(&event);
      }
    }
  }
}
