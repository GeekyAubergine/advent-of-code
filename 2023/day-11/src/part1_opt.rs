use std::collections::HashMap;

use crate::prelude::*;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
struct Line {
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
}

#[derive(Debug, Clone, PartialEq)]
struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    chars: Vec<char>,
    width: usize,
    height: usize,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl Input {
    #[tracing::instrument]
    fn new(input: &str) -> Self {
        let lines = input.lines().map(|l| l.trim()).collect::<Vec<_>>();

        let width = lines[0].len();
        let height = lines.len();

        let chars = lines.iter().flat_map(|l| l.chars()).collect::<Vec<_>>();

        let empty_rows = (0..height)
            .filter(|y| {
                for x in 0..width {
                    if chars[*y * width + x] == '#' {
                        return false;
                    }
                }
                true
            })
            .collect::<Vec<_>>();

        let empty_cols = (0..width)
            .filter(|x| {
                for y in 0..height {
                    if chars[y * width + *x] == '#' {
                        return false;
                    }
                }
                true
            })
            .collect::<Vec<_>>();

        Self {
            chars,
            width,
            height,
            empty_rows,
            empty_cols,
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

    #[tracing::instrument]
    fn is_row_empty(&self, y: usize) -> bool {
        self.empty_rows.contains(&y)
    }

    #[tracing::instrument]
    fn is_col_empty(&self, x: usize) -> bool {
        self.empty_cols.contains(&x)
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
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        dx.abs() + dy.abs()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct GalaxyMap {
    galaxies: HashMap<u16, Galaxy>,
}

impl GalaxyMap {
    #[tracing::instrument]
    fn new() -> Self {
        Self {
            galaxies: HashMap::new(),
        }
    }

    #[tracing::instrument]
    fn from_input(input: &Input) -> Self {
        let mut map = Self::new();

        let mut id = 1;

        let mut y_offset = 0;

        for y in 0..input.height {
            if input.is_row_empty(y) {
                y_offset += 1;
            }
            let mut x_offset = 0;
            for x in 0..input.width {
                if input.is_col_empty(x) {
                    x_offset += 1;
                }
                if input.get(x, y) == Some('#') {
                    map.add(Galaxy::new(
                        id,
                        (x + x_offset) as f32,
                        (y + y_offset) as f32,
                    ));
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
    fn distance(&self, a: u16, b: u16) -> u32 {
        self.galaxies[&a].distance(&self.galaxies[&b]) as u32
    }

    #[tracing::instrument]
    fn galaxy_ids(&self) -> Vec<u16> {
        self.galaxies.keys().copied().collect::<Vec<_>>()
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<u32> {
    let input = Input::new(input);

    let map = GalaxyMap::from_input(&input);

    let galaxy_ids = map.galaxy_ids();

    let mut galaxys_to_compute: Vec<(u16, u16)> = Vec::new();

    for a in 0..galaxy_ids.len() {
        for b in a + 1..galaxy_ids.len() {
            galaxys_to_compute.push((galaxy_ids[a], galaxy_ids[b]));
        }
    }

    let total_distance = galaxys_to_compute
        .par_iter()
        .map(|(a, b)| map.distance(*a, *b))
        .sum::<u32>();

    Ok(total_distance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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

    #[test]
    fn it_should_get_right_output() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!(9565386, process(input)?);
        Ok(())
    }
}
