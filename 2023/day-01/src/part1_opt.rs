use crate::{error::Error, prelude::*};

#[tracing::instrument]
fn extract_first_digit(input: &str) -> Result<u8> {
    for c in input.chars() {
        if let Some(digit) = c.to_digit(10) {
            return Ok(digit as u8);
        }
    }

    Err(Error::NoFirstDigitInLine)
}

#[tracing::instrument]
fn extract_last_digit(input: &str) -> Result<u8> {
    for c in input.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            return Ok(digit as u8);
        }
    }

    Err(Error::NoLastDigitInLine)
}

#[tracing::instrument]
fn number_for_line(line: &str) -> Result<u32> {
    let first = extract_first_digit(line)?;
    let last = extract_last_digit(line)?;
    Ok((first * 10 + last) as u32)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    Ok(input
        .lines()
        .map(number_for_line)
        .collect::<Result<Vec<u32>>>()
        .map(|v| v.iter().sum())?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(142, process(input)?);
        Ok(())
    }
}
