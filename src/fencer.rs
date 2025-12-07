use std::convert::TryFrom;

use super::enums::{FencerStatus, PCard, Reserve};
use super::error::ParseError;
use super::parser::{get_field, parse_optional_bool, parse_optional_u8};

#[derive(Debug, Clone, Default)]
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

        // Supprime les champs vides à la fin
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
