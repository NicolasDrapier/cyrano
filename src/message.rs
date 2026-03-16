use std::fmt::Display;

use super::enums::{ApparatusState, Command, CompetitionType, Priority, Protocol, Weapon};
use super::error::ParseError;
use super::fencer::Fencer;
use super::parser::*;
use super::referee::Referee;

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub protocol: Protocol,
    pub command: Command,
    pub piste: Option<String>,
    pub competition_id: Option<String>,
    pub phase: Option<u8>,
    pub pool_tableau: Option<String>,
    pub match_number: Option<u16>,
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

        let raw = raw.trim_matches('|');
        let zones: Vec<&str> = raw.split('%').collect();

        // General zone
        let fields: Vec<&str> = strip_outer_pipes(zones[0]).split('|').collect();

        let protocol = Protocol::try_from(get_required_field(&fields, PROTOCOL, "protocol")?)?;
        let command = Command::try_from(get_required_field(&fields, COMMAND, "command")?)?;

        let piste = get_field(&fields, PISTE).map(String::from);
        let competition_id = get_field(&fields, COMPETITION).map(String::from);
        let phase = parse_optional(&fields, PHASE);
        let pool_tableau = get_field(&fields, POOL_TABLEAU).map(String::from);
        let match_number = parse_optional(&fields, MATCH_NUMBER);
        let round = parse_optional(&fields, ROUND);
        let time = get_field(&fields, TIME).map(String::from);
        let stopwatch = get_field(&fields, STOPWATCH).map(String::from);
        let competition_type =
            get_field(&fields, COMP_TYPE).and_then(|s| CompetitionType::try_from(s).ok());
        let weapon = get_field(&fields, WEAPON).and_then(|s| Weapon::try_from(s).ok());
        let priority = get_field(&fields, PRIORITY).and_then(|s| Priority::try_from(s).ok());
        let state = get_field(&fields, STATE).and_then(|s| ApparatusState::try_from(s).ok());

        let referee = Referee {
            id: get_field(&fields, REF_ID).map(String::from),
            name: get_field(&fields, REF_NAME).map(String::from),
            nation: get_field(&fields, REF_NATION).map(String::from),
        };

        // Right fencer zone
        let right_fencer = if zones.len() > 1 {
            let f: Vec<&str> = strip_outer_pipes(zones[1]).split('|').collect();
            Fencer::parse(&f)
        } else {
            Fencer::default()
        };

        // Left fencer zone
        let left_fencer = if zones.len() > 2 {
            let f: Vec<&str> = strip_outer_pipes(zones[2]).split('|').collect();
            Fencer::parse(&f)
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
        write!(f, "|{}|{}", self.protocol, self.command)?;
        write!(f, "|")?; write_opt(f, &self.piste)?;
        write!(f, "|")?; write_opt(f, &self.competition_id)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    // === Spec examples (pages 6-7) ===

    #[test]
    fn test_spec_hello() {
        let raw = "|EFP1.1|HELLO|17|fm-eq|%|";
        let msg = Message::try_from(raw).unwrap();
        assert_eq!(msg.protocol, Protocol::Efp1_1);
        assert_eq!(msg.command, Command::Hello);
        assert_eq!(msg.piste, Some("17".to_string()));
        assert_eq!(msg.competition_id, Some("fm-eq".to_string()));
    }

    #[test]
    fn test_spec_next() {
        let raw = "|EFP1.1|NEXT|17|fm-eq|%|";
        let msg = Message::try_from(raw).unwrap();
        assert_eq!(msg.command, Command::Next);
    }

    #[test]
    fn test_spec_prev() {
        let raw = "|EFP1.1|PREV|17|fm-eq|%|";
        let msg = Message::try_from(raw).unwrap();
        assert_eq!(msg.command, Command::Prev);
    }

    #[test]
    fn test_spec_full_info() {
        let raw = "|EFP1.1|INFO|17|efj-eq|1|A32|12|2|10:30|3:00|I|S||W|132|J.Smith|GBR|%|28|P.Martin|FRA|8|V|0|1|1|0|0|N|%|32|B. Panini|ITA|6|D|0|1|0|0|0|N|%|";
        let msg = Message::try_from(raw).unwrap();

        assert_eq!(msg.command, Command::Info);
        assert_eq!(msg.piste, Some("17".to_string()));
        assert_eq!(msg.phase, Some(1));
        assert_eq!(msg.pool_tableau, Some("A32".to_string()));
        assert_eq!(msg.match_number, Some(12));
        assert_eq!(msg.round, Some(2));
        assert_eq!(msg.time, Some("10:30".to_string()));
        assert_eq!(msg.stopwatch, Some("3:00".to_string()));
        assert_eq!(msg.competition_type, Some(CompetitionType::Individual));
        assert_eq!(msg.weapon, Some(Weapon::Sabre));
        assert_eq!(msg.priority, None);
        assert_eq!(msg.state, Some(ApparatusState::Waiting));
        assert_eq!(msg.referee.id, Some("132".to_string()));
        assert_eq!(msg.referee.name, Some("J.Smith".to_string()));
        assert_eq!(msg.referee.nation, Some("GBR".to_string()));

        assert_eq!(msg.right_fencer.id, Some("28".to_string()));
        assert_eq!(msg.right_fencer.name, Some("P.Martin".to_string()));
        assert_eq!(msg.right_fencer.nation, Some("FRA".to_string()));
        assert_eq!(msg.right_fencer.score, Some(8));
        assert_eq!(msg.right_fencer.status, Some(FencerStatus::Victory));
        assert_eq!(msg.right_fencer.yellow_card, Some(0));
        assert_eq!(msg.right_fencer.red_card, Some(1));
        assert_eq!(msg.right_fencer.light, Some(true));
        assert_eq!(msg.right_fencer.white_light, Some(false));
        assert_eq!(msg.right_fencer.medical, Some(0));
        assert_eq!(msg.right_fencer.reserve, Some(Reserve::None));

        assert_eq!(msg.left_fencer.id, Some("32".to_string()));
        assert_eq!(msg.left_fencer.name, Some("B. Panini".to_string()));
        assert_eq!(msg.left_fencer.nation, Some("ITA".to_string()));
        assert_eq!(msg.left_fencer.score, Some(6));
        assert_eq!(msg.left_fencer.status, Some(FencerStatus::Defeat));
    }

    #[test]
    fn test_spec_incomplete_info_no_disp() {
        let raw = "|EFP1.1|INFO||||||||3:00||||W|%||||0|U|0|1|1|0|0|N|%||||0|U|0|1|0|0|0|N|%|";
        let msg = Message::try_from(raw).unwrap();

        assert_eq!(msg.piste, None);
        assert_eq!(msg.competition_id, None);
        assert_eq!(msg.stopwatch, Some("3:00".to_string()));
        assert_eq!(msg.state, Some(ApparatusState::Waiting));
        assert_eq!(msg.right_fencer.score, Some(0));
        assert_eq!(msg.right_fencer.status, Some(FencerStatus::Undefined));
        assert_eq!(msg.left_fencer.score, Some(0));
        assert_eq!(msg.left_fencer.status, Some(FencerStatus::Undefined));
    }

    #[test]
    fn test_spec_incomplete_info_with_data() {
        let raw = "|EFP1.1|INFO|17|efj-eq||||||3:00||||W|%||||8|V|0|1|1|0|0|N|%||||6|D|0|1|0|0|0|N|%|";
        let msg = Message::try_from(raw).unwrap();

        assert_eq!(msg.piste, Some("17".to_string()));
        assert_eq!(msg.stopwatch, Some("3:00".to_string()));
        assert_eq!(msg.state, Some(ApparatusState::Waiting));
        assert_eq!(msg.right_fencer.score, Some(8));
        assert_eq!(msg.right_fencer.status, Some(FencerStatus::Victory));
        assert_eq!(msg.left_fencer.score, Some(6));
        assert_eq!(msg.left_fencer.status, Some(FencerStatus::Defeat));
    }

    #[test]
    fn test_spec_waiting_no_match() {
        let raw = "|EFP1.1|INFO||||||||||||W||%|";
        let msg = Message::try_from(raw).unwrap();
        assert_eq!(msg.command, Command::Info);
        assert_eq!(msg.piste, None);
        assert_eq!(msg.state, Some(ApparatusState::Waiting));
    }

    // === Roundtrip tests ===

    #[test]
    fn test_roundtrip_hello() {
        let raw = "|EFP1.1|HELLO|17|fm-eq|%|";
        let msg = Message::try_from(raw).unwrap();
        let reparsed = Message::try_from(msg.to_string().as_str()).unwrap();
        assert_eq!(msg, reparsed);
    }

    #[test]
    fn test_roundtrip_full_info() {
        let raw = "|EFP1.1|INFO|17|efj-eq|1|A32|12|2|10:30|3:00|I|S||W|132|J.Smith|GBR|%|28|P.Martin|FRA|8|V|0|1|1|0|0|N|%|32|B. Panini|ITA|6|D|0|1|0|0|0|N|%|";
        let msg = Message::try_from(raw).unwrap();
        let reparsed = Message::try_from(msg.to_string().as_str()).unwrap();
        assert_eq!(msg, reparsed);
    }

    #[test]
    fn test_roundtrip_sparse_fencer() {
        let raw = "|EFP1.1|INFO|17|fm-eq|%||||||||1|||%|%|";
        let msg = Message::try_from(raw).unwrap();
        assert_eq!(msg.right_fencer.light, Some(true));
        assert_eq!(msg.right_fencer.id, None);
        let reparsed = Message::try_from(msg.to_string().as_str()).unwrap();
        assert_eq!(msg, reparsed);
    }

    // === Error handling ===

    #[test]
    fn test_empty_message() {
        assert!(matches!(Message::try_from(""), Err(ParseError::EmptyMessage)));
    }

    #[test]
    fn test_invalid_command() {
        let result = Message::try_from("|EFP1.1|INVALID|17|fm-eq|%|");
        assert!(matches!(result, Err(ParseError::InvalidCommand(_))));
    }

    #[test]
    fn test_invalid_protocol() {
        let result = Message::try_from("|EFP2.0|HELLO|17|fm-eq|%|");
        assert!(matches!(result, Err(ParseError::InvalidProtocol(_))));
    }

    #[test]
    fn test_command_is_case_sensitive() {
        let result = Message::try_from("|EFP1.1|hello|17|fm-eq|%|");
        assert!(matches!(result, Err(ParseError::InvalidCommand(_))));
    }

    // === Edge cases ===

    #[test]
    fn test_match_number_above_255() {
        let raw = "|EFP1.1|INFO|17|fm-eq||A32|999|||||||||||||%|";
        let msg = Message::try_from(raw).unwrap();
        assert_eq!(msg.match_number, Some(999));
    }

    #[test]
    fn test_piste_as_text() {
        let raw = "|EFP1.1|HELLO|podium|fm-eq|%|";
        let msg = Message::try_from(raw).unwrap();
        assert_eq!(msg.piste, Some("podium".to_string()));
    }

    #[test]
    fn test_stopwatch_with_centiseconds() {
        let raw = "|EFP1.1|INFO|17|fm-eq||||||1:09.25||||||||%|";
        let msg = Message::try_from(raw).unwrap();
        assert_eq!(msg.stopwatch, Some("1:09.25".to_string()));
    }

    #[test]
    fn test_protocol_efp1() {
        let raw = "|EFP1|HELLO|17|fm-eq|%|";
        let msg = Message::try_from(raw).unwrap();
        assert_eq!(msg.protocol, Protocol::Efp1);
    }

    // === Fencer serialization ===

    #[test]
    fn test_fencer_serialize_all_empty() {
        let fencer = Fencer::default();
        assert_eq!(fencer.serialize(), "");
    }

    #[test]
    fn test_fencer_serialize_only_late_field() {
        let fencer = Fencer {
            light: Some(true),
            ..Default::default()
        };
        assert_eq!(fencer.serialize(), "|||||||1");
    }

    #[test]
    fn test_fencer_serialize_full() {
        let fencer = Fencer {
            id: Some("28".to_string()),
            name: Some("P.Martin".to_string()),
            nation: Some("FRA".to_string()),
            score: Some(8),
            status: Some(FencerStatus::Victory),
            yellow_card: Some(0),
            red_card: Some(1),
            light: Some(true),
            white_light: Some(false),
            medical: Some(0),
            reserve: Some(Reserve::None),
            p_card: None,
        };
        assert_eq!(fencer.serialize(), "28|P.Martin|FRA|8|V|0|1|1|0|0|N");
    }
}
