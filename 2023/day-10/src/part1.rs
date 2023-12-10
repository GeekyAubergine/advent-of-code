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

#[derive(Debug, Clone, PartialEq, Eq)]
struct PipeMap {
    map: Vec<Vec<Pipe>>,
    start: Position,
}

impl PipeMap {
    #[tracing::instrument]
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

        Ok(Self { map, start })
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
fn find_loop(map: &PipeMap, start: &Position) -> Result<u32> {
    let mut walk = None;

    for direction in &[
        Direction::Up,
        Direction::Down,
        Direction::Right,
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
            Ok((walk.positions.len() as u32) / 2)
        }
        None => Err(Error::InvalidStart),
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<u32> {
    let map = PipeMap::from_str(input)?;

    find_loop(&map, &map.start)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn it_should_give_correct_exit_direction_for_pipe_based_on_entry_direction(
    ) -> miette::Result<()> {
        assert_eq!(
            Pipe::Vertical.exit_direction(&Direction::Up),
            Some(Direction::Up),
        );
        assert_eq!(
            Pipe::Vertical.exit_direction(&Direction::Down),
            Some(Direction::Down),
        );

        assert_eq!(
            Pipe::Horizontal.exit_direction(&Direction::Right),
            Some(Direction::Right),
        );
        assert_eq!(
            Pipe::Horizontal.exit_direction(&Direction::Left),
            Some(Direction::Left),
        );

        // L
        assert_eq!(
            Pipe::L.exit_direction(&Direction::Down),
            Some(Direction::Right),
        );
        assert_eq!(
            Pipe::L.exit_direction(&Direction::Left),
            Some(Direction::Up),
        );

        // J
        assert_eq!(
            Pipe::J.exit_direction(&Direction::Down),
            Some(Direction::Left),
        );
        assert_eq!(
            Pipe::J.exit_direction(&Direction::Right),
            Some(Direction::Up),
        );

        // F
        assert_eq!(
            Pipe::F.exit_direction(&Direction::Up),
            Some(Direction::Right),
        );
        assert_eq!(
            Pipe::F.exit_direction(&Direction::Left),
            Some(Direction::Down),
        );

        // 7
        assert_eq!(
            Pipe::Seven.exit_direction(&Direction::Up),
            Some(Direction::Left),
        );
        assert_eq!(
            Pipe::Seven.exit_direction(&Direction::Right),
            Some(Direction::Down),
        );
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
                    Pipe::Seven,
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
                    Pipe::L,
                    Pipe::Horizontal,
                    Pipe::J,
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

        let map = PipeMap::from_str(input)?;

        assert_eq!(map, expected);

        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!(
            process(
                ".....
                .S-7.
                .|.|.
                .L-J.
                ....."
            )?,
            4,
        );

        assert_eq!(
            process(
                "-L|F7
                7S-7|
                L|7||
                -L-J|
                L|-JF"
            )?,
            4,
        );

        assert_eq!(
            process(
                "..F7.
                .FJ|.
                SJ.L7
                |F--J
                LJ..."
            )?,
            8,
        );
        Ok(())
    }

    #[test]
    fn it_should_not_fail_on_data() -> miette::Result<()> {
        let input = include_str!("../input1.txt");

        assert_ne!( process(input)?, 51);

        Ok(())
    }
}
