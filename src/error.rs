pub type BngResult<T> = std::result::Result<T, BngError>;

pub enum BngError {
    InvalidReferenceString(String),
}

impl std::fmt::Display for BngError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BngError::InvalidReferenceString(string) => write!(f, "{}", string),
        }
    }
}

impl std::fmt::Debug for BngError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidReferenceString(string) => f.debug_tuple(string).finish(),
        }
    }
}

impl std::error::Error for BngError {}
