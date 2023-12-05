
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
    #[error("Could not find seeds header {0}")]
    CannotFindSeedsHeader(String),
    #[error("Could not parse seed {0}")]
    CouldNotParseSeed(String)
}