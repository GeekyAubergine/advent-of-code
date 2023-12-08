use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Could not parse number {0}")]
    CouldNotParseNumber(#[from] std::num::ParseIntError),
    #[error("Could not find id for instruction {0}")]
    CouldNotFindIdForInstruction(String),
    #[error("Could not find left instruction {0}")]
    CouldNotFindLeftInstruction(String),
    #[error("Could not find right instruction {0}")]
    CouldNotFindRightInstruction(String),
    #[error("Invalid number of letters for id {0}")]
    InvalidNumberOfLettersForId(String),
    #[error("Could not find instruction for id {0}")]
    CouldNotInspectionForId(String),
    #[error("No instructions found")]
    NoInstructionsFound,
    #[error("Unexpected instruction {0}")]
    UnexpectedInstruction(String),
    #[error("Unexpected end of instructions")]
    UnexpectedEndOfInstructions,
    #[error("Unknown number of min steps")]
    UnknownNumberOfMinSteps,
    #[error("Unknown number of max steps")]
    UnknownNumberOfMaxSteps,
}