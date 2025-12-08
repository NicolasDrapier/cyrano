use std::convert::TryFrom;
use crate::utils::{get_field, parse_optional_bool, parse_optional_u8};
use super::enums::{FencerStatus, PCard, Reserve};
use super::error::ParseError;

/// Information about a fencer participating in a match.
///
/// Contains all relevant data about a fencer including their identity, score,
/// penalties, and status indicators.
#[derive(Debug, Clone, Default)]
pub struct Fencer {
    /// Unique identifier for the fencer.
    pub id: Option<String>,
    /// Full name of the fencer.
    pub name: Option<String>,
    /// Three-letter country code of the fencer's nation (e.g., "FRA", "USA").
    pub nation: Option<String>,
    /// Current score in the match.
    pub score: Option<u8>,
    /// Match status (victory, defeat, etc.).
    pub status: Option<FencerStatus>,
    /// Number of yellow cards received (warnings).
    pub yellow_card: Option<u8>,
    /// Number of red cards received (penalty touches).
    pub red_card: Option<u8>,
    /// Whether the fencer's scoring light is on.
    pub light: Option<bool>,
    /// Whether the white light is on (off-target touch).
    pub white_light: Option<bool>,
    /// Fencer medical interventions.
    pub medical: Option<u8>,
    /// Reserve fencer status.
    pub reserve: Option<Reserve>,
    /// Fencer P-Card.
    pub p_card: Option<PCard>,
}

impl Fencer {
    /// Parses fencer data from an array of string fields.
    ///
    /// # Arguments
    ///
    /// * `fields` - Array of string slices containing fencer data in protocol format
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the parsed `Fencer` or a `ParseError` if parsing fails.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if any required field is missing or contains invalid data.
    pub fn parse(fields: &[&str]) -> Result<Self, ParseError> {
        Ok(Fencer {
            id: get_field(fields, 0).map(String::from),
            name: get_field(fields, 1).map(String::from),
            nation: get_field(fields, 2).map(String::from),
            score: parse_optional_u8(fields, 3),
            status: get_field(fields, 4).and_then(|s| FencerStatus::try_from(s).ok()),
            yellow_card: parse_optional_u8(fields, 5),
            red_card: parse_optional_u8(fields, 6),
            light: parse_optional_bool(fields, 7),
            white_light: parse_optional_bool(fields, 8),
            medical: parse_optional_u8(fields, 9),
            reserve: get_field(fields, 10).and_then(|s| Reserve::try_from(s).ok()),
            p_card: get_field(fields, 11).and_then(|s| PCard::try_from(s).ok()),
        })
    }

    /// Serializes the fencer data into protocol format.
    ///
    /// Converts the fencer's data into a pipe-delimited string format according
    /// to the EFP protocol specification. Empty trailing fields are trimmed.
    ///
    /// # Returns
    ///
    /// A `String` containing the serialized fencer data.
    pub fn serialize(&self) -> String {
        let fields: Vec<String> = vec![
            self.id.clone().unwrap_or_default(),
            self.name.clone().unwrap_or_default(),
            self.nation.clone().unwrap_or_default(),
            self.score.map(|v| v.to_string()).unwrap_or_default(),
            self.status.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            self.yellow_card.map(|v| v.to_string()).unwrap_or_default(),
            self.red_card.map(|v| v.to_string()).unwrap_or_default(),
            self.light.map(|v| if v { "1" } else { "0" }.to_string()).unwrap_or_default(),
            self.white_light.map(|v| if v { "1" } else { "0" }.to_string()).unwrap_or_default(),
            self.medical.map(|v| v.to_string()).unwrap_or_default(),
            self.reserve.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            self.p_card.as_ref().map(|v| v.to_string()).unwrap_or_default(),
        ];

        // Remove empty fields at the end
        let trimmed = fields
            .into_iter()
            .rev()
            .skip_while(|s| s.is_empty())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>();

        trimmed.join("|")
    }
}
