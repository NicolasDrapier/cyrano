//! # Cyrano
//!
//! A Rust library for parsing and serializing Ethernet Fencing Protocol (EFP) messages.
//!
//! This crate provides types and parsers for working with the EFP protocol versions 1.0 and 1.1,
//! which is used by electronic scoring equipment in fencing competitions.
//!
//! ## Features
//!
//! - Parse EFP protocol messages from strings
//! - Serialize messages back to protocol format
//! - Type-safe representation of all protocol fields
//! - Support for all fencing weapons (Foil, Épée, Sabre)
//! - Comprehensive error handling
//!
//! ## Quick Start
//!
//! ```
//! use std::convert::TryFrom;
//! use cyrano::message::Message;
//!
//! // Parse a protocol message
//! let raw = "|EFP1.1|HELLO|17|fm-eq|%|";
//! let message = Message::try_from(raw).unwrap();
//!
//! // Access message fields
//! assert_eq!(message.piste, "17");
//! assert_eq!(message.competition_id, "fm-eq");
//!
//! // Serialize back to string
//! let serialized = message.to_string();
//! ```
//!
//! ## Protocol Format
//!
//! EFP messages are structured as pipe-delimited fields with three zones separated
//! by percent signs:
//!
//! ```text
//! |general_fields|%|right_fencer_fields|%|left_fencer_fields|%|
//! ```
//!
//! ## Modules
//!
//! - [`message`] - The main `Message` type and parsing logic
//! - [`error`] - Error types for parsing failures
//! - [`enums`] - Enumerations for protocol values (commands, weapons, states, etc.)
//! - [`fencer`] - Fencer information and data structures
//! - [`referee`] - Referee information
//!
//! ## Examples
//!
//! ### Parsing a Complete Match Message
//!
//! ```
//! use std::convert::TryFrom;
//! use cyrano::message::Message;
//! use cyrano::enums::{Command, Weapon, FencerStatus};
//!
//! let raw = "|EFP1.1|INFO|17|efj-eq|1|A32|12|2|10:30|3:00|I|S||W|132|J.Smith|GBR|%|28|P.Martin|FRA|8|V|0|1|1|0|0|N|%|32|B. Panini|ITA|6|D|0|1|0|0|0|N|%|";
//! let msg = Message::try_from(raw).unwrap();
//!
//! assert_eq!(msg.command, Command::Info);
//! assert_eq!(msg.weapon, Some(Weapon::Sabre));
//! assert_eq!(msg.right_fencer.score, Some(8));
//! assert_eq!(msg.right_fencer.status, Some(FencerStatus::Victory));
//! ```

pub mod message;
pub mod error;
pub mod enums;
pub mod fencer;
pub mod referee;
mod utils;

// Re-export main types for convenience
pub use message::Message;
pub use error::ParseError;
pub use referee::Referee;
pub use fencer::Fencer;
