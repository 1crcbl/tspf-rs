use std::fmt::Display;

#[derive(Debug)]
pub enum ParseTspError {
    IoError(std::io::Error),
    MissingEntry(String),
    InvalidEntry(String),
    InvalidInput { key: String, val: String },
}

impl From<std::io::Error> for ParseTspError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl Display for ParseTspError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "{}", format!("IO error: {}", e.to_string())),
            Self::MissingEntry(e) => write!(f, "{}", format!("Missing entry: {}", e)),
            Self::InvalidEntry(e) => write!(f, "{}", format!("Invalid entry: {}", e)),
            ParseTspError::InvalidInput { key, val } => {
                write!(f, "{}", format!("Invalid input {} : {}", key, val))
            }
        }
    }
}
