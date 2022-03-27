pub type BngResult<T> = std::result::Result<T, BngError>;

pub enum BngError {
    InvalidReferenceString(String),
    InvalidEastings(String),
    InvalidNorthings(String),
    NegativeNumber(String),
    InvalidLetters(String),
    InvalidCoordinateRemainder(String),
    Other(String),
}

impl std::fmt::Display for BngError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BngError::InvalidReferenceString(string) => write!(f, "{}", string),
            BngError::InvalidEastings(string) => write!(f, "{}", string),
            BngError::InvalidNorthings(string) => write!(f, "{}", string),
            BngError::NegativeNumber(string) => write!(f, "{}", string),
            BngError::InvalidLetters(string) => write!(f, "{}", string),
            BngError::InvalidCoordinateRemainder(string) => write!(f, "{}", string),
            BngError::Other(string) => write!(f, "{}", string),
        }
    }
}

impl std::fmt::Debug for BngError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidReferenceString(string) => f.debug_tuple(string).finish(),
            Self::InvalidEastings(string) => f.debug_tuple(string).finish(),
            Self::InvalidNorthings(string) => f.debug_tuple(string).finish(),
            Self::NegativeNumber(string) => f.debug_tuple(string).finish(),
            Self::InvalidLetters(string) => f.debug_tuple(string).finish(),
            Self::InvalidCoordinateRemainder(string) => f.debug_tuple(string).finish(),
            Self::Other(string) => f.debug_tuple(string).finish(),
        }
    }
}

impl std::error::Error for BngError {}
