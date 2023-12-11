use std::collections::HashMap;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    chars: Vec<char>,
    width: usize,
    height: usize,
}

impl Input {
    #[tracing::instrument]
    fn new(input: &str) -> Self {
        let lines = input.lines().map(|l| l.trim()).collect::<Vec<_>>();
        let width = lines[0].len();

        // Expand "empty" rows to two "empty" rows
        let rows = lines
            .iter()
            .flat_map(|row| {
                if row.chars().all(|c| c == '.') {
                    vec![row, row]
                } else {
                    vec![row]
                }
            })
            .collect::<Vec<_>>();

        let height = rows.len();

        let mut cols = vec![];

        for x in 0..width {
            let mut col = vec![];
            for y in 0..height {
                col.push(rows[y].chars().nth(x).unwrap());
            }
            cols.push(col);
        }

        let cols = cols
            .iter()
            .flat_map(|col| {
                if col.iter().all(|&c| c == '.') {
                    vec![col.clone(), col.clone()]
                } else {
                    vec![col.clone()]
                }
            })
            .collect::<Vec<_>>();

        let width = cols.len();

        let mut chars = vec![];

        for y in 0..height {
            for x in 0..width {
                chars.push(cols[x][y]);
            }
        }

        Self {
            chars,
            width,
            height,
        }
    }

    #[tracing::instrument]
    fn to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push(self.chars[y * self.width + x]);
            }
            s.push('\n');
        }
        s
    }

    #[tracing::instrument]
    fn get(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.chars[y * self.width + x])
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Galaxy {
    id: u16,
    x: f32,
    y: f32,
}

impl Galaxy {
    #[tracing::instrument]
    fn new(id: u16, x: f32, y: f32) -> Self {
        Self { id, x, y }
    }

    #[tracing::instrument]
    fn distance(&self, other: &Self) -> f32 {
        let mut distance = 0.0;

        let mut x = self.x;
        let mut y = self.y;

        while x != other.x || y != other.y {
            let dx = other.x - x;
            let dy = other.y - y;

            if dx.abs() >= dy.abs() {
                x += dx.signum();
            } else {
                y += dy.signum();
            }

            distance += 1.0;
        }

        distance
    }
}

#[tracing::instrument]
fn galaxy_distance_hash_id(galaxy_a: u16, galaxy_b: u16) -> u32 {
    let min = galaxy_a.min(galaxy_b);
    let max = galaxy_a.max(galaxy_b);

    (min as u32) << 16 | (max as u32)
}

#[derive(Debug, Clone, PartialEq)]
struct GalaxyMap {
    galaxies: HashMap<u16, Galaxy>,
    galaxy_distances: HashMap<u32, u32>,
}

impl GalaxyMap {
    #[tracing::instrument]
    fn new() -> Self {
        Self {
            galaxies: HashMap::new(),
            galaxy_distances: HashMap::new(),
        }
    }

    #[tracing::instrument]
    fn from_input(input: &Input) -> Self {
        let mut map = Self::new();

        let mut id = 1;

        for y in 0..input.height {
            for x in 0..input.width {
                if input.get(x, y) == Some('#') {
                    map.add(Galaxy::new(id, x as f32, y as f32));
                    id += 1;
                }
            }
        }

        map
    }

    #[tracing::instrument]
    fn add(&mut self, galaxy: Galaxy) {
        self.galaxies.insert(galaxy.id, galaxy);
    }

    #[tracing::instrument]
    fn distance(&mut self, a: u16, b: u16) -> u32 {
        let key = galaxy_distance_hash_id(a, b);
        if let Some(distance) = self.galaxy_distances.get(&key) {
            *distance
        } else {
            let distance = self.galaxies[&a].distance(&self.galaxies[&b]) as u32;
            self.galaxy_distances.insert(key, distance);
            distance
        }
    }

