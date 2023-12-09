use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Could not parse number {0}")]
    CouldNotParseNumber(#[from] std::num::ParseIntError),
    #[error("Could not get bottom row of values")]
    CouldNotGetBottomRowOfValues,
    #[error("Could not get last value of row {0}")]
    CouldNotGetLastValueOfRow(usize),
    #[error("Could not get first value of row {0}")]
    CouldNotGetFirstValueOfRow(usize),
}