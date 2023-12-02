
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
    #[error("Could not parse color count from hand {0}")]
    CouldNotParseColorCount(String),
    #[error("Unknown color {0}")]
    UnknownColor(String),
    #[error("Could not parse count {0}")]
    CouldNotParseCount(String),
    #[error("Could not parse game id {0}")]
    CouldNotParseGameId(String),
    #[error("Could not parse game hands {0}")]
    CouldNotParseGameHands(String),

}