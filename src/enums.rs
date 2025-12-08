use std::convert::TryFrom;
use std::fmt::Display;

use super::error::ParseError;

// ===== ENUMS =====

/// Protocol command types supported by the EFP protocol.
///
/// These commands define the type of message being sent or received.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Initial handshake command.
    Hello,
    /// Display command.
    Disp,
    /// Acknowledgment command.
    Ack,
    /// Negative acknowledgment command.
    Nak,
    /// Information message containing match data.
    Info,
    /// Move to next match.
    Next,
    /// Move to previous match.
    Prev,
}

/// Type of fencing competition.
#[derive(Debug, Clone, PartialEq)]
pub enum CompetitionType {
    /// Individual competition (one fencer per side).
    Individual,
    /// Team competition.
    Team,
}

/// Type of fencing weapon.
#[derive(Debug, Clone, PartialEq)]
pub enum Weapon {
    /// Foil weapon.
    Foil,
    /// Épée weapon.
    Epee,
    /// Sabre weapon.
    Sabre,
}

/// Priority indicator in fencing, typically used in sabre.
///
/// Indicates which fencer has priority (right of way) in the current action.
#[derive(Debug, Clone, PartialEq)]
pub enum Priority {
    /// No priority assigned.
    None,
    /// Right fencer has priority.
    Right,
    /// Left fencer has priority.
    Left,
}

/// Current state of the fencing apparatus/scoring machine.
#[derive(Debug, Clone, PartialEq)]
pub enum ApparatusState {
    /// Fencing is in progress.
    Fencing,
    /// Match is halted.
    Halt,
    /// Match is paused.
    Pause,
    /// Apparatus is waiting for match to begin.
    Waiting,
    /// Match is ending.
    Ending,
}

/// Status of a fencer in the match.
#[derive(Debug, Clone, PartialEq)]
pub enum FencerStatus {
    /// Status is undefined (match in progress).
    Undefined,
    /// Fencer has won the match.
    Victory,
    /// Fencer has lost the match.
    Defeat,
    /// Fencer has abandoned the match.
    Abandonment,
    /// Fencer has been excluded from the match.
    Exclusion,
}

/// Reserve fencer status indicator.
#[derive(Debug, Clone, PartialEq)]
pub enum Reserve {
    /// No reserve status.
    None,
    /// Reserve fencer to be introduced.
    Introduce,
}

/// Penalty card status for a fencer.
///
/// Represents the cumulative penalty cards a fencer has received.
#[derive(Debug, Clone, PartialEq)]
pub enum PCard {
    /// No penalty cards.
    None,
    /// Yellow card (warning).
    Yellow,
    /// First red card (penalty touch).
    OneRed,
    /// Second red card (two penalty touches).
    TwoRed,
    /// First black card (exclusion).
    OneBlack,
    /// Second black card (expulsion).
    TwoBlack,
}

// ===== IMPL PARSING ENUMS =====

impl TryFrom<&str> for Command {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "HELLO" => Ok(Command::Hello),
            "DISP" => Ok(Command::Disp),
            "ACK" => Ok(Command::Ack),
            "NAK" => Ok(Command::Nak),
            "INFO" => Ok(Command::Info),
            "NEXT" => Ok(Command::Next),
            "PREV" => Ok(Command::Prev),
            _ => Err(ParseError::InvalidCommand(value.to_string())),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Hello => write!(f, "HELLO"),
            Command::Disp => write!(f, "DISP"),
            Command::Ack => write!(f, "ACK"),
            Command::Nak => write!(f, "NAK"),
            Command::Info => write!(f, "INFO"),
            Command::Next => write!(f, "NEXT"),
            Command::Prev => write!(f, "PREV"),
        }
    }
}

impl TryFrom<&str> for CompetitionType {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "I" => Ok(CompetitionType::Individual),
            "T" => Ok(CompetitionType::Team),
            _ => Err(ParseError::InvalidValue {
                field: "competition_type",
                value: value.to_string(),
            }),
        }
    }
}

