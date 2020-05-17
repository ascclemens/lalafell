pub mod params;
pub mod prelude;

pub use self::params::helpers::mention::MentionOrId;
pub use self::params::helpers::channel::ChannelOrId;
pub use self::params::helpers::role::RoleOrId;

use error::{self, ResultExt};

use serenity::client::Context;
use serenity::model::channel::{Message, Channel, GuildChannel};
use serenity::model::id::GuildId;
use serenity::builder::CreateEmbed;

use structopt::StructOpt;
use structopt::clap::{App, AppSettings};

use std::sync::Arc;
use serenity::prelude::RwLock;

pub type CommandResult<'a> = Result<CommandSuccess<'a>, CommandFailure<'a>>;

pub const TEMPLATE: &str = "{about}\n\nUSAGE:\n    **{usage}**\n\n{all-args}";

pub trait Command<'a> {
  fn run(&self, context: &Context, message: &Message, params: &[&str]) -> CommandResult<'a>;
}

pub trait PublicChannelCommand<'a> {
  fn run(&self, context: &Context, message: &Message, guild: GuildId, channel: Arc<RwLock<GuildChannel>>, params: &[&str]) -> CommandResult<'a>;
}

pub trait HasParams {
  type Params: StructOpt;

  fn params<'a>(&self, name: &str, params: &[&str]) -> Result<Self::Params, CommandFailure<'a>> {
    self.params_then(name, params, |x| x)
  }

  fn params_then<'a, 'b, 'c, F>(&self, name: &str, params: &[&str], then: F) -> Result<Self::Params, CommandFailure<'a>>
    where F: FnOnce(App<'b, 'c>) -> App<'b, 'c>,
          'b: 'c
  {
    let prefixed_name = format!("!{}", name);
    let prefixed_name = prefixed_name.as_str();
    let params = then(Self::Params::clap()
      .global_settings(&[
        AppSettings::DeriveDisplayOrder,
        AppSettings::DisableVersion,
        AppSettings::NextLineHelp,
      ])
      // TODO: raw(setting = "::structopt::clap::AppSettings::ArgRequiredElseHelp")
      //       https://github.com/kbknapp/clap-rs/issues/1183 blocked until clap 3.x
      .template(TEMPLATE))
      .name(prefixed_name)
      .get_matches_from_safe([prefixed_name].iter().chain(params));
    match params {
      Ok(p) => Ok(Self::Params::from_clap(&p)),
      Err(e) => Err(e.to_string().into())
    }
  }
}

impl<'a, T> Command<'a> for T
  where T: PublicChannelCommand<'a>
{
  fn run(&self, context: &Context, message: &Message, params: &[&str]) -> CommandResult<'a> {
    let channel = message.channel_id.to_channel(context).chain_err(|| "could not get channel for message")?;
    let public_channel = match channel {
      Channel::Guild(c) => c,
      _ => return Err("This command must be run in a public channel.".into())
    };
    let guild_id = public_channel.read().guild_id;
    self.run(context, message, guild_id, public_channel, params)
  }
}

#[derive(Default)]
pub struct CommandSuccess<'a> {
  pub message: Option<Box<dyn Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'a>>,
}

impl<'a> CommandSuccess<'a> {
  pub fn message<F>(mut self, message: F) -> Self
  where F: 'a + Fn(&mut CreateEmbed) -> &mut CreateEmbed,
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
      .message(move |e: &mut CreateEmbed| e.description(&message))
  }
}

pub enum CommandFailure<'a> {
  Internal(InternalCommandFailure),
  External(ExternalCommandFailure<'a>),
}

#[derive(Default)]
pub struct ExternalCommandFailure<'a> {
  pub message: Option<Box<dyn Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'a>>,
}

impl<'a> ExternalCommandFailure<'a> {
  pub fn message<F>(mut self, message: F) -> Self
  where F: 'a + Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'static,
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
      .message(move |e: &mut CreateEmbed| e.description(&message))
      .wrap()
  }
}

#[derive(Debug)]
pub struct InternalCommandFailure {
  pub error: error::Error
}

impl<'a> From<error::Error> for CommandFailure<'a> {
  fn from(error: error::Error) -> Self {
    CommandFailure::Internal(InternalCommandFailure { error })
  }
}
