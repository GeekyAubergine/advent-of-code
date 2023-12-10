use std::collections::VecDeque;

use crate::{error::Error, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Vertical,        // |
    Horizontal,      // -
    CornerDownRight, // L
    CornerDownLeft,  // J
    CornerUpLeft,    // 7
    CornerUpRight,   // F
    Ground,          // .
    Start,           // S
}

impl TryFrom<char> for Pipe {
    type Error = Error;

    #[tracing::instrument]
    fn try_from(c: char) -> Result<Self> {
        match c {
            '|' => Ok(Pipe::Vertical),
            '-' => Ok(Pipe::Horizontal),
            'L' => Ok(Pipe::CornerDownRight),
            'J' => Ok(Pipe::CornerDownLeft),
            'F' => Ok(Pipe::CornerUpRight),
            '7' => Ok(Pipe::CornerUpLeft),
            '.' => Ok(Pipe::Ground),
            'S' => Ok(Pipe::Start),
            _ => Err(Error::UnknownPipe(c)),
        }
    }
}

impl Pipe {
    #[tracing::instrument]
    fn can_enter_pipe(&self, direction: &Direction) -> bool {
        match self {
            Pipe::Vertical => match direction {
                Direction::Up => true,
                Direction::Down => true,
                Direction::Right => false,
                Direction::Left => false,
            },
            Pipe::Horizontal => match direction {
                Direction::Up => false,
                Direction::Down => false,
                Direction::Right => true,
                Direction::Left => true,
            },
            // L
            Pipe::CornerDownRight => match direction {
                Direction::Up => false,
                Direction::Down => true,
                Direction::Right => false,
                Direction::Left => true,
            },
            // J
            Pipe::CornerDownLeft => match direction {
                Direction::Up => false,
                Direction::Down => true,
                Direction::Right => true,
                Direction::Left => false,
            },
            // F
            Pipe::CornerUpRight => match direction {
                Direction::Up => true,
                Direction::Down => false,
                Direction::Right => false,
                Direction::Left => true,
            },
            // 7
            Pipe::CornerUpLeft => match direction {
                Direction::Up => true,
                Direction::Down => false,
                Direction::Right => true,
                Direction::Left => false,
            },
            Pipe::Ground => match direction {
                Direction::Up => false,
                Direction::Down => false,
                Direction::Right => false,
                Direction::Left => false,
            },
            Pipe::Start => match direction {
                Direction::Up => false,
                Direction::Down => false,
                Direction::Right => false,
                Direction::Left => false,
            },
        }
    }

