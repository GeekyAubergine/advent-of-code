#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PartNumber {
    x: i32,
    y: i32,
    width: i32,
    number: i32,
}

impl PartNumber {
    #[tracing::instrument]
    fn new(x: i32, y: i32, width: i32, number: i32) -> Self {
        Self {
            x,
            y,
            width,
            number,
        }
    }

    #[tracing::instrument]
    fn contains_point(&self, x: i32, y: i32) -> bool {
        let start_x = self.x;
        let end_x = self.x + self.width;

        x >= start_x && x < end_x && y == self.y
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Symbol {
    x: i32,
    y: i32,
    symbol: char,
}

impl Symbol {
    #[tracing::instrument]
    fn new(x: i32, y: i32, symbol: char) -> Self {
        Self { x, y, symbol }
    }

    #[tracing::instrument]
    fn adjacent_part_numbers(&self, part_numbers: &[PartNumber]) -> Vec<i32> {
        part_numbers
            .iter()
            .filter(|part_number| {
                part_number.contains_point(self.x - 1, self.y) // left
                    || part_number.contains_point(self.x + 1, self.y) // right
                    || part_number.contains_point(self.x, self.y - 1) // top
                    || part_number.contains_point(self.x, self.y + 1) // bottom
                    || part_number.contains_point(self.x - 1, self.y - 1) // top left
                    || part_number.contains_point(self.x + 1, self.y - 1) // top right
                    || part_number.contains_point(self.x - 1, self.y + 1) // bottom left
                    || part_number.contains_point(self.x + 1, self.y + 1) // bottom right
            })
            .map(|part_number| part_number.number)
            .collect::<Vec<_>>()
    }
}

#[tracing::instrument]
fn extract_part_numbers_from_line(line: &str, line_index: i32) -> Vec<PartNumber> {
    let mut part_numbers = Vec::new();

    let mut in_digits = false;
    let mut number_start = 0;

    for (i, c) in line.char_indices() {
        if c.is_ascii_digit() {
            if !in_digits {
                in_digits = true;
                number_start = i;
            }
        } else if in_digits {
            in_digits = false;
            let number = line.get(number_start..i).unwrap().parse::<i32>().unwrap();
            part_numbers.push(PartNumber::new(
                number_start as i32,
                line_index,
                i as i32 - number_start as i32,
                number,
            ));
        }
    }

    if in_digits {
        let number = line.get(number_start..).unwrap().parse::<i32>().unwrap();
        part_numbers.push(PartNumber::new(
            number_start as i32,
            line_index,
            line.len() as i32 - number_start as i32,
            number,
        ));
    }

    part_numbers
}

#[tracing::instrument]
fn extract_symbols_from_line(line: &str, line_index: i32) -> Vec<Symbol> {
    let mut symbols = Vec::new();

    for (i, c) in line.char_indices() {
        if c == '*' {
            symbols.push(Symbol::new(i as i32, line_index, c));
        }
    }

    symbols
}

#[tracing::instrument]
fn symbols_with_2_adjacent_part_numbers(
    symbols: &[Symbol],
    part_numbers: &[PartNumber],
) -> Vec<i32> {
    symbols
        .iter()
        .map(|symbol| symbol.adjacent_part_numbers(part_numbers))
        .filter(|adjacent_part_numbers| adjacent_part_numbers.len() == 2)
        .map(|adjacent_part_numbers| adjacent_part_numbers.iter().product())
        .collect::<Vec<_>>()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i32> {
    let part_numbers = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| extract_part_numbers_from_line(line.trim(), i as i32))
        .collect::<Vec<_>>();

    let symbols = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| extract_symbols_from_line(line.trim(), i as i32))
        .collect::<Vec<_>>();

    let gear_ratios = symbols_with_2_adjacent_part_numbers(&symbols, &part_numbers);

    let sum = gear_ratios.iter().sum::<i32>();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
        assert_eq!(467835, process(input)?);
        Ok(())
    }
}
