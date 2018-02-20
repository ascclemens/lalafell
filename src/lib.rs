#![feature(box_syntax, fnbox)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate serenity;
extern crate structopt;
extern crate quoted_strings;

pub mod error;
pub mod listeners;
pub mod commands;