    #[tracing::instrument]
    fn to_char(&self) -> char {
        match self {
            Pipe::Vertical => '|',
            Pipe::Horizontal => '-',
            Pipe::CornerDownRight => 'L',
            Pipe::CornerDownLeft => 'J',
            Pipe::CornerUpRight => 'F',
            Pipe::CornerUpLeft => '7',
            Pipe::Ground => '.',
            Pipe::Start => 'S',
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PipeMap {
    map: Vec<Vec<Pipe>>,
    start: Position,
}

impl PipeMap {
    fn new(map: Vec<Vec<Pipe>>, start: Position) -> Self {
        Self { map, start }
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

        // dbg!(start);

        let mut pipes = Self { map, start };

        pipes.clean_non_doubly_connected_pipes();

        Ok(pipes)
    }

    #[tracing::instrument]
    fn clean_non_doubly_connected_pipes(&mut self) {
        let mut some_pipes_with_only_one_connection = true;

        while some_pipes_with_only_one_connection {
            some_pipes_with_only_one_connection = false;

            for y in 0..self.map.len() {
                for x in 0..self.map[y].len() {
                    let position = Position::new(x as i32, y as i32);
                    if let Some(pipe) = self.get(&position) {
                        let mut connections = 0;
                        for direction in &[
                            Direction::Up,
                            Direction::Down,
                            Direction::Right,
                            Direction::Left,
                        ] {
                            let next_position = position.move_in_direction(*direction);
                            if let Some(next_pipe) = self.get(&next_position) {
                                if next_pipe.can_enter_pipe(&direction) {
                                    connections += 1;
                                }
                            }
                        }
                        if connections == 1 {
                            self.map[y][x] = Pipe::Ground;
                            some_pipes_with_only_one_connection = true;
                        }
                    }
                }
            }
        }
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum WalkResult {
    DeadEnd,
    Start { distance: u32 },
}

#[tracing::instrument]
fn walk(map: &PipeMap, position: &Position, direction: &Direction, distance: u32) -> WalkResult {
    let pipe = map.get(position);

    match pipe {
        None => return WalkResult::DeadEnd,
        Some(pipe) => {
            if pipe == &Pipe::Ground {
                return WalkResult::DeadEnd;
            }

            if pipe == &Pipe::Start {
                return WalkResult::Start { distance };
            }

            if !pipe.can_enter_pipe(&direction) {
                return WalkResult::DeadEnd;
            }

            let next_position = position.move_in_direction(direction);

            for next_direction in vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let next_result = walk(map, &next_position, &next_direction, distance + 1);

                match next_result {
                    WalkResult::DeadEnd => {}
                    WalkResult::Start { distance } => {
                        next_distances.push(distance);
                    }
                }
            }

            if next_distances.is_empty() {
                return WalkResult::DeadEnd;
            }

            let mut max_distance = 0;

            for next_distance in next_distances {
                if next_distance > max_distance {
                    max_distance = next_distance;
                }
            }

            WalkResult::Start {
                distance: max_distance,
            }
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let map = PipeMap::from_str(input)?;

    println!("{}", map.distances_to_string());

    Ok(map.greatest_distance_on_main_loop())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn it_should_return_correct_pipe_entry() -> miette::Result<()> {
        assert_eq!(true, Pipe::Vertical.can_enter_pipe(&Direction::Up));
        assert_eq!(true, Pipe::Vertical.can_enter_pipe(&Direction::Down));
        assert_eq!(false, Pipe::Vertical.can_enter_pipe(&Direction::Right));
        assert_eq!(false, Pipe::Vertical.can_enter_pipe(&Direction::Left));

        assert_eq!(false, Pipe::Horizontal.can_enter_pipe(&Direction::Up));
        assert_eq!(false, Pipe::Horizontal.can_enter_pipe(&Direction::Down));
        assert_eq!(true, Pipe::Horizontal.can_enter_pipe(&Direction::Right));
        assert_eq!(true, Pipe::Horizontal.can_enter_pipe(&Direction::Left));

        // L
        assert_eq!(false, Pipe::CornerDownRight.can_enter_pipe(&Direction::Up));
        assert_eq!(true, Pipe::CornerDownRight.can_enter_pipe(&Direction::Down));
        assert_eq!(
            false,
            Pipe::CornerDownRight.can_enter_pipe(&Direction::Right)
        );
        assert_eq!(true, Pipe::CornerDownRight.can_enter_pipe(&Direction::Left));

        // J
        assert_eq!(false, Pipe::CornerDownLeft.can_enter_pipe(&Direction::Up));
        assert_eq!(true, Pipe::CornerDownLeft.can_enter_pipe(&Direction::Down));
        assert_eq!(false, Pipe::CornerDownLeft.can_enter_pipe(&Direction::Left));
        assert_eq!(true, Pipe::CornerDownLeft.can_enter_pipe(&Direction::Right));

        // F
        assert_eq!(true, Pipe::CornerUpRight.can_enter_pipe(&Direction::Up));
        assert_eq!(false, Pipe::CornerUpRight.can_enter_pipe(&Direction::Down));
        assert_eq!(true, Pipe::CornerUpRight.can_enter_pipe(&Direction::Left));
        assert_eq!(false, Pipe::CornerUpRight.can_enter_pipe(&Direction::Right));

        // 7
        assert_eq!(true, Pipe::CornerUpLeft.can_enter_pipe(&Direction::Up));
        assert_eq!(false, Pipe::CornerUpLeft.can_enter_pipe(&Direction::Down));
        assert_eq!(false, Pipe::CornerUpLeft.can_enter_pipe(&Direction::Left));
        assert_eq!(true, Pipe::CornerUpLeft.can_enter_pipe(&Direction::Right));

        Ok(())
    }

    #[test]
    fn it_should_parse_a_pipe_map() -> miette::Result<()> {
        let input = ".....
        .S-7.
        .|.|.
        .L-J.
        .....";

        let expected = PipeMap::new(
            vec![
                vec![
                    Pipe::Ground,
                    Pipe::Ground,
                    Pipe::Ground,
                    Pipe::Ground,
                    Pipe::Ground,
                ],
                vec![
                    Pipe::Ground,
                    Pipe::Start,
                    Pipe::Horizontal,
                    Pipe::CornerUpLeft,
                    Pipe::Ground,
                ],
                vec![
                    Pipe::Ground,
                    Pipe::Vertical,
                    Pipe::Ground,
                    Pipe::Vertical,
                    Pipe::Ground,
                ],
                vec![
                    Pipe::Ground,
                    Pipe::CornerDownRight,
                    Pipe::Horizontal,
                    Pipe::CornerDownLeft,
                    Pipe::Ground,
                ],
                vec![
                    Pipe::Ground,
                    Pipe::Ground,
                    Pipe::Ground,
                    Pipe::Ground,
                    Pipe::Ground,
                ],
            ],
            Position::new(1, 1),
        );

        assert_eq!(expected, PipeMap::from_str(input)?);

        Ok(())
    }

    // #[test]
    // fn it_should_clean_map_of_non_loop_pipes() -> miette::Result<()> {
    //     let busy_map = PipeMap::from_str(
    //         "-L|F7
    //         7S-7|
    //         L|7||
    //         -L-J|
    //         L|-JF",
    //     )?;

    //     println!("{}", busy_map.to_string());

    //     let minimum_map = PipeMap::from_str(
    //         ".....
    //         .S-7.
    //         .|.|.
    //         .L-J.
    //         ......",
    //     )?;

    //     assert_eq!(minimum_map, busy_map);

    //     Ok(())
    // }

    #[test]
    fn it_should_build_map_example_a() -> miette::Result<()> {
        let input = ".....
        .S-7.
        .|.|.
        .L-J.
        .....";

        let map = Map::new(PipeMap::from_str(input)?);

        // assert_eq!(expected, Map::new(PipeMap::from_str(input)?));

        let expected = vec![
            vec![None, None, None, None, None],
            vec![None, Some(0), Some(1), Some(2), None],
            vec![None, Some(1), None, Some(3), None],
            vec![None, Some(2), Some(3), Some(4), None],
            vec![None, None, None, None, None],
        ];

        assert_eq!(expected, map.distances_from_start);

        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!(
            4,
            process(
                ".....
                .S-7.
                .|.|.
                .L-J.
                ....."
            )?
        );

        assert_eq!(
            4,
            process(
                "-L|F7
                7S-7|
                L|7||
                -L-J|
                L|-JF"
            )?
        );

        assert_eq!(
            8,
            process(
                "..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ..."
            )?
        );
        Ok(())
    }

    // #[test]
    // fn it_should_not_fail_on_data() -> miette::Result<()> {
    //     let input = include_str!("../input1.txt");

    //     assert_ne!(51, process(input)?);

    //     Ok(())
    // }
}
