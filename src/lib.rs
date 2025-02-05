//! Library for blackjack clients to interact with the server

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(random)]
#![feature(associated_type_defaults)]

pub mod client;
pub mod messages;
pub mod model;
