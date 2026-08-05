// Custom errors for the tool, using thiserror (Day 18).
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScoreError {
    #[error("could not read the score file: {0}")]
    Read(String),

    #[error("could not write the score file: {0}")]
    Write(String),

    #[error("the score file is corrupted: {0}")]
    Corrupt(String),

    #[error("no scores recorded yet")]
    Empty,
}