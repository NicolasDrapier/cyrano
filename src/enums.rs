use std::fmt::Display;

use super::error::ParseError;

macro_rules! efp_enum {
    ($name:ident, $field:expr, { $($wire:expr => $variant:ident),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $($variant),+
        }

        impl TryFrom<&str> for $name {
            type Error = ParseError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value {
                    $($wire => Ok($name::$variant),)+
                    _ => Err(ParseError::InvalidValue {
                        field: $field,
                        value: value.to_string(),
                    }),
                }
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($name::$variant => write!(f, $wire)),+
                }
            }
        }
    };
}

// Protocol and Command have dedicated error variants, so they're hand-written.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protocol {
    Efp1,
    Efp1_1,
}

impl TryFrom<&str> for Protocol {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "EFP1" => Ok(Protocol::Efp1),
            "EFP1.1" => Ok(Protocol::Efp1_1),
            _ => Err(ParseError::InvalidProtocol(value.to_string())),
        }
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Efp1 => write!(f, "EFP1"),
            Protocol::Efp1_1 => write!(f, "EFP1.1"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Command {
    Hello,
    Disp,
    Ack,
    Nak,
    Info,
    Next,
    Prev,
}

impl TryFrom<&str> for Command {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
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

efp_enum!(CompetitionType, "competition_type", {
    "I" => Individual,
    "T" => Team,
});

efp_enum!(Weapon, "weapon", {
    "F" => Foil,
    "E" => Epee,
    "S" => Sabre,
});

efp_enum!(Priority, "priority", {
    "N" => None,
    "R" => Right,
    "L" => Left,
});

efp_enum!(ApparatusState, "state", {
    "F" => Fencing,
    "H" => Halt,
    "P" => Pause,
    "W" => Waiting,
    "E" => Ending,
});

efp_enum!(FencerStatus, "fencer_status", {
    "U" => Undefined,
    "V" => Victory,
    "D" => Defeat,
    "A" => Abandonment,
    "E" => Exclusion,
});

efp_enum!(Reserve, "reserve", {
    "N" => None,
    "R" => Introduce,
});

efp_enum!(PCard, "p_card", {
    "0" => None,
    "1" => Yellow,
    "2" => OneRed,
    "3" => TwoRed,
    "4" => OneBlack,
    "5" => TwoBlack,
});
