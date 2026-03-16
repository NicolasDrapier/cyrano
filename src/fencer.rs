use super::enums::{FencerStatus, PCard, Reserve};
use super::parser::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Fencer {
    pub id: Option<String>,
    pub name: Option<String>,
    pub nation: Option<String>,
    pub score: Option<u8>,
    pub status: Option<FencerStatus>,
    pub yellow_card: Option<u8>,
    pub red_card: Option<u8>,
    pub light: Option<bool>,
    pub white_light: Option<bool>,
    pub medical: Option<u8>,
    pub reserve: Option<Reserve>,
    pub p_card: Option<PCard>,
}

impl Fencer {
    pub fn parse(fields: &[&str]) -> Self {
        Fencer {
            id: get_field(fields, FENCER_ID).map(String::from),
            name: get_field(fields, FENCER_NAME).map(String::from),
            nation: get_field(fields, FENCER_NATION).map(String::from),
            score: parse_optional(fields, FENCER_SCORE),
            status: get_field(fields, FENCER_STATUS).and_then(|s| FencerStatus::try_from(s).ok()),
            yellow_card: parse_optional(fields, FENCER_YCARD),
            red_card: parse_optional(fields, FENCER_RCARD),
            light: parse_optional_bool(fields, FENCER_LIGHT),
            white_light: parse_optional_bool(fields, FENCER_WLIGHT),
            medical: parse_optional(fields, FENCER_MEDICAL),
            reserve: get_field(fields, FENCER_RESERVE).and_then(|s| Reserve::try_from(s).ok()),
            p_card: get_field(fields, FENCER_PCARD).and_then(|s| PCard::try_from(s).ok()),
        }
    }

    pub fn serialize(&self) -> String {
        let fields: Vec<String> = vec![
            self.id.clone().unwrap_or_default(),
            self.name.clone().unwrap_or_default(),
            self.nation.clone().unwrap_or_default(),
            self.score.map(|v| v.to_string()).unwrap_or_default(),
            self.status.map(|v| v.to_string()).unwrap_or_default(),
            self.yellow_card.map(|v| v.to_string()).unwrap_or_default(),
            self.red_card.map(|v| v.to_string()).unwrap_or_default(),
            self.light.map(|v| if v { "1" } else { "0" }.to_string()).unwrap_or_default(),
            self.white_light.map(|v| if v { "1" } else { "0" }.to_string()).unwrap_or_default(),
            self.medical.map(|v| v.to_string()).unwrap_or_default(),
            self.reserve.map(|v| v.to_string()).unwrap_or_default(),
            self.p_card.map(|v| v.to_string()).unwrap_or_default(),
        ];

        // Trim trailing empty fields
        let last = fields.iter().rposition(|s| !s.is_empty());
        match last {
            Some(idx) => fields[..=idx].join("|"),
            None => String::new(),
        }
    }
}
