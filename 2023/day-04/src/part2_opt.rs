use std::collections::HashMap;

use crate::{error::Error, prelude::*};

#[tracing::instrument]
fn parse_numbers(input: &str) -> Result<Vec<u32>> {
    let input = input.trim();

    let mut in_number = false;
    let mut numbers = vec![];
    let mut number_start = 0;

    for (i, c) in input.chars().enumerate() {
        if c.is_ascii_digit() {
            if !in_number {
                in_number = true;
                number_start = i;
            }
        } else if in_number {
            numbers.push(
                input[number_start..i]
                    .parse()
                    .map_err(|_| Error::CouldNotParseNumber(input.to_string()))?,
            );
            in_number = false;
        }
    }

    if in_number {
        numbers.push(
            input[number_start..]
                .parse()
                .map_err(|_| Error::CouldNotParseNumber(input.to_string()))?,
        );
    }

    Ok(numbers)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cards {
    copies: HashMap<u32, u32>,
}

impl Cards {
    #[tracing::instrument]
    fn new() -> Self {
        Self {
            copies: HashMap::new(),
        }
    }

    #[tracing::instrument]
    fn add_card(&mut self, card: u32) {
        *self.copies.entry(card).or_insert(0) += 1;
    }

    #[tracing::instrument]
    fn add_card_copies(&mut self, card: u32, copies: u32) {
        *self.copies.entry(card).or_insert(0) += copies;
    }

    #[tracing::instrument]
    fn get_count(&self, card: u32) -> u32 {
        *self.copies.get(&card).unwrap_or(&0)
    }
}

#[tracing::instrument]
pub fn score_line(line: &str, cards: Cards) -> Result<Cards> {
    let mut cards = cards;

    let mut card_and_numbers = line.split(':');

    let card_number = card_and_numbers
        .next()
        .ok_or_else(|| Error::CannotFindCardNumber(line.to_owned()))?
        .split(' ')
        .last()
        .ok_or_else(|| Error::CannotFindCardNumber(line.to_owned()))?
        .parse::<u32>()
        .map_err(|_| Error::CouldNotParseCardNumber(line.to_owned()))?;

    cards.add_card(card_number);

    let numbers = card_and_numbers
        .last()
        .ok_or(Error::CannotFindNumbers { line: 0 })?;

    let mut numbers = numbers.split('|');

    let winning_numbers = numbers
        .next()
        .ok_or(Error::CannotFindWinningNumbers { line: 0 })?;

    let winning_numbers = parse_numbers(winning_numbers)?;

    let scratch_numbers = numbers
        .last()
        .ok_or(Error::CannotFindScratchedNumbers { line: 0 })?;

    let scratch_numbers = parse_numbers(scratch_numbers)?;

    let winning_scratched = winning_numbers
        .iter()
        .filter(|n| scratch_numbers.contains(n))
        .count();

    let copies = cards.get_count(card_number);

    for i in 1..=winning_scratched {
        cards.add_card_copies(card_number + i as u32, copies);
    }

    Ok(cards)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let mut lines = input.lines();

    let cards = lines.try_fold(Cards::new(), |cards, line| score_line(line.trim(), cards))?;

    let card_count = input
        .lines()
        .enumerate()
        .map(|(i, _line)| cards.get_count(i as u32 + 1))
        .sum::<u32>();

    Ok(card_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(30, process(input)?);
        Ok(())
    }
}
