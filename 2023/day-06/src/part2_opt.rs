use crate::{error::Error, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Race {
    time: u64,
    distance: u64,
}

#[tracing::instrument]
fn number_from_line(input: &str) -> Result<u64> {
    let colon_split: Vec<&str> = input.split(": ").collect();

    colon_split[1]
        .chars()
        .filter(|s| s != &' ')
        .collect::<String>()
        .parse::<u64>()
        .map_err(Error::CouldNotParseNumber)
}

#[tracing::instrument]
fn input_to_race(input: &str) -> Result<Race> {
    let lines: Vec<&str> = input.split('\n').map(|l| l.trim()).collect();

    let time = number_from_line(lines[0])?;
    let distance = number_from_line(lines[1])?;

    Ok(Race { time, distance })
}

#[tracing::instrument]
fn calculate_max_distance_for_time(press_down_time: u64, max_time: u64) -> u64 {
    let time_remaining = max_time - press_down_time;
    time_remaining * press_down_time
}

#[tracing::instrument]
fn find_first_winning_number(race: &Race) -> u64 {
    let mut low = 0;
    let mut high = race.time;

    loop {
        let index = (low + high) / 2;
        let left = index - 1;

        let distance = calculate_max_distance_for_time(index, race.time);
        let left_distance = calculate_max_distance_for_time(left, race.time);

        if distance > race.distance && left_distance <= race.distance {
            return index;
        }

        if distance <= race.distance {
            low = index;
        } else {
            high = index;
        }
    }
}

#[tracing::instrument]
fn find_last_winning_number(race: &Race) -> u64 {
    let mut low = 0;
    let mut high = race.time;

    loop {
        let index = (low + high) / 2;
        let right = index + 1;

        let distance = calculate_max_distance_for_time(index, race.time);
        let right_distance = calculate_max_distance_for_time(right, race.time);

        if distance > race.distance && right_distance <= race.distance {
            return index;
        }

        if distance > race.distance {
            low = index;
        } else {
            high = index;
        }
    }
}

#[tracing::instrument]
fn number_of_ways_to_beat_race(race: &Race) -> u64 {
    find_last_winning_number(race) - find_first_winning_number(race)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64> {
    let race = input_to_race(input)?;

    Ok(number_of_ways_to_beat_race(&race))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_parse_races() -> miette::Result<()> {
        let input = include_str!("../input1.txt");

        let expected = Race {
            time: 41777096,
            distance: 249136211271011,
        };

        assert_eq!(expected, input_to_race(input)?);

        Ok(())
    }

    #[test]
    fn it_should_calculate_max_distance_for_time() -> miette::Result<()> {
        assert_eq!(0, calculate_max_distance_for_time(0, 7));

        assert_eq!(6, calculate_max_distance_for_time(1, 7));

        assert_eq!(10, calculate_max_distance_for_time(2, 7));

        assert_eq!(12, calculate_max_distance_for_time(3, 7));

        assert_eq!(12, calculate_max_distance_for_time(4, 7));

        assert_eq!(10, calculate_max_distance_for_time(5, 7));

        assert_eq!(6, calculate_max_distance_for_time(6, 7));

        assert_eq!(0, calculate_max_distance_for_time(7, 7));

        Ok(())
    }
}
