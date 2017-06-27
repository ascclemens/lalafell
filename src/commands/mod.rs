pub mod params;
pub mod prelude;

pub use self::params::helpers::mention::MentionOrId;
pub use self::params::helpers::channel::ChannelOrId;

use bot::Bot;

use error::{self, ResultExt};

use discord::model::{Message, LiveServer, Channel, PublicChannel};
use discord::builders::EmbedBuilder;

use serde::de::DeserializeOwned;

use std::boxed::FnBox;

pub type CommandResult<'a> = Result<CommandSuccess<'a>, CommandFailure<'a>>;

pub trait Command<'a> {
  fn run(&self, message: &Message, params: &[&str]) -> CommandResult<'a>;
}

pub trait PublicChannelCommand<'a> {
  fn run(&self, message: &Message, server: &LiveServer, channel: &PublicChannel, params: &[&str]) -> CommandResult<'a>;
}

pub trait HasBot {
  fn bot(&self) -> &Bot;
}

pub trait HasParams {
  type Params: DeserializeOwned;

  fn params<'a>(&self, usage: &str, params: &[&str]) -> Result<Self::Params, CommandFailure<'a>> {
    let string = params.join(" ");
    match params::from_str(&string) {
      Ok(p) => Ok(p),
      Err(::commands::params::error::Error::MissingParams) => {
        let usage = usage.to_owned();
        Err(ExternalCommandFailure::default()
          .message(move |e: EmbedBuilder| e
            .title("Not enough parameters.")
            .description(&usage))
          .wrap())
      },
      Err(e) => {
        // I promise there's a better way, but I can't figure it out right now
        let message = format!("{}", e);
        if message.starts_with("could not parse target: ") {
          Err(ExternalCommandFailure::default()
            .message(|e: EmbedBuilder| e
              .title("Invalid target.")
              .description("The target was not a mention, and it was not a user ID."))
            .wrap())
        } else if message.starts_with("could not parse channel: ") {
          Err(ExternalCommandFailure::default()
            .message(|e: EmbedBuilder| e
              .title("Invalid channel.")
              .description("The channel was not a channel reference, and it was not a channel ID."))
            .wrap())
        } else {
          Err(e).chain_err(|| "could not parse params")?
        }
      }
    }
  }
}

impl<'a, T> Command<'a> for T
  where T: PublicChannelCommand<'a> + HasBot
{
  fn run(&self, message: &Message, params: &[&str]) -> CommandResult<'a> {
    let channel = self.bot().discord().get_channel(message.channel_id).chain_err(|| "could not get channel for message")?;
    let public_channel = match channel {
      Channel::Public(c) => c,
      _ => return Err("This command must be run in a public channel.".into())
    };
    let server_id = public_channel.server_id;
    let server = {
      let bot = self.bot();
      let state_option = bot.state().read().unwrap();
      let state = state_option.as_ref().unwrap();
      match state.servers().iter().find(|x| x.id == server_id) {
        Some(s) => s.clone(),
        None => {
          let err: error::Error = "could not find server for channel".into();
          return Err(err.into());
        }
      }
    };
    self.run(message, &server, &public_channel, params)
  }
}

#[derive(Default)]
pub struct CommandSuccess<'a> {
  pub message: Option<Box<FnBox(EmbedBuilder) -> EmbedBuilder + 'a>>
}

impl<'a> CommandSuccess<'a> {
  pub fn message<F>(mut self, message: F) -> Self
    where F: 'a + FnBox(EmbedBuilder) -> EmbedBuilder
  {
    self.message = Some(box message);
    self
  }
}

impl<'a, T> From<T> for CommandSuccess<'a>
  where T: AsRef<str>
{
  fn from(message: T) -> Self {
    let message = message.as_ref().to_string();
    CommandSuccess::default()
      .message(move |e: EmbedBuilder| e.description(&message))
  }
}

pub enum CommandFailure<'a> {
  Internal(InternalCommandFailure),
  External(ExternalCommandFailure<'a>)
}

#[derive(Default)]
pub struct ExternalCommandFailure<'a> {
  pub message: Option<Box<FnBox(EmbedBuilder) -> EmbedBuilder + 'a>>
}

impl<'a> ExternalCommandFailure<'a> {
  pub fn message<F>(mut self, message: F) -> Self
    where F: 'a + FnBox(EmbedBuilder) -> EmbedBuilder + 'static
  {
    self.message = Some(box message);
    self
  }

  pub fn wrap(self) -> CommandFailure<'a> {
    CommandFailure::External(self)
  }
}

impl<'a, T> From<T> for CommandFailure<'a>
  where T: AsRef<str>
{
  fn from(message: T) -> Self {
    let message = message.as_ref().to_string();
    ExternalCommandFailure::default()
      .message(move |e: EmbedBuilder| e.description(&message))
      .wrap()
  }
}

#[derive(Debug)]
pub struct InternalCommandFailure {
  pub error: error::Error
}

impl<'a> From<error::Error> for CommandFailure<'a> {
  fn from(error: error::Error) -> Self {
    CommandFailure::Internal(InternalCommandFailure { error: error })
  }
}
