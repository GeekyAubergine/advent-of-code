
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
    #[error("Cannot find numbers for line {line}")]
    CannotFindNumbers { line: usize },
    #[error("Cannot find winning numbers for line {line}")]
    CannotFindWinningNumbers { line: usize },
    #[error("Cannot find scratched numbers for line {line}")]
    CannotFindScratchedNumbers { line: usize },
    #[error("Could not parse number from {0}")]
    CouldNotParseNumber(String),
    #[error("Could not find card numer {0}")]
    CannotFindCardNumber(String),
    #[error("Could not parse card number {0}")]
    CouldNotParseCardNumber(String)
}