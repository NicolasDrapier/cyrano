use std::error::Error;
use std::fmt::Display;

/// Errors that can occur when parsing EFP protocol messages.
///
/// This enum represents all possible parsing errors that can occur when converting
/// raw protocol strings into structured message types.
#[derive(Debug)]
pub enum ParseError {
    /// The input message string is empty.
    EmptyMessage,
    /// The message format is invalid (e.g., malformed CSV structure).
    InvalidFormat,
    /// A required field is missing from the message.
    ///
    /// The `&'static str` contains the name of the missing field.
    MissingField(&'static str),
    /// The command field contains an unrecognized command.
    ///
    /// The `String` contains the invalid command value.
    InvalidCommand(String),
    /// The protocol version is not supported.
    ///
    /// The `String` contains the unsupported protocol version.
    InvalidProtocol(String),
    /// A field contains an invalid value.
    ///
    /// Contains both the field name and the invalid value.
    InvalidValue { field: &'static str, value: String },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EmptyMessage => write!(f, "Empty message"),
            ParseError::InvalidFormat => write!(f, "Invalid CSV format"),
            ParseError::MissingField(field) => write!(f, "Required field missing: {}", field),
            ParseError::InvalidCommand(cmd) => write!(f, "Invalid command: {}", cmd),
            ParseError::InvalidProtocol(proto) => write!(f, "Invalid protocol: {}", proto),
            ParseError::InvalidValue { field, value } => {
                write!(f, "Invalid value for {}: {}", field, value)
            }
        }
    }
}

impl Error for ParseError {}
