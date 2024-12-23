//! The definition of all messages used in the protocol

use generate::Generate;
use parse::{Parse, Parser};
use r#macro::messages;
use std::num::NonZeroU16;

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

/// The current version of the protocol being used
pub const CURRENT_PROTOCOL_VERSION: u32 = 1;

/// The default port to listen for clients on
pub const DEFAULT_PORT: NonZeroU16 = unsafe { NonZeroU16::new_unchecked(9261) };
