use crate::{error::Error, prelude::*};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    #[tracing::instrument]
    fn new(start: i64, end: i64) -> Range {
        Range { start, end }
    }

    #[tracing::instrument]
    fn is_empty(&self) -> bool {
        self.start == self.end
    }

    #[tracing::instrument]
    fn contains(&self, value: i64) -> bool {
        value >= self.start && value < self.end
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    lines: Vec<String>,
    cursor: usize,
}

impl Input {
    #[tracing::instrument]
    fn from_str(input: &str) -> Result<Input> {
        let lines = input
            .lines()
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();

        Ok(Input { lines, cursor: 0 })
    }

    #[tracing::instrument]
    fn peak(&self) -> Option<&String> {
        self.lines.get(self.cursor)
    }

    #[tracing::instrument]
    fn next(&mut self) -> Result<&String> {
        let next = self
            .lines
            .get(self.cursor)
            .ok_or_else(|| Error::CannotFindNextLine(self.cursor));
        self.cursor += 1;
        next
    }

    #[tracing::instrument]
    fn to_string(&self) -> String {
        self.lines.join("\n")
    }
}

type ParserOutput<T> = (T, Input);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Seeds {
    seeds: Vec<Range>,
}

impl Seeds {
    #[tracing::instrument]
    fn from_input(mut input: Input) -> Result<ParserOutput<Seeds>> {
        let first_line = input.next().map_err(|_| Error::CannotFindSeedsHeader)?;

        if !first_line.starts_with("seeds:") {
            return Err(Error::CannotFindSeedsHeader);
        }

        let seed_pairs = first_line
            .split(':')
            .last()
            .ok_or_else(|| Error::CannotFindSeedsHeader)?
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<i64>().map_err(Error::CouldNotParseNumber))
            .collect::<Result<Vec<_>>>()?;

        let mut seeds = Vec::new();

        for seed_pair in seed_pairs.chunks(2) {
            let seed = seed_pair[0];
            let count = seed_pair[1];

            let seed_range = Range::new(seed, seed + count);

            seeds.push(seed_range);
        }

        Ok((Seeds { seeds }, input))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MapRange {
    source_range: Range,
    mapping_offset: i64,
}

impl MapRange {
    #[tracing::instrument]
    fn new(destination_start: i64, source_start: i64, range: i64) -> MapRange {
        MapRange {
            source_range: Range::new(source_start, source_start + range),
            mapping_offset: destination_start - source_start,
        }
    }

    #[tracing::instrument]
    fn apply_to_value(&self, value: i64) -> i64 {
        if self.source_range.contains(value) {
            // println!("{} -> {}", value, value + self.mapping_offset);
            value + self.mapping_offset
        } else {
            value
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    mapped_ranges: Vec<MapRange>,
}

impl Map {
    #[tracing::instrument]
    fn new(mapped_ranges: Vec<MapRange>) -> Map {
        Map { mapped_ranges }
    }

    #[tracing::instrument]
    fn apply_to_value(&self, value: i64) -> i64 {
        self.mapped_ranges
            .iter()
            .find(|map_range| map_range.source_range.contains(value))
            .map(|map_range| map_range.apply_to_value(value))
            .unwrap_or(value)
    }
}

#[tracing::instrument]
fn maps_from_input(mut input: Input) -> Result<Vec<Map>> {
    let mut maps = Vec::new();
    let mut mapped_ranges = Vec::new();
    while let Ok(line) = input.next() {
        if line.is_empty() {
            continue;
        }

        if line.ends_with("map:") {
            maps.push(Map::new(mapped_ranges));
            mapped_ranges = Vec::new();
            continue;
        }

        let numbers = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<i64>().map_err(Error::CouldNotParseNumber))
            .collect::<Result<Vec<_>>>()?;

        if numbers.len() != 3 {
            return Err(Error::UnexpectedNumberOfValuesForMap(line.to_string()));
        }

        let destination_start = numbers[0];
        let source_start = numbers[1];
        let range = numbers[2];

        let map_range = MapRange::new(destination_start, source_start, range);

        // println!("{} -> map_range: {:?}", line, map_range);

        mapped_ranges.push(map_range);
    }

    maps.push(Map::new(mapped_ranges));

    Ok(maps)
}

#[tracing::instrument]
fn process_seed(seed: i64, maps: &[Map]) -> i64 {
    // println!("seed: {}", seed);
    maps.iter().fold(seed, |acc, map| map.apply_to_value(acc))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i64> {
    let input = Input::from_str(input)?;

    let (seeds, input) = Seeds::from_input(input)?;

    let maps = maps_from_input(input)?;

    let min = seeds
        .seeds
        .iter()
        .map(|seed| process_seed(seed.start, &maps))
        .min()
        .unwrap();

    Ok(min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_map() -> miette::Result<()> {
        let input = Input::from_str(
            "seed-to-soil map:
        50 98 2
        52 50 48",
        )?;

        let maps = maps_from_input(input)?;

        assert_eq!(process_seed(0, &maps), 0);
        assert_eq!(process_seed(1, &maps), 1);

        assert_eq!(process_seed(48, &maps), 48);
        assert_eq!(process_seed(49, &maps), 49);
        assert_eq!(process_seed(50, &maps), 52);
        assert_eq!(process_seed(51, &maps), 53);

        assert_eq!(process_seed(96, &maps), 98);
        assert_eq!(process_seed(97, &maps), 99);
        assert_eq!(process_seed(98, &maps), 50);
        assert_eq!(process_seed(99, &maps), 51);

        assert_eq!(process_seed(79, &maps), 81);
        assert_eq!(process_seed(14, &maps), 14);
        assert_eq!(process_seed(55, &maps), 57);
        assert_eq!(process_seed(13, &maps), 13);

        Ok(())
    }

    #[test]
    fn it_should_process_seed() -> miette::Result<()> {
        let input = include_str!("../example1.txt");
        let input = Input::from_str(input)?;

        let (_, input) = Seeds::from_input(input)?;

        let maps = maps_from_input(input)?;

        assert_eq!(process_seed(79, &maps), 82);
        assert_eq!(process_seed(14, &maps), 43);
        assert_eq!(process_seed(55, &maps), 86);
        assert_eq!(process_seed(13, &maps), 35);

        // assert_eq!(35, process(input)?);
        Ok(())
    }

    #[test]
    fn it_should_be_correct_for_real_data() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!(process(input)?, 486613012);
        Ok(())
    }
}
