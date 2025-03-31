use std::fmt::Display;

#[derive(Debug)]
pub enum ConcurrentCollectionError {
    Poison,
}

impl std::error::Error for ConcurrentCollectionError {}

impl Display for ConcurrentCollectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConcurrentCollectionError::Poison => write!(f, "Lock is poisoned"),
        }
    }
}

impl Eq for ConcurrentCollectionError {}

impl PartialEq for ConcurrentCollectionError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (
                ConcurrentCollectionError::Poison,
                ConcurrentCollectionError::Poison
            )
        )
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidNumber(String),
}

impl std::error::Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidNumber(s) => write!(f, "Parse error: Invalid number: {s}"),
        }
    }
}

impl Eq for ParseError {}

impl PartialEq for ParseError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (ParseError::InvalidNumber(_), ParseError::InvalidNumber(_))
        )
    }
}