    #[tracing::instrument]
    fn galaxy_ids(&self) -> Vec<u16> {
        self.galaxies.keys().copied().collect::<Vec<_>>()
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<u32> {
    let input = Input::new(input);

    let mut map = GalaxyMap::from_input(&input);

    let mut galaxy_ids = map.galaxy_ids();

    let mut total_distance = 0;
    let mut x = 0;

    for a in 0..galaxy_ids.len() {
        for b in a + 1..galaxy_ids.len() {
            let distance = map.distance(galaxy_ids[a], galaxy_ids[b]);
            total_distance += distance;
            x += 1;

            // println!(
            //     "distance from {} to {}: {}",
            //     galaxy_ids[a], galaxy_ids[b], distance
            // );
        }
    }

    // println!("total distance: {}", total_distance);
    // println!("x: {}", x);

    Ok(total_distance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_expand_input() -> miette::Result<()> {
        let input = Input::new(
            "...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....",
        );

        assert_eq!(input.width, 13);
        assert_eq!(input.height, 12);

        assert_eq!(input.get(0, 0), Some('.'));
        assert_eq!(input.get(1, 0), Some('.'));
        assert_eq!(input.get(2, 0), Some('.'));
        assert_eq!(input.get(3, 0), Some('.'));
        assert_eq!(input.get(4, 0), Some('#'));
        assert_eq!(input.get(5, 0), Some('.'));
        assert_eq!(input.get(6, 0), Some('.'));
        assert_eq!(input.get(7, 0), Some('.'));
        assert_eq!(input.get(8, 0), Some('.'));
        assert_eq!(input.get(9, 0), Some('.'));
        assert_eq!(input.get(10, 0), Some('.'));
        assert_eq!(input.get(11, 0), Some('.'));
        assert_eq!(input.get(12, 0), Some('.'));

        assert_eq!(input.get(0, 0), Some('.'));
        assert_eq!(input.get(0, 1), Some('.'));
        assert_eq!(input.get(0, 2), Some('#'));
        assert_eq!(input.get(0, 3), Some('.'));
        assert_eq!(input.get(0, 4), Some('.'));
        assert_eq!(input.get(0, 5), Some('.'));
        assert_eq!(input.get(0, 6), Some('.'));
        assert_eq!(input.get(0, 7), Some('.'));
        assert_eq!(input.get(0, 8), Some('.'));
        assert_eq!(input.get(0, 9), Some('.'));
        assert_eq!(input.get(0, 10), Some('.'));
        assert_eq!(input.get(0, 11), Some('#'));

        Ok(())
    }

    #[test]
    fn it_should_find_galaxies() -> miette::Result<()> {
        let input = Input::new(
            "...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....",
        );

        let map = GalaxyMap::from_input(&input);

        assert_eq!(map.galaxies.len(), 9);

        assert_eq!(map.galaxies[&1].x, 4.0);
        assert_eq!(map.galaxies[&1].y, 0.0);

        assert_eq!(map.galaxies[&2].x, 9.0);
        assert_eq!(map.galaxies[&2].y, 1.0);

        assert_eq!(map.galaxies[&3].x, 0.0);
        assert_eq!(map.galaxies[&3].y, 2.0);

        assert_eq!(map.galaxies[&4].x, 8.0);
        assert_eq!(map.galaxies[&4].y, 5.0);

        assert_eq!(map.galaxies[&5].x, 1.0);
        assert_eq!(map.galaxies[&5].y, 6.0);

        assert_eq!(map.galaxies[&6].x, 12.0);
        assert_eq!(map.galaxies[&6].y, 7.0);

        assert_eq!(map.galaxies[&7].x, 9.0);
        assert_eq!(map.galaxies[&7].y, 10.0);

        assert_eq!(map.galaxies[&8].x, 0.0);
        assert_eq!(map.galaxies[&8].y, 11.0);

        assert_eq!(map.galaxies[&9].x, 5.0);
        assert_eq!(map.galaxies[&9].y, 11.0);

        Ok(())
    }

    #[test]
    fn it_should_calculate_distances() -> miette::Result<()> {
        let input = Input::new(
            "...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....",
        );

        let mut map = GalaxyMap::from_input(&input);

        // ....1........
        // .........2...
        // 3............
        // .............
        // .............
        // ........4....
        // .5...........
        // .##.........6
        // ..##.........
        // ...##........
        // ....##...7...
        // 8....9.......

        assert_eq!(map.distance(5, 9), 9);
        assert_eq!(map.distance(9, 5), 9);

        assert_eq!(map.distance(1, 7), 15);
        assert_eq!(map.distance(3, 6), 17);
        assert_eq!(map.distance(8, 9), 5);

        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";
        assert_eq!(374, process(input)?);
        Ok(())
    }
}
