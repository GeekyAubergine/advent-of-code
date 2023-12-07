use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    CouldNotParseNumber(#[from] std::num::ParseIntError),
    #[error("Could not parse card {0}")]
    CouldNotParseCard(String),
    #[error("Unexpected number of cards in hand")]
    UnexpectedNumberOfCards,
    #[error("Could not parse hand and bet {0}")]
    CouldNotParseHandAndBet(String),
}