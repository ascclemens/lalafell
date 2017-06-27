use discord::{Discord, State};

use listeners::ReceivesEvents;

use std::sync::RwLock;

pub trait Bot {
  fn discord(&self) -> &Discord;

  fn discord_mut(&mut self) -> &mut Discord;

  fn state(&self) -> &RwLock<Option<State>>;

  fn listeners(&self) -> &RwLock<Vec<Box<ReceivesEvents + Send + Sync>>>;
}
