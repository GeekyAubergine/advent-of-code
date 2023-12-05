use crate::{error::Error, prelude::*};
use rayon::prelude::*;

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
struct SeedRange {
    start: u64,
    end: u64,
}

impl SeedRange {
    #[tracing::instrument]
    fn new(start: u64, end: u64) -> SeedRange {
        SeedRange { start, end }
    }

    #[tracing::instrument]
    fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Seeds {
    seeds: Vec<SeedRange>,
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
            .map(|s| s.trim().parse::<u64>().map_err(Error::CouldNotParseNumber))
            .collect::<Result<Vec<_>>>()?;

        let mut seeds = Vec::new();

        for seed_pair in seed_pairs.chunks(2) {
            let seed = seed_pair[0];
            let count = seed_pair[1];

            let seed_range = SeedRange::new(seed, seed + count - 1);

            seeds.push(seed_range);
        }

        Ok((Seeds { seeds }, input))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MapRange {
    destination_start: u64,
    source_start: u64,
    range: u64,
}

impl MapRange {
    #[tracing::instrument]
    fn new(destination_start: u64, source_start: u64, range: u64) -> MapRange {
        MapRange {
            destination_start,
            source_start,
            range,
        }
    }

    #[tracing::instrument]
    fn contains_value(&self, value: u64) -> bool {
        value >= self.source_start && value < self.source_start + self.range
    }

    #[tracing::instrument]
    fn map_value(&self, value: u64) -> u64 {
        if !self.contains_value(value) {
            return value;
        }

        let offset = value - self.source_start;
        let destination = self.destination_start + offset;

        destination
    }

    #[tracing::instrument]
    fn map_contained_seed_range(&self, seed_range: &SeedRange) -> bool {
        if seed_range.start < self.source_start && seed_range.end < self.source_start {
            return false;
        }

        if seed_range.start >= self.source_start + self.range
            && seed_range.end >= self.source_start + self.range
        {
            return false;
        }

        true
    }

    #[tracing::instrument]
    fn map_seed_range(&self, seed_range: SeedRange) -> Vec<SeedRange> {
        // Out of bounds
        if seed_range.end < self.source_start {
            return vec![seed_range];
        }

        if seed_range.start >= self.source_start + self.range {
            return vec![seed_range];
        }

        // Contained
        if seed_range.start >= self.source_start && seed_range.end <= self.source_start + self.range
        {
            let start = self.map_value(seed_range.start);
            let end = self.map_value(seed_range.end);

            return vec![SeedRange::new(start, end)];
        }

        // Left partial

        if seed_range.start < self.source_start && seed_range.end <= self.source_start + self.range
        {
            let end = self.map_value(seed_range.end);

            return vec![
                SeedRange::new(seed_range.start, self.source_start - 1),
                SeedRange::new(self.map_value(self.source_start), self.map_value(end)),
            ];
        }

        // Right partial

        if seed_range.start >= self.source_start && seed_range.end > self.source_start + self.range
        {
            let start = self.map_value(seed_range.start);
            let map_end = self.map_value(self.source_start + self.range - 1);
            let included_span = map_end - start + 1;

            dbg!(start, included_span, map_end);

            return vec![
                SeedRange::new(start, map_end),
                SeedRange::new(self.source_start + self.range, seed_range.end),
            ];
        }

        let mut mapped_ranges = Vec::new();

        let before_range = SeedRange::new(seed_range.start, self.source_start - 1);
        let in_range = SeedRange::new(
            self.map_value(self.source_start),
            self.map_value(self.source_start + self.range - 1),
        );
        let after_range = SeedRange::new(self.source_start + self.range, seed_range.end);

        if !before_range.is_empty() {
            mapped_ranges.push(before_range);
        }

        mapped_ranges.push(in_range);

        if !after_range.is_empty() {
            mapped_ranges.push(after_range);
        }

        mapped_ranges
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    mapped_ranges: Vec<MapRange>,
}

impl Map {
    #[tracing::instrument]
    fn from_input(mut input: Input) -> Result<ParserOutput<Map>> {
        let mut mapped_ranges = Vec::new();

        if !input.next()?.ends_with("map:") {
            return Err(Error::CannotFindMapHeader);
        }

        while let Some(line) = input.peak() {
            if line.is_empty() {
                break;
            }

            let line = input.next()?;

            let numbers = line
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse::<u64>().map_err(Error::CouldNotParseNumber))
                .collect::<Result<Vec<_>>>()?;

            if numbers.len() != 3 {
                return Err(Error::UnexpectedNumberOfValuesForMap(line.to_string()));
            }

            let destination_start = numbers[0];
            let source_start = numbers[1];
            let range = numbers[2];

            let map_range = MapRange::new(destination_start, source_start, range);

            mapped_ranges.push(map_range);
        }

        Ok((Map { mapped_ranges }, input))
    }

    #[tracing::instrument]
    fn get_mapped_value(&self, value: u64) -> u64 {
        self.mapped_ranges
            .iter()
            .find(|map_range| map_range.contains_value(value))
            .map(|map_range| map_range.map_value(value))
            .unwrap_or(value)
    }

    #[tracing::instrument]
    fn map_seed_ranges(&self, seed_ranges: Vec<SeedRange>) -> Vec<SeedRange> {
        let mut new_seed_ranges = vec![];
        for seed_range in seed_ranges {
            let mut mapped = false;
            for map_range in &self.mapped_ranges {
                if map_range.map_contained_seed_range(&seed_range) {
                    println!("here");
                    let mapped_seed_ranges = map_range.map_seed_range(seed_range.clone());
                    new_seed_ranges.extend(mapped_seed_ranges);
                    mapped = true;
                }
            }

            if !mapped {
                new_seed_ranges.push(seed_range.clone());
            }
        }

        new_seed_ranges
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Data {
    seeds: Seeds,
    seed_to_soil_map: Map,
    soil_to_fertilizer_map: Map,
    fertilizer_to_water_map: Map,
    water_to_light_map: Map,
    light_to_temperature_map: Map,
    temparure_to_humity_map: Map,
    humidity_to_location_map: Map,
}

impl Data {
    #[tracing::instrument]
    fn from_input(input: Input) -> Result<Data> {
        let (seeds, mut input) = Seeds::from_input(input)?;

        input.next()?;

        let (seed_to_soil_map, mut input) = Map::from_input(input)?;

        input.next()?;

        let (soil_to_fertilizer_map, mut input) = Map::from_input(input)?;

        input.next()?;

        let (fertilizer_to_water_map, mut input) = Map::from_input(input)?;

        input.next()?;

        let (water_to_light_map, mut input) = Map::from_input(input)?;

        input.next()?;

        let (light_to_temperature_map, mut input) = Map::from_input(input)?;

        input.next()?;

        let (temparure_to_humity_map, mut input) = Map::from_input(input)?;

        input.next()?;

        let (humidity_to_location_map, _) = Map::from_input(input)?;

        Ok(Data {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temparure_to_humity_map,
            humidity_to_location_map,
        })
    }

    #[tracing::instrument]
    fn seeds(&self) -> &Seeds {
        &self.seeds
    }

    #[tracing::instrument]
    fn map_seeds(&self, seed_ranges: Vec<SeedRange>) -> u64 {
        println!("seeds {:?}", seed_ranges);

        let soil = self.seed_to_soil_map.map_seed_ranges(seed_ranges.clone());

        println!("soil {:?}", soil);

        let fertilizer = self.soil_to_fertilizer_map.map_seed_ranges(soil);

        println!("fertilizer {:?}", fertilizer);

        let water = self.fertilizer_to_water_map.map_seed_ranges(fertilizer);

        println!("water {:?}", water);

        let light = self.water_to_light_map.map_seed_ranges(water);

        println!("light {:?}", light);

        let temperature = self.light_to_temperature_map.map_seed_ranges(light);

        println!("temperature {:?}", temperature);

        let humidity = self.temparure_to_humity_map.map_seed_ranges(temperature);

        println!("humidity {:?}", humidity);

        let location = self.humidity_to_location_map.map_seed_ranges(humidity);

        println!("location {:?}", location);

        location.iter().map(|r| r.start).min().unwrap()
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64> {
    let input = Input::from_str(input)?;

    let data = Data::from_input(input)?;

    println!("built data");

    let x = data
        .seeds()
        .seeds
        .iter()
        .map(|seed| data.map_seeds(vec![seed.clone()]))
        .collect::<Vec<_>>();

    println!("{:?}", x);

    let min_location = data
        .seeds()
        .seeds
        .iter()
        .map(|seed| data.map_seeds(vec![seed.clone()]))
        .min()
        .ok_or(Error::NoMinValue)?;

    Ok(min_location)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // #[test]
    // fn it_should_parse_seed() -> miette::Result<()> {
    //     let input = Input::from_str("seeds: 79 14 55 13")?;
    //     let (seeds, _) = Seeds::from_input(input)?;
    //     assert_eq!(
    //         vec![
    //             79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 55, 56, 57, 58, 59, 60, 61,
    //             62, 63, 64, 65, 66, 67
    //         ],
    //         seeds.seeds
    //     );

    //     Ok(())
    // }

    #[test]
    fn it_should_map_seed_range() -> miette::Result<()> {
        let map_range = MapRange::new(70, 50, 5);

        // Not in range
        assert_eq!(
            map_range.map_seed_range(SeedRange::new(90, 92)),
            vec![SeedRange::new(90, 92)]
        );

        assert_eq!(
            map_range.map_seed_range(SeedRange::new(92, 92)),
            vec![SeedRange::new(92, 92)]
        );

        assert_eq!(
            map_range.map_seed_range(SeedRange::new(30, 32)),
            vec![SeedRange::new(30, 32)]
        );

        assert_eq!(
            map_range.map_seed_range(SeedRange::new(30, 30)),
            vec![SeedRange::new(30, 30)]
        );

        // Competely containd
        assert_eq!(
            map_range.map_seed_range(SeedRange::new(50, 52)),
            vec![SeedRange::new(70, 72)]
        );

        assert_eq!(
            map_range.map_seed_range(SeedRange::new(51, 51)),
            vec![SeedRange::new(71, 71)]
        );

        // Left partial
        assert_eq!(
            map_range.map_seed_range(SeedRange::new(48, 51)),
            vec![SeedRange::new(48, 49), SeedRange::new(70, 71)]
        );

        assert_eq!(
            map_range.map_seed_range(SeedRange::new(48, 48)),
            vec![SeedRange::new(48, 48)]
        );

        assert_eq!(
            map_range.map_seed_range(SeedRange::new(50, 50)),
            vec![SeedRange::new(70, 70)]
        );

        // Right partial
        assert_eq!(
            map_range.map_seed_range(SeedRange::new(53, 57)),
            vec![SeedRange::new(73, 74), SeedRange::new(55, 57)]
        );

        assert_eq!(
            map_range.map_seed_range(SeedRange::new(57, 57)),
            vec![SeedRange::new(57, 57)]
        );

        assert_eq!(
            map_range.map_seed_range(SeedRange::new(53, 53)),
            vec![SeedRange::new(73, 73)]
        );

        // Partial

        assert_eq!(
            map_range.map_seed_range(SeedRange::new(48, 57)),
            vec![
                SeedRange::new(48, 49),
                SeedRange::new(70, 74),
                SeedRange::new(55, 57)
            ]
        );

        Ok(())
    }

    #[test]
    fn it_should_map_range_single() -> miette::Result<()> {
        let input = include_str!("../example1.txt");
        let input = Input::from_str(input)?;

        let data = Data::from_input(input)?;

        let seed_range = vec![SeedRange::new(79, 79)];

        let mapped_ranges = data.seed_to_soil_map.map_seed_ranges(seed_range);
        let expected = vec![SeedRange::new(81, 81)];
        assert_eq!(mapped_ranges, expected);

        let mapped_ranges = data.soil_to_fertilizer_map.map_seed_ranges(mapped_ranges);
        let expected = vec![SeedRange::new(81, 81)];
        assert_eq!(mapped_ranges, expected);

        let mapped_ranges = data.fertilizer_to_water_map.map_seed_ranges(mapped_ranges);
        let expected = vec![SeedRange::new(81, 81)];
        assert_eq!(mapped_ranges, expected);

        let mapped_ranges = data.water_to_light_map.map_seed_ranges(mapped_ranges);
        let expected = vec![SeedRange::new(74, 74)];
        assert_eq!(mapped_ranges, expected);

        let mapped_ranges = data.light_to_temperature_map.map_seed_ranges(mapped_ranges);
        let expected = vec![SeedRange::new(78, 78)];
        assert_eq!(mapped_ranges, expected);

        let mapped_ranges = data.temparure_to_humity_map.map_seed_ranges(mapped_ranges);
        let expected = vec![SeedRange::new(78, 78)];
        assert_eq!(mapped_ranges, expected);

        let mapped_ranges = data.humidity_to_location_map.map_seed_ranges(mapped_ranges);
        let expected = vec![SeedRange::new(82, 82)];
        assert_eq!(mapped_ranges, expected);

        Ok(())
    }

    // #[test]
    // fn it_should_map_range_multi() -> miette::Result<()> {
    //     let input = include_str!("../example1.txt");
    //     let input = Input::from_str(input)?;

    //     let data = Data::from_input(input)?;

    //     let seed_range = vec![SeedRange::new(79, 93)];

    //     let mapped_ranges = data.seed_to_soil_map.map_seed_ranges(seed_range);
    //     let expected = vec![SeedRange::new(81, 95)];
    //     assert_eq!(mapped_ranges, expected);

    //     let mapped_ranges = data.soil_to_fertilizer_map.map_seed_ranges(mapped_ranges);
    //     let expected = vec![SeedRange::new(81, 95)];
    //     assert_eq!(mapped_ranges, expected);

    //     let mapped_ranges = data.fertilizer_to_water_map.map_seed_ranges(mapped_ranges);
    //     let expected = vec![SeedRange::new(81, 95)];
    //     assert_eq!(mapped_ranges, expected);

    //     let mapped_ranges = data.water_to_light_map.map_seed_ranges(mapped_ranges);
    //     let expected = vec![SeedRange::new(74, 95)];
    //     assert_eq!(mapped_ranges, expected);

    //     let mapped_ranges = data.light_to_temperature_map.map_seed_ranges(mapped_ranges);
    //     let expected = vec![SeedRange::new(45, 56), SeedRange::new(78, 81)];
    //     assert_eq!(mapped_ranges, expected);

    //     let mapped_ranges = data.temparure_to_humity_map.map_seed_ranges(mapped_ranges);
    //     let expected = vec![SeedRange::new(46, 57), SeedRange::new(78, 81)];
    //     assert_eq!(mapped_ranges, expected);

    //     let mapped_ranges = data.humidity_to_location_map.map_seed_ranges(mapped_ranges);
    //     let expected = vec![
    //         SeedRange::new(46, 56),
    //         SeedRange::new(60, 61),
    //         SeedRange::new(82, 85),
    //     ];
    //     assert_eq!(mapped_ranges, expected);

    //     Ok(())
    // }

    // #[test]
    // fn it_should_map_range_broken_example() -> miette::Result<()> {
    //     let input = include_str!("../example1.txt");
    //     let input = Input::from_str(input)?;

    //     let data = Data::from_input(input)?;

    //     let seed_range = vec![SeedRange::new(74, 88)];

    //     let mapped_ranges = data.light_to_temperature_map.map_seed_ranges(seed_range);
    //     let expected = vec![SeedRange::new(45, 56), SeedRange::new(78, 81)];
    //     println!("{:?}", mapped_ranges);
    //     assert_eq!(mapped_ranges, expected);


    //     Ok(())
    // }

    // #[test]
    // fn it_should_parse_map() -> miette::Result<()> {
    //     let input = Input::from_str(
    //         "seed-to-soil map:
    //     50 98 2
    //     52 50 48",
    //     )?;

    //     let (map, _) = Map::from_input(input)?;

    //     assert_eq!(map.get_mapped_value(0), 0);
    //     assert_eq!(map.get_mapped_value(1), 1);

    //     assert_eq!(map.get_mapped_value(48), 48);
    //     assert_eq!(map.get_mapped_value(49), 49);
    //     assert_eq!(map.get_mapped_value(50), 52);
    //     assert_eq!(map.get_mapped_value(51), 53);

    //     assert_eq!(map.get_mapped_value(96), 98);
    //     assert_eq!(map.get_mapped_value(97), 99);
    //     assert_eq!(map.get_mapped_value(98), 50);
    //     assert_eq!(map.get_mapped_value(99), 51);

    //     assert_eq!(map.get_mapped_value(79), 81);
    //     assert_eq!(map.get_mapped_value(14), 14);
    //     assert_eq!(map.get_mapped_value(55), 57);
    //     assert_eq!(map.get_mapped_value(13), 13);

    //     Ok(())
    // }

    // #[test]
    // fn it_should_process_data() -> miette::Result<()> {
    //     let input = include_str!("../example1.txt");
    //     let input = Input::from_str(input)?;

    //     let data = Data::from_input(input)?;

    //     assert_eq!(data.map_seeds(vec![SeedRange::new(79, 79)]), 82);
    //     assert_eq!(data.map_seeds(vec![SeedRange::new(14, 14)]), 43);
    //     assert_eq!(data.map_seeds(vec![SeedRange::new(55, 55)]), 86);
    //     assert_eq!(data.map_seeds(vec![SeedRange::new(13, 13)]), 35);
    //     Ok(())
    // }

    // #[test]
    // fn test_process() -> miette::Result<()> {
    //     let input = include_str!("../example1.txt");
    //     assert_eq!(46, process(input)?);
    //     // assert_eq!(463, process(input)?);
    //     Ok(())
    // }

    // #[test]
    // fn it_should_be_correct_for_real_data() -> miette::Result<()> {
    //     let input = include_str!("../input2.txt");
    //     assert_eq!(process(input)?, 56931769);
    //     Ok(())
    // }
}
