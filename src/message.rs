use std::convert::TryFrom;
use std::fmt::Display;

use crate::enums::*;
use crate::error::ParseError;
use crate::fencer::Fencer;
use crate::referee::Referee;
use crate::utils::{get_field, get_required_field, parse_optional_u8};

/// A complete EFP protocol message.
///
/// Represents a parsed message from the Electronic Fencing Protocol (EFP),
/// containing information about a fencing match including referee, fencers,
/// scores, and match state.
///
/// # Message Format
///
/// Messages follow the format: `|general_fields|%|right_fencer|%|left_fencer|%|`
/// where fields are pipe-delimited and sections are separated by percent signs.
///
/// # Examples
///
/// ```
/// use std::convert::TryFrom;
/// use cyrano::message::Message;
///
/// let raw = "|EFP1.1|HELLO|17|fm-eq|%|";
/// let msg = Message::try_from(raw).unwrap();
/// assert_eq!(msg.piste, "17");
/// ```
#[derive(Debug, Clone)]
pub struct Message {
    /// Protocol version (e.g., "EFP1.1" or "EFP1").
    pub protocol: String,
    /// The command type of this message.
    pub command: Command,
    /// Piste (strip) identifier.
    pub piste: String,
    /// Competition identifier.
    pub competition_id: String,
    /// Competition phase number.
    pub phase: Option<u8>,
    /// Pool or tableau identifier.
    pub pool_tableau: Option<String>,
    /// Match number within the competition.
    pub match_number: Option<u8>,
    /// Round number.
    pub round: Option<u8>,
    /// Current match time.
    pub time: Option<String>,
    /// Stopwatch time.
    pub stopwatch: Option<String>,
    /// Type of competition (Individual or Team).
    pub competition_type: Option<CompetitionType>,
    /// Weapon type being used.
    pub weapon: Option<Weapon>,
    /// Priority indicator (for weapons with right-of-way).
    pub priority: Option<Priority>,
    /// Current state of the apparatus.
    pub state: Option<ApparatusState>,
    /// Information about the referee.
    pub referee: Referee,
    /// Information about the fencer on the right.
    pub right_fencer: Fencer,
    /// Information about the fencer on the left.
    pub left_fencer: Fencer,
}

impl TryFrom<&str> for Message {
    type Error = ParseError;

    /// Parses an EFP protocol message from a string slice.
    ///
    /// # Arguments
    ///
    /// * `raw` - The raw protocol message string
    ///
    /// # Returns
    ///
    /// Returns `Ok(Message)` if parsing succeeds.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if:
    /// - The message is empty
    /// - The format is invalid
    /// - Required fields are missing
    /// - The protocol version is not supported (only EFP1 and EFP1.1 are supported)
    /// - Field values are invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use cyrano::message::Message;
    ///
    /// let raw = "|EFP1.1|INFO|17|fm-eq|%|";
    /// let msg = Message::try_from(raw).unwrap();
    /// ```
    fn try_from(raw: &str) -> Result<Self, Self::Error> {
        let raw = raw.trim();

        if raw.is_empty() {
            return Err(ParseError::EmptyMessage);
        }

        let raw = raw.trim_matches('|');
        let zones: Vec<&str> = raw.split('%').collect();

        if zones.is_empty() {
            return Err(ParseError::InvalidFormat);
        }

        let general_fields: Vec<&str> = zones[0].trim_matches('|').split('|').collect();

        let protocol = get_required_field(&general_fields, 0, "protocol")?;
        if protocol != "EFP1.1" && protocol != "EFP1" {
            return Err(ParseError::InvalidProtocol(protocol.to_string()));
        }

        let command = Command::try_from(get_required_field(&general_fields, 1, "command")?)?;
        let piste = get_field(&general_fields, 2).map(String::from).unwrap_or_else(String::new);
        let competition_id = get_field(&general_fields, 3).map(String::from).unwrap_or_else(String::new);

        let phase = parse_optional_u8(&general_fields, 4);
        let pool_tableau = get_field(&general_fields, 5).map(String::from);
        let match_number = parse_optional_u8(&general_fields, 6);
        let round = parse_optional_u8(&general_fields, 7);
        let time = get_field(&general_fields, 8).map(String::from);
        let stopwatch = get_field(&general_fields, 9).map(String::from);
        let competition_type = get_field(&general_fields, 10)
            .and_then(|s| CompetitionType::try_from(s).ok());
        let weapon = get_field(&general_fields, 11).and_then(|s| Weapon::try_from(s).ok());
        let priority = get_field(&general_fields, 12).and_then(|s| Priority::try_from(s).ok());
        let state = get_field(&general_fields, 13)
            .and_then(|s| ApparatusState::try_from(s).ok());

        let referee = Referee {
            id: get_field(&general_fields, 14).map(String::from),
            name: get_field(&general_fields, 15).map(String::from),
            nation: get_field(&general_fields, 16).map(String::from),
        };

        let right_fencer = if zones.len() > 1 {
            let right_fields: Vec<&str> = zones[1].trim_matches('|').split('|').collect();
            Fencer::parse(&right_fields)?
        } else {
            Fencer::default()
        };

        let left_fencer = if zones.len() > 2 {
            let left_fields: Vec<&str> = zones[2].trim_matches('|').split('|').collect();
            Fencer::parse(&left_fields)?
        } else {
            Fencer::default()
        };

        Ok(Message {
            protocol: protocol.to_string(),
            command,
            piste,
            competition_id,
            phase,
            pool_tableau,
            match_number,
            round,
            time,
            stopwatch,
            competition_type,
            weapon,
            priority,
            state,
            referee,
            right_fencer,
            left_fencer,
        })
    }
}

