#![feature(box_syntax, fnbox)]

extern crate serenity;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate fern;
#[macro_use]
extern crate error_chain;
extern crate chrono;
extern crate ansi_term;

pub mod error;
pub mod logging;
pub mod listeners;
pub mod commands;
