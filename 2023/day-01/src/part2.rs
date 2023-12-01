use crate::{error::Error, prelude::*};

#[tracing::instrument]
fn parse_digit(input: &str) -> Result<u64> {
    let first_char = input
        .chars()
        .next()
        .ok_or_else(|| Error::NoFirstDigitInLine)?;

    if let Some(digit) = first_char.to_digit(10) {
        return Ok(digit as u64);
    }

    if input.starts_with("zero") {
        return Ok(0);
    }

    if input.starts_with("one") {
        return Ok(1);
    }

    if input.starts_with("two") {
        return Ok(2);
    }

    if input.starts_with("three") {
        return Ok(3);
    }

    if input.starts_with("four") {
        return Ok(4);
    }

    if input.starts_with("five") {
        return Ok(5);
    }

    if input.starts_with("six") {
        return Ok(6);
    }

    if input.starts_with("seven") {
        return Ok(7);
    }

    if input.starts_with("eight") {
        return Ok(8);
    }

    if input.starts_with("nine") {
        return Ok(9);
    }

    Err(Error::ParseBasicIntError())
}

#[tracing::instrument]
fn extract_digits(input: &str) -> Result<Vec<u64>> {
    let mut digits = Vec::new();
    for i in 0..input.len() {
        match parse_digit(&input[i..]) {
            Ok(d) => digits.push(d),
            Err(_) => continue,
        }
    }

    Ok(digits)
}

#[tracing::instrument]
fn number_for_line(line: &str) -> Result<u64> {
    let digits = extract_digits(line)?;
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
    fn it_should_parse_digit() -> miette::Result<()> {
        assert_eq!(0, parse_digit("zero")?);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!(281, process(input)?);
        Ok(())
    }
}
