use crate::{error::Error, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bag {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameResult {
    Possible { game_id: u32 },
    Impossible,
}

enum HandResult {
    Possible { length: usize },
    Impossible,
}

#[tracing::instrument]
fn parse_hand_color(input: &str, bag: &Bag) -> Result<HandResult> {
    let mut count_chars: String = String::new();

    for c in input[0..5].chars() {
        if c.is_ascii_digit() {
            count_chars.push(c);
        } else {
            break;
        }
    }
    let color_start = count_chars.len() + 1;

    let count = count_chars
        .parse::<u8>()
        .map_err(|_| Error::CouldNotParseCount(input.to_string()))?;

    let color = input
        .get(color_start..color_start + 1)
        .ok_or_else(|| Error::CouldNotParseColorCount(input.to_string()))?;

    match color {
        "r" => {
            if count > bag.red {
                return Ok(HandResult::Impossible);
            } else {
                return Ok(HandResult::Possible {
                    length: color_start + 3,
                });
            }
        }
        "g" => {
            if count > bag.green {
                return Ok(HandResult::Impossible);
            } else {
                return Ok(HandResult::Possible {
                    length: color_start + 5,
                });
            }
        }
        "b" => {
            if count > bag.blue {
                return Ok(HandResult::Impossible);
            } else {
                return Ok(HandResult::Possible {
                    length: color_start + 4,
                });
            }
        }
        _ => return Err(Error::UnknownColor(color.to_string())),
    }
}

#[tracing::instrument]
fn parse_game(input: &str, bag: &Bag) -> Result<GameResult> {
    let mut id_chars: String = String::new();

    for c in input[5..10].chars() {
        if c.is_ascii_digit() {
            id_chars.push(c);
        } else {
            break;
        }
    }

    let hands_start = 5 + id_chars.len() + 2;

    let game_id = id_chars
        .parse::<u32>()
        .map_err(|_| Error::CouldNotParseGameId(id_chars))?;

    let mut index = hands_start;

    while index < input.len() {
        let hand = &input[index..];

        let hand_result = parse_hand_color(hand, bag)?;

        match hand_result {
            HandResult::Possible { length } => {
                index += length + 2;
            }
            HandResult::Impossible { .. } => {
                return Ok(GameResult::Impossible);
            }
        }
    }

    Ok(GameResult::Possible { game_id })
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut possible_game_ids = vec![];

    for line in input.lines() {
        let game_result = parse_game(line.trim(), &bag)?;
        match game_result {
            GameResult::Possible { game_id } => {
                possible_game_ids.push(game_id);
            }
            GameResult::Impossible => {}
        }
    }

    Ok(possible_game_ids.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_possibe_game() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let game = parse_game(
            input,
            &Bag {
                red: 12,
                green: 13,
                blue: 14,
            },
        );

        let game = game?;

        assert_eq!(GameResult::Possible { game_id: 1 }, game);

        Ok(())
    }

    #[test]
    fn it_should_parse_impossible_game() -> miette::Result<()> {
        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";

        let game = parse_game(
            input,
            &Bag {
                red: 12,
                green: 13,
                blue: 14,
            },
        );

        assert!(game.is_ok());

        let game = game?;

        assert_eq!(GameResult::Impossible, game);

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
