use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    // #[error("data store disconnected")]
    // Disconnect(#[from] std:;io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}
