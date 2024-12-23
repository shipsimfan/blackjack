//! The definition of all messages used in the protocol

use generate::Generate;
use parse::{Parse, Parser};
use r#macro::messages;

mod client;
mod generate;
mod r#macro;
mod parse;
mod server;
mod version;

pub mod header;

pub use client::*;
pub use parse::ParseMessageError;
pub use server::*;
pub use version::Version;

const CURRENT_PROTOCOL_VERSION: u32 = 1;
