use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Could not parse number {0}")]
    CouldNotParseNumber(#[from] std::num::ParseIntError),
    #[error("Unknown pipe: {0}")]
    UnknownPipe(char),
    #[error("No start found")]
    NoStart,
    #[error("No current position found")]
    NoCurrentPosition,
    #[error("Could not find pipe for position {0} {1}")]
    CouldNotFindPipeForPosition(i32, i32),
    #[error("Pipe has not valid exit: {0}")]
    PipeHasNotValidExit(char),
    #[error("Could not enter next pipe: {0}")]
    CouldNotEnterNextPipe(char),
    #[error("Invalid start. No connection found")]
    InvalidStart,
}