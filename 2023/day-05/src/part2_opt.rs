use crate::{error::Error, prelude::*};
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Range {
    start: i64,
    end: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RangeIntersection {
    before: Option<Range>,
    overlapping: Option<Range>,
    after: Option<Range>,
}

#[tracing::instrument]
fn intersect_range(base: &Range, other: &Range) -> RangeIntersection {
    // Excluded
    if other.end < base.start {
        return RangeIntersection {
            before: Some(other.clone()),
            overlapping: None,
            after: None,
        };
    }

    if other.start > base.end {
        return RangeIntersection {
            before: None,
            overlapping: None,
            after: Some(other.clone()),
        };
    }

    // Contained
    if other.start >= base.start && other.end <= base.end {
        return RangeIntersection {
            before: None,
            overlapping: Some(other.clone()),
            after: None,
        };
    }

    // Left partial

    if other.start < base.start && other.end <= base.end {
        return RangeIntersection {
            before: Some(Range::new(other.start, base.start - 1)),
            overlapping: Some(Range::new(base.start, other.end)),
            after: None,
        };
    }

    // Right partial

    if other.start >= base.start && other.end > base.end {
        return RangeIntersection {
            before: None,
            overlapping: Some(Range::new(other.start, base.end)),
            after: Some(Range::new(base.end + 1, other.end)),
        };
    }

    // Partial

    RangeIntersection {
        before: Some(Range::new(other.start, base.start - 1)),
        overlapping: Some(Range::new(base.start, base.end)),
        after: Some(Range::new(base.end + 1, other.end)),
    }
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
        value >= self.start && value <= self.end
    }

    #[tracing::instrument]
    fn overlaps(&self, other: &Range) -> bool {
        self.contains(other.start) || self.contains(other.end)
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Seeds {
    seeds: Vec<Range>,
}

impl Seeds {
    #[tracing::instrument]
    fn from_input(mut input: Input) -> Result<(Seeds, Input)> {
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

    #[tracing::instrument]
    fn intersect(&self, other: &MapRange) -> RangeIntersection {
        intersect_range(&self.source_range, &other.source_range)
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

    #[tracing::instrument]
    fn apply_to_range(&self, other: &Range) -> Vec<Range> {
        let mut unmapped_ranges = vec![other.clone()];
        let mut mapped_ranges = Vec::new();

        for map_range in &self.mapped_ranges {
            let mut new_unmapped_ranges = Vec::new();

            for unmapped_range in unmapped_ranges {
                let intersection = map_range.intersect(&MapRange::new(
                    0,
                    unmapped_range.start,
                    unmapped_range.end - unmapped_range.start + 1,
                ));

                if let Some(before) = intersection.before {
                    new_unmapped_ranges.push(before);
                }

                if let Some(overlapping) = intersection.overlapping {
                    mapped_ranges.push(Range::new(
                        map_range.apply_to_value(overlapping.start),
                        map_range.apply_to_value(overlapping.end),
                    ));
                }

                if let Some(after) = intersection.after {
                    new_unmapped_ranges.push(after);
                }
            }

            unmapped_ranges = new_unmapped_ranges;
        }

        mapped_ranges.extend(unmapped_ranges);

        mapped_ranges
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
fn process_seed_range(seed_range: &Range, maps: &[Map]) -> Vec<Range> {
    let mut mapped_ranges = vec![seed_range.clone()];

    for map in maps {
        let mut new_mapped_ranges = Vec::new();

        for mapped_range in mapped_ranges {
            new_mapped_ranges.extend(map.apply_to_range(&mapped_range));
        }

        mapped_ranges = new_mapped_ranges;
    }

    mapped_ranges
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i64> {
    let input = Input::from_str(input)?;

    let (seeds, input) = Seeds::from_input(input)?;

    let maps = maps_from_input(input)?;

    // let data = Data::from_input(input)?;

    // println!("built data");

    // let x = data
    //     .seeds()
    //     .seeds
    //     .iter()
    //     .map(|seed| data.map_seeds(vec![seed.clone()]))
    //     .collect::<Vec<_>>();

    // println!("{:?}", x);

    let min_location = seeds
        .seeds
        .iter()
        .flat_map(|seed| process_seed_range(seed, &maps))
        .map(|range| range.start)
        .min()
        .ok_or(Error::NoMinValue)?;

    Ok(min_location)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_calculate_range_overlaps_correctly() -> miette::Result<()> {
        assert_eq!(
            intersect_range(&Range::new(3, 7), &Range::new(1, 2)),
            RangeIntersection {
                before: Some(Range::new(1, 2)),
                overlapping: None,
                after: None,
            }
        );

        assert_eq!(
            intersect_range(&Range::new(3, 7), &Range::new(8, 9)),
            RangeIntersection {
                before: None,
                overlapping: None,
                after: Some(Range::new(8, 9)),
            }
        );

        assert_eq!(
            intersect_range(&Range::new(3, 7), &Range::new(1, 9)),
            RangeIntersection {
                before: Some(Range::new(1, 2)),
                overlapping: Some(Range::new(3, 7)),
                after: Some(Range::new(8, 9)),
            }
        );

        assert_eq!(
            intersect_range(&Range::new(3, 7), &Range::new(1, 5)),
            RangeIntersection {
                before: Some(Range::new(1, 2)),
                overlapping: Some(Range::new(3, 5)),
                after: None,
            }
        );

        assert_eq!(
            intersect_range(&Range::new(3, 7), &Range::new(5, 9)),
            RangeIntersection {
                before: None,
                overlapping: Some(Range::new(5, 7)),
                after: Some(Range::new(8, 9)),
            }
        );

        Ok(())
    }

    // #[test]
    // fn it_should_map_seed_range() -> miette::Result<()> {
    //     let map_range = MapRange::new(70, 50, 5);

    //     // Not in range
    //     assert_eq!(
    //         map_range.map_seed_range(Range::new(90, 92)),
    //         vec![Range::new(90, 92)]
    //     );

    //     assert_eq!(
    //         map_range.map_seed_range(Range::new(92, 92)),
    //         vec![Range::new(92, 92)]
    //     );

    //     assert_eq!(
    //         map_range.map_seed_range(Range::new(30, 32)),
    //         vec![Range::new(30, 32)]
    //     );

    //     assert_eq!(
    //         map_range.map_seed_range(Range::new(30, 30)),
    //         vec![Range::new(30, 30)]
    //     );

    //     // Competely containd
    //     assert_eq!(
    //         map_range.map_seed_range(Range::new(50, 52)),
    //         vec![Range::new(70, 72)]
    //     );

    //     assert_eq!(
    //         map_range.map_seed_range(Range::new(51, 51)),
    //         vec![Range::new(71, 71)]
    //     );

    //     // Left partial
    //     assert_eq!(
    //         map_range.map_seed_range(Range::new(48, 51)),
    //         vec![Range::new(48, 49), Range::new(70, 71)]
    //     );

    //     assert_eq!(
    //         map_range.map_seed_range(Range::new(48, 48)),
    //         vec![Range::new(48, 48)]
    //     );

    //     assert_eq!(
    //         map_range.map_seed_range(Range::new(50, 50)),
    //         vec![Range::new(70, 70)]
    //     );

    //     // Right partial
    //     assert_eq!(
    //         map_range.map_seed_range(Range::new(53, 57)),
    //         vec![Range::new(73, 74), Range::new(55, 57)]
    //     );

    //     assert_eq!(
    //         map_range.map_seed_range(Range::new(57, 57)),
    //         vec![Range::new(57, 57)]
    //     );

    //     assert_eq!(
    //         map_range.map_seed_range(Range::new(53, 53)),
    //         vec![Range::new(73, 73)]
    //     );

    //     // Partial

    //     assert_eq!(
    //         map_range.map_seed_range(Range::new(48, 57)),
    //         vec![Range::new(48, 49), Range::new(70, 74), Range::new(55, 57)]
    //     );

    //     Ok(())
    // }

    // #[test]
    // fn it_should_map_range_single() -> miette::Result<()> {
    //     let input = include_str!("../example1.txt");
    //     let input = Input::from_str(input)?;

    //     let data = Data::from_input(input)?;

    //     let seed_range = vec![Range::new(79, 79)];

    //     let mapped_ranges = data.seed_to_soil_map.map_seed_ranges(seed_range);
    //     let expected = vec![Range::new(81, 81)];
    //     assert_eq!(mapped_ranges, expected);

    //     let mapped_ranges = data.soil_to_fertilizer_map.map_seed_ranges(mapped_ranges);
    //     let expected = vec![Range::new(81, 81)];
    //     assert_eq!(mapped_ranges, expected);

    //     let mapped_ranges = data.fertilizer_to_water_map.map_seed_ranges(mapped_ranges);
    //     let expected = vec![Range::new(81, 81)];
    //     assert_eq!(mapped_ranges, expected);

    //     let mapped_ranges = data.water_to_light_map.map_seed_ranges(mapped_ranges);
    //     let expected = vec![Range::new(74, 74)];
    //     assert_eq!(mapped_ranges, expected);

    //     let mapped_ranges = data.light_to_temperature_map.map_seed_ranges(mapped_ranges);
    //     let expected = vec![Range::new(78, 78)];
    //     assert_eq!(mapped_ranges, expected);

    //     let mapped_ranges = data.temparure_to_humity_map.map_seed_ranges(mapped_ranges);
    //     let expected = vec![Range::new(78, 78)];
    //     assert_eq!(mapped_ranges, expected);

    //     let mapped_ranges = data.humidity_to_location_map.map_seed_ranges(mapped_ranges);
    //     let expected = vec![Range::new(82, 82)];
    //     assert_eq!(mapped_ranges, expected);

    //     Ok(())
    // }

    // #[test]
    // fn it_should_map_range_multi() -> miette::Result<()> {
    //     let input = include_str!("../example1.txt");
    //     let input = Input::from_str(input)?;

    //     let data = Data::from_input(input)?;

    //     let seed_range = vec![Range::new(79, 93)];

    //     let soil = data.seed_to_soil_map.map_seed_ranges(seed_range);
    //     let expected = vec![Range::new(81, 95)];

    //     dbg!(&soil);

    //     assert_eq!(soil, expected);

    //     // assert!(false);

    //     let fertilizer = data.soil_to_fertilizer_map.map_seed_ranges(soil);
    //     let expected = vec![Range::new(81, 95)];
    //     assert_eq!(fertilizer, expected);

    //     let water = data.fertilizer_to_water_map.map_seed_ranges(fertilizer);
    //     let expected = vec![Range::new(81, 95)];
    //     assert_eq!(water, expected);

    //     let light = data.water_to_light_map.map_seed_ranges(water);
    //     let expected = vec![Range::new(74, 88)];
    //     assert_eq!(light, expected);

    //     dbg!(&light);

    //     let temperature = data.light_to_temperature_map.map_seed_ranges(light);
    //     let expected = vec![Range::new(45, 56), Range::new(78, 81)];

    //     dbg!(&temperature);

    //     assert_eq!(temperature, expected);

    //     let humidity = data.temparure_to_humity_map.map_seed_ranges(temperature);
    //     let expected = vec![Range::new(46, 57), Range::new(78, 81)];
    //     assert_eq!(humidity, expected);

    //     let mapped_ranges = data.humidity_to_location_map.map_seed_ranges(humidity);
    //     let expected = vec![
    //         Range::new(46, 56),
    //         Range::new(60, 61),
    //         Range::new(82, 85),
    //     ];
    //     assert_eq!(mapped_ranges, expected);

    //     Ok(())
    // }

    // #[test]
    // fn it_should_map_range_broken_example() -> miette::Result<()> {
    //     let input = include_str!("../example1.txt");
    //     let input = Input::from_str(input)?;

    //     let data = Data::from_input(input)?;

    //     let seed_range = vec![Range::new(74, 88)];

    //     let mapped_ranges = data.light_to_temperature_map.map_seed_ranges(seed_range);
    //     let expected = vec![Range::new(45, 56), Range::new(78, 81)];
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

    //     assert_eq!(data.map_seeds(vec![Range::new(79, 79)]), 82);
    //     assert_eq!(data.map_seeds(vec![Range::new(14, 14)]), 43);
    //     assert_eq!(data.map_seeds(vec![Range::new(55, 55)]), 86);
    //     assert_eq!(data.map_seeds(vec![Range::new(13, 13)]), 35);
    //     Ok(())
    // }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../example1.txt");
        assert_eq!(process(input)?, 46);
        // assert_eq!(463, process(input)?);
        Ok(())
    }

    #[test]
    fn it_should_be_correct_for_real_data() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!(process(input)?, 56931769);
        Ok(())
    }
}
