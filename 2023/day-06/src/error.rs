
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
    #[error("Could not parse number {0}")]
    CouldNotParseNumber(#[from] std::num::ParseIntError),
    #[error("Missing distance {0}")]
    MissingDistance(usize),
}