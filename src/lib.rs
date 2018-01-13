#![feature(box_syntax, fnbox)]

extern crate serenity;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;

pub mod error;
pub mod listeners;
pub mod commands;
