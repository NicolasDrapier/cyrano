use super::error::ParseError;

/// Retrieves an optional field from an array of string slices.
///
/// Returns `None` if the field is missing or empty, otherwise returns `Some` with the field value.
///
/// # Arguments
///
/// * `fields` - Array of string slices to search
/// * `index` - Index of the field to retrieve
///
/// # Returns
///
/// `Some(&str)` if the field exists and is non-empty, `None` otherwise.
pub fn get_field<'a>(fields: &'a [&str], index: usize) -> Option<&'a str> {
    fields.get(index).and_then(|s| {
        if s.is_empty() {
            None
        } else {
            Some(*s)
        }
    })
}

/// Retrieves a required field from an array of string slices.
///
/// # Arguments
///
/// * `fields` - Array of string slices to search
/// * `index` - Index of the field to retrieve
/// * `name` - Name of the field for error reporting
///
/// # Returns
///
/// `Ok(&str)` if the field exists and is non-empty.
///
/// # Errors
///
/// Returns `ParseError::MissingField` if the field is missing or empty.
pub fn get_required_field<'a>(
    fields: &'a [&str],
    index: usize,
    name: &'static str,
) -> Result<&'a str, ParseError> {
    get_field(fields, index).ok_or(ParseError::MissingField(name))
}

/// Parses an optional unsigned 8-bit integer from a field.
///
/// # Arguments
///
/// * `fields` - Array of string slices to search
/// * `index` - Index of the field to parse
///
/// # Returns
///
/// `Some(u8)` if the field exists and can be parsed as a `u8`, `None` otherwise.
pub fn parse_optional_u8(fields: &[&str], index: usize) -> Option<u8> {
    get_field(fields, index).and_then(|s| s.parse().ok())
}

/// Parses an optional boolean from a field.
///
/// Interprets "1" as `true` and any other value as `false`.
///
/// # Arguments
///
/// * `fields` - Array of string slices to search
/// * `index` - Index of the field to parse
///
/// # Returns
///
/// `Some(bool)` if the field exists, `None` if the field is missing or empty.
pub fn parse_optional_bool(fields: &[&str], index: usize) -> Option<bool> {
    get_field(fields, index).map(|s| s == "1")
}
