use std::io;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO: {0}")]
    Io(#[from] io::Error),
    #[error("Input file must end in .srt (received {0})")]
    InvalidFile(PathBuf),
    #[error("Invalid timestamp received ({0})")]
    InvalidTimestamp(String),
}
