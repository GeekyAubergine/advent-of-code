use itertools::Itertools;

use crate::{error::Error, prelude::*};
use colored::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Vertical,   // |
    Horizontal, // -
    L,          // L
    J,          // J
    Seven,      // 7
    F,          // F
    Ground,     // .
    Start,      // S
}

impl TryFrom<char> for Pipe {
    type Error = Error;

    #[tracing::instrument]
    fn try_from(c: char) -> Result<Self> {
        match c {
            '|' => Ok(Pipe::Vertical),
            '-' => Ok(Pipe::Horizontal),
            'L' => Ok(Pipe::L),
            'J' => Ok(Pipe::J),
            'F' => Ok(Pipe::F),
            '7' => Ok(Pipe::Seven),
            '.' => Ok(Pipe::Ground),
            'S' => Ok(Pipe::Start),
            _ => Err(Error::UnknownPipe(c)),
        }
    }
}

impl Pipe {
    #[tracing::instrument]
    fn exit_direction(&self, entry_direction: &Direction) -> Option<Direction> {
        match self {
            Pipe::Vertical => match entry_direction {
                Direction::Up => Some(Direction::Up),
                Direction::Down => Some(Direction::Down),
                Direction::Right => None,
                Direction::Left => None,
            },
            Pipe::Horizontal => match entry_direction {
                Direction::Up => None,
                Direction::Down => None,
                Direction::Right => Some(Direction::Right),
                Direction::Left => Some(Direction::Left),
            },
            // L
            Pipe::L => match entry_direction {
                Direction::Up => None,
                Direction::Down => Some(Direction::Right),
                Direction::Right => None,
                Direction::Left => Some(Direction::Up),
            },
            // J
            Pipe::J => match entry_direction {
                Direction::Up => None,
                Direction::Down => Some(Direction::Left),
                Direction::Right => Some(Direction::Up),
                Direction::Left => None,
            },
            // F
            Pipe::F => match entry_direction {
                Direction::Up => Some(Direction::Right),
                Direction::Down => None,
                Direction::Right => None,
                Direction::Left => Some(Direction::Down),
            },
            // 7
            Pipe::Seven => match entry_direction {
                Direction::Up => Some(Direction::Left),
                Direction::Down => None,
                Direction::Right => Some(Direction::Down),
                Direction::Left => None,
            },
            Pipe::Ground => match entry_direction {
                Direction::Up => None,
                Direction::Down => None,
                Direction::Right => None,
                Direction::Left => None,
            },
            Pipe::Start => match entry_direction {
                Direction::Up => None,
                Direction::Down => None,
                Direction::Right => None,
                Direction::Left => None,
            },
        }
    }

    #[tracing::instrument]
    fn to_char(&self) -> char {
        match self {
            Pipe::Vertical => '|',
            Pipe::Horizontal => '-',
            Pipe::L => 'L',
            Pipe::J => 'J',
            Pipe::F => 'F',
            Pipe::Seven => '7',
            Pipe::Ground => '.',
            Pipe::Start => 'S',
        }
    }

