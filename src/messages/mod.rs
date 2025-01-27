//! The definition of all messages used in the protocol

use r#macro::messages;
use std::num::NonZeroU16;

mod client;
mod generate;
mod r#macro;
mod parse;
mod ref_cow;
mod server;
mod version;

pub mod header;

pub(crate) use generate::Generate;
pub(crate) use parse::{Parse, Parser};

pub use client::*;
pub use parse::ParseMessageError;
pub use ref_cow::RefCow;
pub use server::*;
pub use version::Version;

/// The current version of the protocol being used
pub const CURRENT_PROTOCOL_VERSION: u32 = 1;

/// The default port to listen for clients on
pub const DEFAULT_PORT: NonZeroU16 = unsafe { NonZeroU16::new_unchecked(9261) };