impl TryFrom<String> for Message {
    type Error = ParseError;

    /// Parses an EFP protocol message from an owned `String`.
    ///
    /// This is a convenience implementation that delegates to `TryFrom<&str>`.
    fn try_from(raw: String) -> Result<Self, Self::Error> {
        Message::try_from(raw.as_str())
    }
}

impl Display for Message {
    /// Formats the message as an EFP protocol string.
    ///
    /// Serializes the message back into the pipe-delimited format with
    /// percent-separated zones according to the EFP protocol specification.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // General zone
        let general_fields: Vec<String> = vec![
            self.protocol.clone(),
            self.command.to_string(),
            self.piste.clone(),
            self.competition_id.clone(),
            self.phase.map(|v| v.to_string()).unwrap_or_default(),
            self.pool_tableau.clone().unwrap_or_default(),
            self.match_number.map(|v| v.to_string()).unwrap_or_default(),
            self.round.map(|v| v.to_string()).unwrap_or_default(),
            self.time.clone().unwrap_or_default(),
            self.stopwatch.clone().unwrap_or_default(),
            self.competition_type
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            self.weapon
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            self.priority
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            self.state
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            self.referee.id.clone().unwrap_or_default(),
            self.referee.name.clone().unwrap_or_default(),
            self.referee.nation.clone().unwrap_or_default(),
        ];

        let right_serialized = self.right_fencer.serialize();
        let left_serialized = self.left_fencer.serialize();

        // Build the complete message
        write!(
            f,
            "|{}|%|{}|%|{}|%|",
            general_fields.join("|"),
            right_serialized,
            left_serialized
        )
    }
}

impl From<Message> for String {
    /// Converts a `Message` into its protocol string representation.
    ///
    /// This is a convenience implementation that uses the `Display` trait.
    fn from(msg: Message) -> Self {
        msg.to_string()
    }
}

// ===== TESTS =====

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hello() {
        let raw = "|EFP1.1|HELLO|17|fm-eq|%|";
        let msg = Message::try_from(raw).unwrap();

        assert_eq!(msg.command, Command::Hello);
        assert_eq!(msg.piste, "17");
        assert_eq!(msg.competition_id, "fm-eq");
    }

    #[test]
    fn test_parse_info_complete() {
        let raw = "|EFP1.1|INFO|17|efj-eq|1|A32|12|2|10:30|3:00|I|S||W|132|J.Smith|GBR|%|28|P.Martin|FRA|8|V|0|1|1|0|0|N|%|32|B. Panini|ITA|6|D|0|1|0|0|0|N|%|";
        let msg = Message::try_from(raw).unwrap();

        assert_eq!(msg.command, Command::Info);
        assert_eq!(msg.piste, "17");
        assert_eq!(msg.phase, Some(1));
        assert_eq!(msg.pool_tableau, Some("A32".to_string()));
        assert_eq!(msg.state, Some(ApparatusState::Waiting));
        assert_eq!(msg.weapon, Some(Weapon::Sabre));

        assert_eq!(msg.right_fencer.id, Some("28".to_string()));
        assert_eq!(msg.right_fencer.name, Some("P.Martin".to_string()));
        assert_eq!(msg.right_fencer.score, Some(8));
        assert_eq!(msg.right_fencer.status, Some(FencerStatus::Victory));

        assert_eq!(msg.left_fencer.id, Some("32".to_string()));
        assert_eq!(msg.left_fencer.score, Some(6));
        assert_eq!(msg.left_fencer.status, Some(FencerStatus::Defeat));
    }

    #[test]
    fn test_parse_info_incomplete() {
        let raw = "|EFP1.1|INFO||||||||||||W||%|";
        let msg = Message::try_from(raw).unwrap();

        assert_eq!(msg.command, Command::Info);
        assert_eq!(msg.piste, "");
        assert_eq!(msg.competition_id, "");
        assert_eq!(msg.state, Some(ApparatusState::Waiting));
    }

    #[test]
    fn test_roundtrip_hello() {
        let raw = "|EFP1.1|HELLO|17|fm-eq|%|";
        let msg = Message::try_from(raw).unwrap();
        let serialized = msg.to_string();
        let reparsed = Message::try_from(serialized.as_str()).unwrap();

        assert_eq!(msg.command, reparsed.command);
        assert_eq!(msg.piste, reparsed.piste);
        assert_eq!(msg.competition_id, reparsed.competition_id);
    }

    #[test]
    fn test_invalid_command() {
        let raw = "|EFP1.1|INVALID|17|fm-eq|%|";
        let result = Message::try_from(raw);
        assert!(matches!(result, Err(ParseError::InvalidCommand(_))));
    }

    #[test]
    fn test_invalid_protocol() {
        let raw = "|EFP2.0|HELLO|17|fm-eq|%|";
        let result = Message::try_from(raw);
        assert!(matches!(result, Err(ParseError::InvalidProtocol(_))));
    }
}