impl Display for CompetitionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompetitionType::Individual => write!(f, "I"),
            CompetitionType::Team => write!(f, "T"),
        }
    }
}

impl TryFrom<&str> for Weapon {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "F" => Ok(Weapon::Foil),
            "E" => Ok(Weapon::Epee),
            "S" => Ok(Weapon::Sabre),
            _ => Err(ParseError::InvalidValue {
                field: "weapon",
                value: value.to_string(),
            }),
        }
    }
}

impl Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Weapon::Foil => write!(f, "F"),
            Weapon::Epee => write!(f, "E"),
            Weapon::Sabre => write!(f, "S"),
        }
    }
}

impl TryFrom<&str> for Priority {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "N" => Ok(Priority::None),
            "R" => Ok(Priority::Right),
            "L" => Ok(Priority::Left),
            _ => Err(ParseError::InvalidValue {
                field: "priority",
                value: value.to_string(),
            }),
        }
    }
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::None => write!(f, "N"),
            Priority::Right => write!(f, "R"),
            Priority::Left => write!(f, "L"),
        }
    }
}

impl TryFrom<&str> for ApparatusState {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "F" => Ok(ApparatusState::Fencing),
            "H" => Ok(ApparatusState::Halt),
            "P" => Ok(ApparatusState::Pause),
            "W" => Ok(ApparatusState::Waiting),
            "E" => Ok(ApparatusState::Ending),
            _ => Err(ParseError::InvalidValue {
                field: "state",
                value: value.to_string(),
            }),
        }
    }
}

impl Display for ApparatusState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApparatusState::Fencing => write!(f, "F"),
            ApparatusState::Halt => write!(f, "H"),
            ApparatusState::Pause => write!(f, "P"),
            ApparatusState::Waiting => write!(f, "W"),
            ApparatusState::Ending => write!(f, "E"),
        }
    }
}

impl TryFrom<&str> for FencerStatus {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "U" => Ok(FencerStatus::Undefined),
            "V" => Ok(FencerStatus::Victory),
            "D" => Ok(FencerStatus::Defeat),
            "A" => Ok(FencerStatus::Abandonment),
            "E" => Ok(FencerStatus::Exclusion),
            _ => Err(ParseError::InvalidValue {
                field: "fencer_status",
                value: value.to_string(),
            }),
        }
    }
}

impl Display for FencerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FencerStatus::Undefined => write!(f, "U"),
            FencerStatus::Victory => write!(f, "V"),
            FencerStatus::Defeat => write!(f, "D"),
            FencerStatus::Abandonment => write!(f, "A"),
            FencerStatus::Exclusion => write!(f, "E"),
        }
    }
}

impl TryFrom<&str> for Reserve {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "N" => Ok(Reserve::None),
            "R" => Ok(Reserve::Introduce),
            _ => Err(ParseError::InvalidValue {
                field: "reserve",
                value: value.to_string(),
            }),
        }
    }
}

impl Display for Reserve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reserve::None => write!(f, "N"),
            Reserve::Introduce => write!(f, "R"),
        }
    }
}

impl TryFrom<&str> for PCard {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "0" => Ok(PCard::None),
            "1" => Ok(PCard::Yellow),
            "2" => Ok(PCard::OneRed),
            "3" => Ok(PCard::TwoRed),
            "4" => Ok(PCard::OneBlack),
            "5" => Ok(PCard::TwoBlack),
            _ => Err(ParseError::InvalidValue {
                field: "p_card",
                value: value.to_string(),
            }),
        }
    }
}

impl Display for PCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PCard::None => write!(f, "0"),
            PCard::Yellow => write!(f, "1"),
            PCard::OneRed => write!(f, "2"),
            PCard::TwoRed => write!(f, "3"),
            PCard::OneBlack => write!(f, "4"),
            PCard::TwoBlack => write!(f, "5"),
        }
    }
}
