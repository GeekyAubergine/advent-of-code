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
                    println!("empty row");
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

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    todo!("day 01 - part 1");
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

    // #[test]
    // fn test_process() -> miette::Result<()> {
    //     todo!("haven't built test yet");
    //     let input = "";
    //     assert_eq!("", process(input)?);
    //     Ok(())
    // }
}
