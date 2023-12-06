use rayon::vec;

use crate::{error::Error, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Race {
    time: u64,
    distance: u64,
}

#[tracing::instrument]
fn numbers_from_line(input: &str) -> Result<Vec<u64>> {
    let colon_split: Vec<&str> = input.split(": ").collect();

    colon_split[1]
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().map_err(Error::CouldNotParseNumber))
        .collect()
}

#[tracing::instrument]
fn input_to_races(input: &str) -> Result<Vec<Race>> {
    let mut races = vec![];

    let lines: Vec<&str> = input.split('\n').map(|l| l.trim()).collect();

    let times = numbers_from_line(lines[0])?;
    let distances = numbers_from_line(lines[1])?;

    for (i, time) in times.iter().enumerate() {
        match distances.get(i) {
            Some(distance) => races.push(Race {
                time: *time,
                distance: *distance,
            }),
            None => return Err(Error::MissingDistance(i)),
        }
    }

    Ok(races)
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
    let races = input_to_races(input)?;

    Ok(races.iter().map(number_of_ways_to_beat_race).product())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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

    #[test]
    fn it_should_find_first_winner() -> miette::Result<()> {
        assert_eq!(2, find_first_winning_number(&Race {
            time: 7,
            distance: 9,
        }));

        assert_eq!(4, find_first_winning_number(&Race {
            time: 15,
            distance: 40,
        }));

        assert_eq!(11, find_first_winning_number(&Race {
            time: 30,
            distance: 200,
        }));

        Ok(())
    }

    #[test]
    fn it_should_find_last_winner() -> miette::Result<()> {     
        assert_eq!(5, find_last_winning_number(&Race {
            time: 7,
            distance: 9,
        }));

        assert_eq!(11, find_last_winning_number(&Race {
            time: 15,
            distance: 40,
        }));

        assert_eq!(19, find_last_winning_number(&Race {
            time: 30,
            distance: 200,
        }));

        Ok(())
    }
}
