#[derive(Debug)]
pub enum ParseError {
    EmptyMessage,
    MissingField(&'static str),
    InvalidCommand(String),
    InvalidProtocol(String),
    InvalidValue { field: &'static str, value: String },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EmptyMessage => write!(f, "Empty message"),
            ParseError::MissingField(field) => write!(f, "Missing required field: {}", field),
            ParseError::InvalidCommand(cmd) => write!(f, "Invalid command: {}", cmd),
            ParseError::InvalidProtocol(proto) => write!(f, "Invalid protocol: {}", proto),
            ParseError::InvalidValue { field, value } => {
                write!(f, "Invalid value for {}: {}", field, value)
            }
        }
    }
}

impl std::error::Error for ParseError {}
