use crate::{error::Error, prelude::*};

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
    seeds: Vec<u64>,
}

impl Seeds {
    #[tracing::instrument]
    fn from_input(mut input: Input) -> Result<ParserOutput<Seeds>> {
        let first_line = input.next().map_err(|_| Error::CannotFindSeedsHeader)?;

        if !first_line.starts_with("seeds:") {
            return Err(Error::CannotFindSeedsHeader);
        }

        let seeds = first_line
            .split(':')
            .last()
            .ok_or_else(|| Error::CannotFindSeedsHeader)?
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<u64>().map_err(Error::CouldNotParseNumber))
            .collect::<Result<Vec<_>>>()?;

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
    fn new(destination_start: u64, source_start: u64, range: u64) -> Result<MapRange> {
        Ok(MapRange {
            destination_start,
            source_start,
            range,
        })
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    mapped_values: Vec<MapRange>,
}

impl Map {
    #[tracing::instrument]
    fn from_input(mut input: Input) -> Result<ParserOutput<Map>> {
        let mut mapped_values = Vec::new();

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

            let map_range = MapRange::new(destination_start, source_start, range)?;

            mapped_values.push(map_range);            
        }

        Ok((Map { mapped_values }, input))
    }

    #[tracing::instrument]
    fn get_mapped_value(&self, value: u64) -> u64 {
        self.mapped_values
            .iter()
            .find(|map_range| map_range.contains_value(value))
            .map(|map_range| map_range.map_value(value))
            .unwrap_or(value)
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
    fn map_seed(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil_map.get_mapped_value(seed);
        let fertilizer = self.soil_to_fertilizer_map.get_mapped_value(soil);
        let water = self.fertilizer_to_water_map.get_mapped_value(fertilizer);
        let light = self.water_to_light_map.get_mapped_value(water);
        let temperature = self.light_to_temperature_map.get_mapped_value(light);
        let humidity = self.temparure_to_humity_map.get_mapped_value(temperature);

        self.humidity_to_location_map.get_mapped_value(humidity)
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64> {
    let input = Input::from_str(input)?;

    let data = Data::from_input(input)?;

    let min_location = data
        .seeds()
        .seeds
        .iter()
        .map(|seed| data.map_seed(*seed))
        .min()
        .ok_or(Error::NoMinValue)?;

    Ok(min_location)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_parse_seed() -> miette::Result<()> {
        let input = Input::from_str("seeds: 79 14 55 13")?;
        let (seeds, _) = Seeds::from_input(input)?;
        assert_eq!(vec![79, 14, 55, 13], seeds.seeds);

        Ok(())
    }

    #[test]
    fn it_should_parse_map() -> miette::Result<()> {
        let input = Input::from_str(
            "seed-to-soil map:
        50 98 2
        52 50 48",
        )?;

        let (map, _) = Map::from_input(input)?;

        assert_eq!(map.get_mapped_value(0), 0);
        assert_eq!(map.get_mapped_value(1), 1);

        assert_eq!(map.get_mapped_value(48), 48);
        assert_eq!(map.get_mapped_value(49), 49);
        assert_eq!(map.get_mapped_value(50), 52);
        assert_eq!(map.get_mapped_value(51), 53);

        assert_eq!(map.get_mapped_value(96), 98);
        assert_eq!(map.get_mapped_value(97), 99);
        assert_eq!(map.get_mapped_value(98), 50);
        assert_eq!(map.get_mapped_value(99), 51);

        assert_eq!(map.get_mapped_value(79), 81);
        assert_eq!(map.get_mapped_value(14), 14);
        assert_eq!(map.get_mapped_value(55), 57);
        assert_eq!(map.get_mapped_value(13), 13);

        Ok(())
    }

    #[test]
    fn it_should_process_data() -> miette::Result<()> {
        let input = include_str!("../example1.txt");
        let input = Input::from_str(input)?;

        let data = Data::from_input(input)?;

        assert_eq!(data.map_seed(79), 82);
        assert_eq!(data.map_seed(14), 43);
        assert_eq!(data.map_seed(55), 86);
        assert_eq!(data.map_seed(13), 35);

        // assert_eq!(35, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../example1.txt");
        assert_eq!(35, process(input)?);
        Ok(())
    }
}
