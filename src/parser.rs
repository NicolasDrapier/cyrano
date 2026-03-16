use super::error::ParseError;

// General zone field indices (spec uses 1-based numbering)
pub const PROTOCOL: usize = 0;     // Field 1
pub const COMMAND: usize = 1;      // Field 2
pub const PISTE: usize = 2;        // Field 3
pub const COMPETITION: usize = 3;  // Field 4
pub const PHASE: usize = 4;        // Field 5
pub const POOL_TABLEAU: usize = 5; // Field 6
pub const MATCH_NUMBER: usize = 6; // Field 7
pub const ROUND: usize = 7;        // Field 8
pub const TIME: usize = 8;         // Field 9
pub const STOPWATCH: usize = 9;    // Field 10
pub const COMP_TYPE: usize = 10;   // Field 11
pub const WEAPON: usize = 11;      // Field 12
pub const PRIORITY: usize = 12;    // Field 13
pub const STATE: usize = 13;       // Field 14
pub const REF_ID: usize = 14;      // Field 15
pub const REF_NAME: usize = 15;    // Field 16
pub const REF_NATION: usize = 16;  // Field 17

// Fencer zone field indices
pub const FENCER_ID: usize = 0;       // R1/L1
pub const FENCER_NAME: usize = 1;     // R2/L2
pub const FENCER_NATION: usize = 2;   // R3/L3
pub const FENCER_SCORE: usize = 3;    // R4/L4
pub const FENCER_STATUS: usize = 4;   // R5/L5
pub const FENCER_YCARD: usize = 5;    // R6/L6
pub const FENCER_RCARD: usize = 6;    // R7/L7
pub const FENCER_LIGHT: usize = 7;    // R8/L8
pub const FENCER_WLIGHT: usize = 8;   // R9/L9
pub const FENCER_MEDICAL: usize = 9;  // R10/L10
pub const FENCER_RESERVE: usize = 10; // R11/L11
pub const FENCER_PCARD: usize = 11;   // R12/L12

/// Returns the value at the given index, or None if empty or absent.
pub fn get_field<'a>(fields: &'a [&str], index: usize) -> Option<&'a str> {
    fields.get(index).and_then(|s| {
        if s.is_empty() {
            None
        } else {
            Some(*s)
        }
    })
}

/// Returns the value at the given index, or a MissingField error.
pub fn get_required_field<'a>(
    fields: &'a [&str],
    index: usize,
    name: &'static str,
) -> Result<&'a str, ParseError> {
    get_field(fields, index).ok_or(ParseError::MissingField(name))
}

/// Parses an optional numeric value at the given index.
pub fn parse_optional<T: std::str::FromStr>(fields: &[&str], index: usize) -> Option<T> {
    get_field(fields, index).and_then(|s| s.parse().ok())
}

/// Parses a boolean from "0"/"1" at the given index.
pub fn parse_optional_bool(fields: &[&str], index: usize) -> Option<bool> {
    get_field(fields, index).map(|s| s == "1")
}

/// Strips exactly one leading and one trailing '|' from a zone string.
pub fn strip_outer_pipes(s: &str) -> &str {
    let s = s.strip_prefix('|').unwrap_or(s);
    s.strip_suffix('|').unwrap_or(s)
}
