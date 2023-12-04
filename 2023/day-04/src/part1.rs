use crate::{error::Error, prelude::*};

#[tracing::instrument]
pub fn score_line(line: &str) -> Result<u32> {
    let numbers = line
        .split(':')
        .last()
        .ok_or_else(|| Error::CannotFindNumbers { line: 0 })?;

    let mut numbers = numbers.split('|');

    let winning_numbers = numbers
        .next()
        .ok_or_else(|| Error::CannotFindWinningNumbers { line: 0 })?
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| {
            n.parse::<u32>()
                .map_err(|_| Error::CouldNotParseNumber(n.to_string()))
        })
        .collect::<Result<Vec<_>>>()?;

    let scratch_numbers = numbers
        .last()
        .ok_or_else(|| Error::CannotFindScratchedNumbers { line: 0 })?
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| {
            n.parse::<u32>()
                .map_err(|_| Error::CouldNotParseNumber(n.to_string()))
        })
        .collect::<Result<Vec<_>>>()?;

    let winning_scratched = winning_numbers
        .iter()
        .filter(|n| scratch_numbers.contains(n))
        .count();

    if winning_scratched == 0 {
        return Ok(0);
    }

    Ok(1 << (winning_scratched - 1))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let x = input
        .lines()
        .map(|line| score_line(line.trim()))
        .collect::<Result<Vec<_>>>()
        .map(|v| v.iter().sum())?;

    Ok(x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_score_line_correctly() -> miette::Result<()> {
        assert_eq!(
            score_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")?,
            8
        );
        assert_eq!(
            score_line("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19")?,
            2
        );
        assert_eq!(
            score_line("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1")?,
            2
        );
        assert_eq!(
            score_line("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83")?,
            1
        );
        assert_eq!(
            score_line("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36")?,
            0
        );
        assert_eq!(
            score_line("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")?,
            0
        );
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(13, process(input)?);
        Ok(())
    }
}
