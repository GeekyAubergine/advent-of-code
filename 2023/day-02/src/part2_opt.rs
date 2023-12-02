use crate::{error::Error, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Hand {
    Red { consumed: u8, count: u8 },
    Green { consumed: u8, count: u8 },
    Blue { consumed: u8, count: u8 },
}

#[tracing::instrument]
fn parse_hand_color(input: &str) -> Result<Hand> {
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
        "r" => Ok(Hand::Red {
            consumed: color_start as u8 + 3,
            count,
        }),
        "g" => Ok(Hand::Green {
            consumed: color_start as u8 + 5,
            count,
        }),
        "b" => Ok(Hand::Blue {
            consumed: color_start as u8 + 4,
            count,
        }),
        _ => return Err(Error::UnknownColor(color.to_string())),
    }
}

#[tracing::instrument]
fn parse_game(input: &str) -> Result<u32> {
    let input = input.trim();
        
    let mut hands_start = 0;

    for c in input[0..10].chars() {
        if c.eq(&':') {
            break;
        } else {
            hands_start += 1;
        }
    }

    hands_start += 1;

    let mut index = hands_start;

    let mut max_red: u32 = 0;
    let mut max_green: u32 = 0;
    let mut max_blue: u32 = 0;

    while index < input.len() {
        let hand = &input[index..];

        let hand_result = parse_hand_color(hand.trim())?;

        match hand_result {
            Hand::Red { consumed, count } => {
                max_red = max_red.max(count as u32);
                index += consumed as usize;
            }
            Hand::Green { consumed, count } => {
                max_green = max_green.max(count as u32);
                index += consumed as usize;
            }
            Hand::Blue { consumed, count } => {
                max_blue = max_blue.max(count as u32);
                index += consumed as usize;
            }
        }

        index += 2;
    }

    Ok(max_red * max_green * max_blue)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let power_sets = input.lines().map(parse_game).collect::<Result<Vec<_>>>()?;

    Ok(power_sets.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_hand() -> miette::Result<()> {
        let input = "3 blue";

        let hand = parse_hand_color(input)?;

        assert_eq!(
            Hand::Blue {
                consumed: 6,
                count: 3
            },
            hand
        );

        Ok(())
    }

    #[test]
    fn it_should_calculate_power_set() -> miette::Result<()> {
        assert_eq!(
            48,
            parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")?
        );
        assert_eq!(
            12,
            parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")?
        );
        assert_eq!(
            1560,
            parse_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")?
        );

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
