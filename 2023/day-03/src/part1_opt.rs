use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Data {
    is_symbol: Vec<bool>,
    width: usize,
}

impl Data {
    #[tracing::instrument]
    fn new(input: &String) -> Self {
        let is_symbol = input
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| is_symbol(Some(c)))
            .collect::<Vec<_>>();
        let width = input.lines().next().unwrap().len();

        Self { width, is_symbol }
    }

    #[tracing::instrument]
    fn is_symbol(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            return false;
        }

        match self.is_symbol.get(y as usize * self.width + x as usize) {
            Some(v) => *v,
            None => false,
        }
    }
}

#[tracing::instrument]
fn is_symbol(char: Option<char>) -> bool {
    match char {
        Some(c) => {
            matches!(c, '-' | '%' | '+' | '=' | '*' | '/' | '$' | '#' | '&' | '@')
        }
        None => false,
    }
}

#[tracing::instrument]
fn parse_line(line: &str, y: i32, data: &Data) -> Vec<u32> {
    let mut in_number = false;
    let mut number_start = 0;
    let mut adjacent_symbol = false;

    let mut numbers = vec![];

    for (i, c) in line.chars().enumerate() {
        let i_as_i32 = i as i32;
        if c.is_ascii_digit() {
            if !in_number {
                in_number = true;
                number_start = i;

                // Previous
                if data.is_symbol(i_as_i32 - 1, y)
                    || data.is_symbol(i_as_i32 - 1, y - 1)
                    || data.is_symbol(i_as_i32 - 1, y + 1)
                {
                    adjacent_symbol = true;
                }
            }

            // Above below
            if (data.is_symbol(i_as_i32, y - 1)) || (data.is_symbol(i_as_i32, y + 1)) {
                adjacent_symbol = true;
            }
        } else if in_number {
            // Check self, above and below
            if data.is_symbol(i_as_i32, y)
                || data.is_symbol(i_as_i32, y - 1)
                || data.is_symbol(i_as_i32, y + 1)
            {
                adjacent_symbol = true;
            }

            if adjacent_symbol {
                numbers.push(line[number_start..i].parse().unwrap());
            }

            in_number = false;
            adjacent_symbol = false;
        }
    }

    if in_number
        && (adjacent_symbol
            || data.is_symbol(line.len() as i32 - 1, y - 1)
            || data.is_symbol(line.len() as i32 - 1, y + 1))
    {
        numbers.push(line[number_start..].parse().unwrap());
    }

    numbers
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let input = input
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n");

    let data = Data::new(&input);

    let sum = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| parse_line(line, y as i32, &data))
        .sum::<u32>();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        assert_eq!(4361, process(input)?);
        Ok(())
    }

    #[test]
    fn test_full() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!(528819, process(input)?);
        Ok(())
    }
}
