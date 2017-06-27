use discord::model::Event;

pub mod commands;

pub use self::commands::CommandListener;

pub trait ReceivesEvents {
  fn receive(&self, event: &Event);
}