    #[tracing::instrument]
    fn is_corner(&self) -> bool {
        match self {
            Pipe::Vertical => false,
            Pipe::Horizontal => false,
            Pipe::L => true,
            Pipe::J => true,
            Pipe::F => true,
            Pipe::Seven => true,
            Pipe::Ground => false,
            Pipe::Start => false,
        }
    }
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[tracing::instrument]
    fn move_in_direction(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self::new(self.x, self.y - 1),
            Direction::Down => Self::new(self.x, self.y + 1),
            Direction::Right => Self::new(self.x + 1, self.y),
            Direction::Left => Self::new(self.x - 1, self.y),
        }
    }

    #[tracing::instrument]
    fn direction_from_previous(&self, previous: &Self) -> Direction {
        if self.x > previous.x {
            Direction::Right
        } else if self.x < previous.x {
            Direction::Left
        } else if self.y > previous.y {
            Direction::Down
        } else if self.y < previous.y {
            Direction::Up
        } else {
            panic!("same position");
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PipeMap {
    map: Vec<Vec<Pipe>>,
    start: Position,
    width: usize,
    height: usize,
}

impl PipeMap {
    #[tracing::instrument]
    fn new(map: Vec<Vec<Pipe>>, start: Position) -> Self {
        let width = map[0].len();
        let height = map.len();
        Self {
            map,
            start,
            width,
            height,
        }
    }

    #[tracing::instrument]
    fn from_str(input: &str) -> Result<Self> {
        let mut map = Vec::new();
        let mut start = None;
        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.trim().chars().enumerate() {
                let pipe = Pipe::try_from(c)?;
                if pipe == Pipe::Start {
                    start = Some(Position::new(x as i32, y as i32));
                }
                row.push(pipe);
            }
            map.push(row);
        }
        let start = start.ok_or(Error::NoStart)?;

        let width = map[0].len();
        let height = map.len();

        Ok(Self {
            map,
            start,
            width,
            height,
        })
    }

    #[tracing::instrument]
    fn get(&self, position: &Position) -> Option<&Pipe> {
        self.map.get(position.y as usize)?.get(position.x as usize)
    }

    #[tracing::instrument]
    fn to_string(&self) -> String {
        let mut output = String::new();
        for row in &self.map {
            for pipe in row {
                output.push(pipe.to_char());
            }
            output.push('\n');
        }
        output
    }

    #[tracing::instrument]
    fn set(&mut self, position: &Position, pipe: Pipe) {
        self.map[position.y as usize][position.x as usize] = pipe;
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Walk {
    positions: Vec<Position>,
    direction: Direction,
}

impl Walk {
    #[tracing::instrument]
    fn new(positions: Vec<Position>, direction: Direction) -> Self {
        Self {
            positions,
            direction,
        }
    }

    #[tracing::instrument]
    fn follow_path(&mut self, map: &PipeMap) -> Result<()> {
        loop {
            let current_position = self.positions.last().ok_or(Error::NoCurrentPosition)?;

            let next_position = current_position.move_in_direction(self.direction);

            let next_pipe = map.get(&next_position).ok_or_else(|| {
                Error::CouldNotFindPipeForPosition(next_position.x, next_position.y)
            })?;

            if next_pipe == &Pipe::Start {
                return Ok(());
            }

            if let Some(exit_direction) = next_pipe.exit_direction(&self.direction) {
                self.positions.push(next_position);
                self.direction = exit_direction;
            } else {
                return Err(Error::CouldNotEnterNextPipe(next_pipe.to_char()));
            }
        }
    }
}

#[tracing::instrument]
fn find_walk(map: &PipeMap, start: &Position) -> Result<Walk> {
    let mut walk = None;

    for direction in &[
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ] {
        let next_position = start.move_in_direction(*direction);
        if let Some(next_pipe) = map.get(&next_position) {
            if next_pipe.exit_direction(direction).is_some() {
                walk = Some(Walk::new(vec![*start], *direction));
                break;
            }
        }
    }

    match walk {
        Some(mut walk) => {
            walk.follow_path(map)?;
            Ok(walk)
        }
        None => Err(Error::InvalidStart),
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Line {
    start: Position,
    end: Position,
}

impl Line {
    #[tracing::instrument]
    fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    #[tracing::instrument]
    fn from_points(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self {
            start: Position::new(x1, y1),
            end: Position::new(x2, y2),
        }
    }

    #[tracing::instrument]
    fn does_line_contain_point(&self, point: &Position) -> bool {
        let x1 = self.start.x;
        let y1 = self.start.y;
        let x2 = self.end.x;
        let y2 = self.end.y;

        let x = point.x;
        let y = point.y;

        let x_min = x1.min(x2);
        let x_max = x1.max(x2);
        let y_min = y1.min(y2);
        let y_max = y1.max(y2);

        x >= x_min && x <= x_max && y >= y_min && y <= y_max
    }

    #[tracing::instrument]
    fn direction(&self) -> Direction {
        self.start.direction_from_previous(&self.end)
    }
}

#[tracing::instrument]
fn walk_to_lines(walk: &Walk, map: &PipeMap) -> Vec<Line> {
    let mut lines = Vec::new();

    let mut previous_corner = walk.positions[0];

    for (i, position) in walk.positions.iter().enumerate() {
        if i == 0 {
            continue;
        }

        match map.get(position) {
            Some(Pipe::Start) => {
                break;
            }
            Some(pipe) if pipe.is_corner() => {
                lines.push(Line::from_points(
                    previous_corner.x,
                    previous_corner.y,
                    position.x,
                    position.y,
                ));

                previous_corner = *position;
            }
            _ => {}
        }
    }

    lines.push(Line::from_points(
        previous_corner.x,
        previous_corner.y,
        walk.positions[0].x,
        walk.positions[0].y,
    ));

    lines
}

#[tracing::instrument]
fn count_point_line_intersections_to_edge(lines: &Vec<Line>, point: &Position) -> u32 {
    let mut uncounted_lines = lines.clone();
    let mut lines_crossed = 0;

    for x in 0..=point.x {
        let position = Position::new(x, point.y);

        let crossed: Vec<Line> = uncounted_lines
            .iter()
            .filter(|line| line.does_line_contain_point(&position))
            .cloned()
            .collect();

        if position.y == 5 {
            println!(
                "{} {} crossed: {:?} t {}",
                position.x, position.y, crossed, lines_crossed
            );
        }

        if crossed.len() > 0 {
            lines_crossed += 1;
        }

        uncounted_lines.retain(|line| !line.does_line_contain_point(&position));
    }

    lines_crossed
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<u32> {
    let mut map = PipeMap::from_str(input)?;

    let walk = find_walk(&map, &map.start)?;

    for y in 0..map.height {
        for x in 0..map.width {
            let position = Position::new(x as i32, y as i32);

            if walk.positions.contains(&position) {
                continue;
            }

            map.set(&position, Pipe::Ground);
        }
    }

    println!("{}", map.to_string());

    let lines = walk_to_lines(&walk, &map);

    let lines = lines
        .iter()
        .filter(|line| line.direction() == Direction::Up || line.direction() == Direction::Down)
        .cloned()
        .collect_vec();

    let mut points_in_walk = 0;

    let mut dbg_str = String::new();

    for (y, row) in map.map.iter().enumerate() {
        for (x, pipe) in row.iter().enumerate() {
            let position = Position::new(x as i32, y as i32);

            if walk.positions.contains(&position) {
                let t = pipe.to_char().to_string().yellow();
                dbg_str = format!("{}{}", dbg_str, t); 
                continue;
            }

            let lines_crossed = count_point_line_intersections_to_edge(&lines, &position);

            dbg_str.push(lines_crossed.to_string().chars().next().unwrap());

            if lines_crossed % 2 == 1 {
                points_in_walk += 1;
            }
        }
        dbg_str.push('\n');
    }

    println!("{}", dbg_str);

    Ok(points_in_walk)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn it_should_make_lines() -> miette::Result<()> {
        let map = PipeMap::from_str(
            "...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........",
        )?;

        let walk = find_walk(&map, &map.start)?;

        let lines = walk_to_lines(&walk, &map);

        assert_eq!(
            lines[0],
            Line::new(Position::new(1, 1), Position::new(9, 1))
        );
        assert_eq!(
            lines[1],
            Line::new(Position::new(9, 1), Position::new(9, 7))
        );
        assert_eq!(
            lines[2],
            Line::new(Position::new(9, 7), Position::new(6, 7))
        );
        assert_eq!(
            lines[3],
            Line::new(Position::new(6, 7), Position::new(6, 5))
        );
        assert_eq!(
            lines[4],
            Line::new(Position::new(6, 5), Position::new(8, 5))
        );
        assert_eq!(
            lines[5],
            Line::new(Position::new(8, 5), Position::new(8, 2))
        );
        assert_eq!(
            lines[6],
            Line::new(Position::new(8, 2), Position::new(2, 2))
        );
        assert_eq!(
            lines[7],
            Line::new(Position::new(2, 2), Position::new(2, 5))
        );
        assert_eq!(
            lines[8],
            Line::new(Position::new(2, 5), Position::new(4, 5))
        );
        assert_eq!(
            lines[9],
            Line::new(Position::new(4, 5), Position::new(4, 7))
        );
        assert_eq!(
            lines[10],
            Line::new(Position::new(4, 7), Position::new(1, 7))
        );
        assert_eq!(
            lines[11],
            Line::new(Position::new(1, 7), Position::new(1, 1))
        );

        assert_eq!(12, lines.len());

        Ok(())
    }

    #[test]
    fn it_should_calculate_intersections() -> miette::Result<()> {
        assert_eq!(
            false,
            Line::from_points(0, 0, 10, 0).does_line_contain_point(&Position::new(0, 10))
        );

        assert_eq!(
            true,
            Line::new(Position::new(9, 1), Position::new(1, 1))
                .does_line_contain_point(&Position::new(5, 1))
        );

        // assert_eq!(
        //     false,
        //     Line::from_points(-2.0, 0.0, 1.0, 0.0)
        //         .does_intersect(&Line::from_points(1.0, 1.0, 1.0, 7.0))
        // );

        // assert_eq!(
        //     true,
        //     Line::from_points(0.0, -3.0, 0.0, 3.0)
        //         .does_intersect(&Line::from_points(-3.0, 0.0, 3.0, 0.0))
        // );

        // assert_eq!(
        //     true,
        //     Line::from_points(2.0, 2.0, 4.0, 2.0)
        //         .does_intersect(&Line::from_points(3.0, 2.0, 5.0, 2.0))
        // );

        Ok(())
    }

    // #[test]
    // fn test_process() -> miette::Result<()> {
    //     assert_eq!(
    //         process(
    //             "...........
    //             .S-------7.
    //             .|F-----7|.
    //             .||.....||.
    //             .||.....||.
    //             .|L-7.F-J|.
    //             .|..|.|..|.
    //             .L--J.L--J.
    //             ..........."
    //         )?,
    //         4,
    //     );

    //     // assert_eq!(
    //     //     4,
    //     //     process(
    //     //         "..........
    //     //         .S------7.
    //     //         .|F----7|.
    //     //         .||....||.
    //     //         .||....||.
    //     //         .|L-7F-J|.
    //     //         .|..||..|.
    //     //         .L--JL--J.
    //     //         .........."
    //     //     )?
    //     // );

    //     assert_eq!(
    //         8,
    //         process(
    //             ".F----7F7F7F7F-7....
    //             .|F--7||||||||FJ....
    //             .||.FJ||||||||L7....
    //             FJL7L7LJLJ||LJ.L-7..
    //             L--J.L7...LJS7F-7L7.
    //             ....F-J..F7FJ|L7L7L7
    //             ....L7.F7||L7|.L7L7|
    //             .....|FJLJ|FJ|F7|.LJ
    //             ....FJL-7.||.||||...
    //             ....L---J.LJ.LJLJ..."
    //         )?
    //     );

    //     // assert_eq!(
    //     //     10,
    //     //     process(
    //     //         ".FF7FSF7F7F7F7F7F---7
    //     //         L|LJ||||||||||||F--J
    //     //         FL-7LJLJ||||||LJL-77
    //     //         F--JF--7||LJLJ7F7FJ-
    //     //         L---JF-JLJ.||-FJLJJ7
    //     //         |F|F-JF---7F7-L7L|7|
    //     //         |FFJF7L7F-JF7|JL---7
    //     //         7-L-JL7||F7|L7F-7F7|
    //     //         L.L7LFJ|||||FJL7||LJ
    //     //         L7JLJL-JLJLJL--JLJ.L"
    //     //     )?
    //     // );
    //     Ok(())
    // }

    // #[test]
    // fn it_should_not_fail_on_data() -> miette::Result<()> {
    //     let input = include_str!("../input1.txt");

    //     assert_ne!(51, process(input)?);

    //     Ok(())
    // }
}
