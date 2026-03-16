#[derive(Debug)]
pub enum ParseError {
    EmptyMessage,
    InvalidFormat,
    MissingField(&'static str),
    InvalidCommand(String),
    InvalidProtocol(String),
    InvalidValue { field: &'static str, value: String },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EmptyMessage => write!(f, "Message vide"),
            ParseError::InvalidFormat => write!(f, "Format CSV invalide"),
            ParseError::MissingField(field) => write!(f, "Champ obligatoire manquant: {}", field),
            ParseError::InvalidCommand(cmd) => write!(f, "Commande invalide: {}", cmd),
            ParseError::InvalidProtocol(proto) => write!(f, "Protocole invalide: {}", proto),
            ParseError::InvalidValue { field, value } => {
                write!(f, "Valeur invalide pour {}: {}", field, value)
            }
        }
    }
}

impl std::error::Error for ParseError {}
