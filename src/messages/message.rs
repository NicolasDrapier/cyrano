use std::convert::TryFrom;
use std::fmt::Display;

use super::enums::{ApparatusState, Command, CompetitionType, Priority, Protocol, Weapon};
use super::error::ParseError;
use super::fencer::Fencer;
use super::parser::{get_field, get_required_field, parse_optional_u8, strip_outer_pipes};
use super::referee::Referee;

#[derive(Debug, Clone)]
pub struct Message {
    pub protocol: Protocol,
    pub command: Command,
    pub piste: String,
    pub competition_id: String,
    pub phase: Option<u8>,
    pub pool_tableau: Option<String>,
    pub match_number: Option<u8>,
    pub round: Option<u8>,
    pub time: Option<String>,
    pub stopwatch: Option<String>,
    pub competition_type: Option<CompetitionType>,
    pub weapon: Option<Weapon>,
    pub priority: Option<Priority>,
    pub state: Option<ApparatusState>,
    pub referee: Referee,
    pub right_fencer: Fencer,
    pub left_fencer: Fencer,
}

impl TryFrom<&str> for Message {
    type Error = ParseError;

    fn try_from(raw: &str) -> Result<Self, Self::Error> {
        let raw = raw.trim();

        if raw.is_empty() {
            return Err(ParseError::EmptyMessage);
        }

        // Enlève les | au début et à la fin
        let raw = raw.trim_matches('|');

        // Sépare les 3 zones par %
        let zones: Vec<&str> = raw.split('%').collect();

        // Zone générale
        let general_fields: Vec<&str> = strip_outer_pipes(zones[0]).split('|').collect();

        // Champs obligatoires (**)
        let protocol =
            Protocol::try_from(get_required_field(&general_fields, 0, "protocol")?)?;

        let command = Command::try_from(get_required_field(&general_fields, 1, "command")?)?;
        let piste = get_field(&general_fields, 2).map(String::from).unwrap_or_default();
        let competition_id = get_field(&general_fields, 3).map(String::from).unwrap_or_default();

        // Champs optionnels de la zone générale
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

        // Zone tireur droit
        let right_fencer = if zones.len() > 1 {
            let right_fields: Vec<&str> = strip_outer_pipes(zones[1]).split('|').collect();
            Fencer::parse(&right_fields)?
        } else {
            Fencer::default()
        };

        // Zone tireur gauche
        let left_fencer = if zones.len() > 2 {
            let left_fields: Vec<&str> = strip_outer_pipes(zones[2]).split('|').collect();
            Fencer::parse(&left_fields)?
        } else {
            Fencer::default()
        };

        Ok(Message {
            protocol,
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

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        Message::try_from(raw.as_str())
    }
}

fn write_opt<T: Display>(f: &mut std::fmt::Formatter<'_>, val: &Option<T>) -> std::fmt::Result {
    if let Some(v) = val {
        write!(f, "{v}")
    } else {
        Ok(())
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Zone générale
        write!(f, "|{}|{}|{}|{}", self.protocol, self.command, self.piste, self.competition_id)?;

        write!(f, "|")?; write_opt(f, &self.phase)?;
        write!(f, "|")?; write_opt(f, &self.pool_tableau)?;
        write!(f, "|")?; write_opt(f, &self.match_number)?;
        write!(f, "|")?; write_opt(f, &self.round)?;
        write!(f, "|")?; write_opt(f, &self.time)?;
        write!(f, "|")?; write_opt(f, &self.stopwatch)?;
        write!(f, "|")?; write_opt(f, &self.competition_type)?;
        write!(f, "|")?; write_opt(f, &self.weapon)?;
        write!(f, "|")?; write_opt(f, &self.priority)?;
        write!(f, "|")?; write_opt(f, &self.state)?;
        write!(f, "|")?; write_opt(f, &self.referee.id)?;
        write!(f, "|")?; write_opt(f, &self.referee.name)?;
        write!(f, "|")?; write_opt(f, &self.referee.nation)?;

        // Fencer zones
        write!(
            f,
            "|%|{}|%|{}|%|",
            self.right_fencer.serialize(),
            self.left_fencer.serialize()
        )
    }
}

impl From<Message> for String {
    fn from(msg: Message) -> Self {
        msg.to_string()
    }
}

// ===== TESTS =====

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::enums::{ApparatusState, Command, FencerStatus, Weapon};

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
        let raw = "|EFP1.1|INFO|17|fm-eq||||||||||W|||%|";
        let msg = Message::try_from(raw).unwrap();

        assert_eq!(msg.command, Command::Info);
        assert_eq!(msg.piste, "17");
        assert_eq!(msg.competition_id, "fm-eq");
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
    fn test_roundtrip_complete() {
        let raw = "|EFP1.1|INFO|17|efj-eq|1|A32|12|2|10:30|3:00|I|S||W|132|J.Smith|GBR|%|28|P.Martin|FRA|8|V|0|1|1|0|0|N|%|32|B. Panini|ITA|6|D|0|1|0|0|0|N|%|";
        let msg = Message::try_from(raw).unwrap();
        let serialized = msg.to_string();
        let reparsed = Message::try_from(serialized.as_str()).unwrap();

        assert_eq!(msg.command, reparsed.command);
        assert_eq!(msg.piste, reparsed.piste);
        assert_eq!(msg.right_fencer.id, reparsed.right_fencer.id);
        assert_eq!(msg.right_fencer.name, reparsed.right_fencer.name);
        assert_eq!(msg.right_fencer.score, reparsed.right_fencer.score);
        assert_eq!(msg.right_fencer.status, reparsed.right_fencer.status);
        assert_eq!(msg.right_fencer.light, reparsed.right_fencer.light);
        assert_eq!(msg.left_fencer.id, reparsed.left_fencer.id);
        assert_eq!(msg.left_fencer.score, reparsed.left_fencer.score);
    }

    #[test]
    fn test_roundtrip_sparse_fencer() {
        // Fencer with only a late field set (light at index 7) — tests the
        // trim_matches fix: leading empty fields must survive serialization roundtrip
        let raw = "|EFP1.1|INFO|17|fm-eq|%||||||||1|||%|%|";
        let msg = Message::try_from(raw).unwrap();
        assert_eq!(msg.right_fencer.light, Some(true));
        assert_eq!(msg.right_fencer.id, None);

        let serialized = msg.to_string();
        let reparsed = Message::try_from(serialized.as_str()).unwrap();
        assert_eq!(reparsed.right_fencer.light, Some(true));
        assert_eq!(reparsed.right_fencer.id, None);
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
