
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
    #[error("no first digit in line")]
    NoFirstDigitInLine,
    #[error("no last digit in line")]
    NoLastDigitInLine,
    #[error("could not parse int")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("could not parse int")]
    ParseBasicIntError(),
}