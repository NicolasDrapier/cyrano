/// Information about the referee officiating a fencing match.
///
/// Contains identifying information about the referee including their ID,
/// name, and national affiliation.
#[derive(Debug, Clone, Default)]
pub struct Referee {
    /// Unique identifier for the referee.
    pub id: Option<String>,
    /// Full name of the referee.
    pub name: Option<String>,
    /// Three-letter country code of the referee's nation (e.g., "FRA", "USA").
    pub nation: Option<String>,
}
