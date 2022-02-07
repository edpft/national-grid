pub type BngResult<T> = std::result::Result<T, BngError>;

pub enum BngError {
    Badness(),
}

impl std::fmt::Display for BngError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BngError::Badness() => write!(f, "Not so good..."),
        }
    }
}

impl std::fmt::Debug for BngError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Badness() => f.debug_tuple("Badness").finish(),
        }
    }
}

impl std::error::Error for BngError {}
