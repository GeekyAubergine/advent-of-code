use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Could not parse number {0}")]
    CouldNotParseNumber(#[from] std::num::ParseIntError),
}