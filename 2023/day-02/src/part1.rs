use crate::{error::Error, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bag {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand {
    red: u8,
    green: u8,
    blue: u8,
}

impl Hand {
    #[tracing::instrument]
    fn from_str(input: &str) -> Result<Self> {
        let mut hand = Self {
            red: 0,
            green: 0,
            blue: 0,
        };

        for card in input.split(',') {
            let parts = card.trim().split(' ').collect::<Vec<_>>();

            let count = parts
                .first()
                .ok_or_else(|| Error::CouldNotParseColorCount(card.to_string()))?;
            let color = parts
                .last()
                .ok_or_else(|| Error::CouldNotParseColorCount(card.to_string()))?;

            let count = count
                .parse::<u8>()
                .map_err(|_| Error::CouldNotParseCount(count.to_string()))?;

            match *color {
                "red" => hand.red = count,
                "green" => hand.green = count,
                "blue" => hand.blue = count,
                _ => return Err(Error::UnknownColor(color.to_string())),
            }
        }

        Ok(hand)
    }

    #[tracing::instrument]
    fn is_possible(&self, bag: &Bag) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
}

impl Game {
    #[tracing::instrument]
    fn from_str(input: &str) -> Result<Self> {
        let id_and_hands = input.split(':').collect::<Vec<_>>();

        let id = id_and_hands
            .first()
            .ok_or_else(|| Error::CouldNotParseGameId(input.to_string()))?
            .trim()
            .split(' ')
            .nth(1)
            .ok_or_else(|| Error::CouldNotParseGameId(input.to_string()))?
            .parse::<u32>()
            .map_err(|_| Error::CouldNotParseGameId(input.to_string()))?;

        let hands = id_and_hands
            .last()
            .ok_or_else(|| Error::CouldNotParseGameHands(input.to_string()))?;

        let hands = hands
            .split(';')
            .map(Hand::from_str)
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { id, hands })
    }

    #[tracing::instrument]
    fn is_possible(&self, bag: &Bag) -> bool {
        self.hands.iter().all(|hand| hand.is_possible(bag))
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = input
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>>>()?;

    let possible_games = games
        .iter()
        .filter(|game| game.is_possible(&bag))
        .map(|game| game.id)
        .sum();

    Ok(possible_games)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_hand() -> miette::Result<()> {
        let input = "1 red, 2 green, 3 blue";
        let hand = Hand::from_str(input)?;
        assert_eq!(1, hand.red);
        assert_eq!(2, hand.green);
        assert_eq!(3, hand.blue);
        Ok(())
    }

    #[test]
    fn it_should_parse_game() -> miette::Result<()> {
        let input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";

        let game = Game::from_str(input);

        assert!(game.is_ok());

        let game = game?;

        assert_eq!(2, game.id);
        assert_eq!(3, game.hands.len());

        assert_eq!(1, game.hands[0].blue);
        assert_eq!(2, game.hands[0].green);
        assert_eq!(0, game.hands[0].red);

        assert_eq!(4, game.hands[1].blue);
        assert_eq!(3, game.hands[1].green);
        assert_eq!(1, game.hands[1].red);

        assert_eq!(1, game.hands[2].blue);
        assert_eq!(1, game.hands[2].green);
        assert_eq!(0, game.hands[2].red);

        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
