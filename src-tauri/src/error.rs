#[derive(Debug, thiserror::Error)]
pub enum CadenceError {
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("database lock was poisoned")]
    LockPoisoned,
    #[error("local midnight was ambiguous due to a timezone transition")]
    AmbiguousLocalTime,
}
