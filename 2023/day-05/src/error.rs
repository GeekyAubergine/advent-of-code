
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
    #[error("Could not parse number {0}")]
    CouldNotParseNumber(#[from] std::num::ParseIntError),
    #[error("Next line no available, line {0}")]
    CannotFindNextLine(usize),
    #[error("Could not find seeds header")]
    CannotFindSeedsHeader,
    #[error("Cannot find map hearder")]
    CannotFindMapHeader,
    #[error("Unexpected number of values for map {0}")]
    UnexpectedNumberOfValuesForMap(String),
    #[error("No min value")]
    NoMinValue
}