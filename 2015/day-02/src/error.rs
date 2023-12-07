
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
    #[error("Could not parse {0} as a number")]
    CouldNotParseNumber(#[from] std::num::ParseIntError),
    #[error("Expectd number")]
    ExpectedNumber,
}