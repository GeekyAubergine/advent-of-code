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
    fn min_possible_bag(&self) -> Bag {
        let mut bag = Bag {
            red: 0,
            green: 0,
            blue: 0,
        };

        for hand in &self.hands {
            bag.red = bag.red.max(hand.red);
            bag.green = bag.green.max(hand.green);
            bag.blue = bag.blue.max(hand.blue);
        }

        bag
    }

    #[tracing::instrument]
    fn power_set(&self) -> u32 {
        let bag = self.min_possible_bag();

        bag.red as u32 * bag.green as u32 * bag.blue as u32
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let games = input
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>>>()?;

    let power_sets = games
        .iter()
        .map(|game| game.power_set())
        .collect::<Vec<_>>();

    Ok(power_sets.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_calculate_min_possible_hand() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let game = Game::from_str(input)?;

        assert_eq!(
            Bag {
                red: 4,
                green: 2,
                blue: 6,
            },
            game.min_possible_bag()
        );

        Ok(())
    }

    #[test]
    fn it_should_calculate_power_set() -> miette::Result<()> {
        assert_eq!(48, Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")?.power_set());
        assert_eq!(12, Game::from_str("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")?.power_set());
        assert_eq!(1560, Game::from_str("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")?.power_set());

        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286, process(input)?);
        Ok(())
    }
}
