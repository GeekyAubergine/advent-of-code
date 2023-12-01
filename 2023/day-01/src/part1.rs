use crate::{error::Error, prelude::*};

#[tracing::instrument]
fn extract_digits(input: &str) -> Vec<u64> {
    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u64)
        .collect()
}

#[tracing::instrument]
fn number_for_line(line: &str) -> Result<u64> {
    let digits = extract_digits(line);
    let first = digits.first().ok_or_else(|| Error::NoFirstDigitInLine)?;
    let last = digits.last().ok_or_else(|| Error::NoLastDigitInLine)?;
    let string = format!("{}{}", first, last);
    Ok(string.parse::<u64>()?)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64> {
    Ok(input
        .lines()
        .map(number_for_line)
        .collect::<Result<Vec<u64>>>()
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
