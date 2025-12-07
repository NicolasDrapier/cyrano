use std::convert::TryFrom;

use super::error::ParseError;

/// Récupère une valeur à un index, retourne None si vide ou absent
pub fn get_field<'a>(fields: &'a [&str], index: usize) -> Option<&'a str> {
    fields.get(index).and_then(|s| {
        if s.is_empty() {
            None
        } else {
            Some(*s)
        }
    })
}

/// Parse un champ obligatoire
pub fn get_required_field<'a>(
    fields: &'a [&str],
    index: usize,
    name: &'static str,
) -> Result<&'a str, ParseError> {
    get_field(fields, index).ok_or(ParseError::MissingField(name))
}

/// Parse un u8 optionnel
pub fn parse_optional_u8(fields: &[&str], index: usize) -> Option<u8> {
    get_field(fields, index).and_then(|s| s.parse().ok())
}

/// Parse un bool depuis "0"/"1"
pub fn parse_optional_bool(fields: &[&str], index: usize) -> Option<bool> {
    get_field(fields, index).map(|s| s == "1")
}

/// Parse une enum optionnelle
pub fn parse_optional_enum<T: TryFrom<&'static str, Error = ParseError>>(
    fields: &[&str],
    index: usize,
) -> Option<T> {
    get_field(fields, index).and_then(|s| {
        // Astuce pour contourner les lifetimes
        let static_str: &'static str = Box::leak(s.to_string().into_boxed_str());
        T::try_from(static_str).ok()
    })
}
