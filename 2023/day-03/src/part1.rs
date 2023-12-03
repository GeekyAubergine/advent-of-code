#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PartNumber {
    x: u32,
    y: u32,
    width: u32,
    number: u32,
}

impl PartNumber {
    #[tracing::instrument]
    fn new(x: u32, y: u32, width: u32, number: u32) -> Self {
        Self {
            x,
            y,
            width,
            number,
        }
    }

    #[tracing::instrument]
    fn has_adjacent_symbol(&self, symbol: &[Symbol]) -> bool {
        let start_x = if self.x == 0 { 0 } else { self.x - 1 };
        let end_x = self.x + self.width + 1;
        let start_y = if self.y == 0 { 0 } else { self.y - 1 };
        let end_y = self.y + 1;

        for x in start_x..end_x {
            for y in start_y..=end_y {
                if symbol.iter().any(|s| s.postition_equals(x, y)) {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Symbol {
    x: u32,
    y: u32,
    symbol: char,
}

impl Symbol {
    #[tracing::instrument]
    fn new(x: u32, y: u32, symbol: char) -> Self {
        Self { x, y, symbol }
    }

    #[tracing::instrument]
    fn postition_equals(&self, x: u32, y: u32) -> bool {
        self.x == x && self.y == y
    }
}

#[tracing::instrument]
fn extract_part_numbers_from_line(line: &str, line_index: u32) -> Vec<PartNumber> {
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
            let number = line.get(number_start..i).unwrap().parse::<u32>().unwrap();
            part_numbers.push(PartNumber::new(
                number_start as u32,
                line_index,
                i as u32 - number_start as u32,
                number,
            ));
        }
    }

    if in_digits {
        let number = line.get(number_start..).unwrap().parse::<u32>().unwrap();
        part_numbers.push(PartNumber::new(
            number_start as u32,
            line_index,
            line.len() as u32 - number_start as u32,
            number,
        ));
    }

    part_numbers
}

#[tracing::instrument]
fn extract_symbols_from_line(line: &str, line_index: u32) -> Vec<Symbol> {
    let mut symbols = Vec::new();

    for (i, c) in line.char_indices() {
        if !c.is_ascii_digit() && c != '.' {
            symbols.push(Symbol::new(i as u32, line_index, c));
        }
    }

    symbols
}

#[tracing::instrument]
fn part_numbers_adaject_to_a_symbol(part_numbers: &[PartNumber], symbols: &[Symbol]) -> Vec<u32> {
    part_numbers
        .iter()
        .filter(|part_number| part_number.has_adjacent_symbol(symbols))
        .map(|part_number| part_number.number)
        .collect::<Vec<_>>()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let part_numbers = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| extract_part_numbers_from_line(line.trim(), i as u32))
        .collect::<Vec<_>>();

    let symbols = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| extract_symbols_from_line(line.trim(), i as u32))
        .collect::<Vec<_>>();

    let parts_next_to_symbols = part_numbers_adaject_to_a_symbol(&part_numbers, &symbols);

    let sum = parts_next_to_symbols.iter().sum::<u32>();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_extract_part_numbers_from_line() -> miette::Result<()> {
        let input = "467..114..";
        let part_numbers = extract_part_numbers_from_line(input, 0);
        assert_eq!(
            vec![PartNumber::new(0, 0, 3, 467), PartNumber::new(5, 0, 3, 114)],
            part_numbers
        );

        Ok(())
    }

    #[test]
    fn it_should_extract_symbols_from_line() -> miette::Result<()> {
        let input = "617*......";
        let symbols = extract_symbols_from_line(input, 0);
        assert_eq!(vec![Symbol::new(3, 0, '*')], symbols);

        Ok(())
    }

    // This test was part of me debugging and is not finished, so it will fail, see my blog post about it https://zoeaubert.me/blog/advent-of-code-2023-day-03/
    // #[test]
    // fn it_should_extract_part_numbers_adjacent_to_symbol() -> miette::Result<()> {
    //     let input = include_str!("../input1.txt");

    //     // let input = input.lines().take(11).collect::<Vec<_>>().join("\n");

    //     let part_numbers = input
    //         .lines()
    //         .enumerate()
    //         .flat_map(|(i, line)| extract_part_numbers_from_line(line.trim(), i as u32))
    //         .collect::<Vec<_>>();

    //     let symbols = input
    //         .lines()
    //         .enumerate()
    //         .flat_map(|(i, line)| extract_symbols_from_line(line.trim(), i as u32))
    //         .collect::<Vec<_>>();

    //     let parts_next_to_symbols = part_numbers_adaject_to_a_symbol(&part_numbers, &symbols);

    //     let expect_part_numbers = vec![
    //         155, 944, 622, 31, 264, 532, 254, 528, //line 1
    //         111, 495, 558, //line 2
    //         791, 62, 618, 818, 642, 789, //line 3
    //         58, 405, 542, 587, 198, 846, 647, // line 4
    //         964, 474, 302, 786, 43, 505, 436, 51, //line 5
    //         832, 951, 984, 111, 198, 322, 186, 262, //line 6
    //         490, 690, 346, 702, 566, 192, 190, 87, //line 7
    //         816, 588, 152, 535, 425, 53, //line 8
    //         36, 290, 831, 374, 579, 536, 733, 169, 146, 179, 658, 260, // line 9
    //         795, 776, 790, 871, 281, // line 10
    //         78, 716, 400, 319, 167, 399, 599, // line 11
    //         719, 376, 800, 211, 478, 326, 93, 889, 684, 285, // line 12
    //         852, 462, 374, 603, 369, // line 13
    //         960, 966, 321, 925, 926, 947, // line 14
    //         479, 909, 339, 17, 284, 657, 587, // line 15,
    //         772, 345, 93, 465, 419, 676, 521, 399, 662, // line 16
    //         17, 2, 531, 79, 589, 198, 734, 534, 614, // line 17
    //         301, 321, 895, 344, 694, 717, 511, // line 18
    //         707, 370, 428, 509, 889, 353, // line 19
    //         973, 877, 855, 955, 670, 682, 150, 958, 197, 555, // line 20
    //         504, 352, 468, 688, 10, 306, // line 21
    //         987, 5, 811, 705, 462, 374, 42, // line 22
    //         402, 804, 295, 406, 150, 22, 429, 268, 324, // line 23
    //         270, 982, 644, 87, 505, //  line 24
    //         98, 370, 19, 867, 396, 272, 760, // line 25
    //         593, 793, 503, 34, 406, 456, 303, 142, 432, // line 26
    //         707, 563, 837, 230, 169, 138, 420, // line 27
    //         689, 503, 449, 39, 77, 404, // line 28
    //         137, 624, 883, 891, 310, 404, // line 29
    //         287, 961, 488, 544, 130, 531, 72, 424, 766, // line 30
    //         476, 722, 780, 613, 533, 96, 553, 91, 835, 690, // line 31
    //         350, 950, 359, 141, 326, 658, 832, // line 32
    //         772, 127, 335, 539, 101, 959, 221, 512, // line 33
    //         798, 138, 207, 999, 574, 484, 364, // line 34
    //         919, 202, 971, 488, 349, 404, 448, // line 35
    //         246, 211, 426, 206, 557, 27, 659, 588, 367, 961, 583, 280, // line 36
    //         724, 324, 788, 685, 788, 532, 85, 139, 75, 196, // line 37
    //         521, 391, 987, 810, 214, // line 38
    //         679, 776, 447, 457, 25, 467, 173, 241, // line 39
    //         43, 898, 412, 742, 540, 825, 259, 997, 514, // line 40
    //         775, 52, 809, 871, 384, 295, 470, 114, // line 41
    //         147, 69, 914, 144, 875, 278, 441, 859, 346, 281, 40, // line 42
    //         89, 578, 519, 676, 473, 361, // line 43
    //         78, 42, 750, 465, 218, 833, 137, 538, 962, 421, 502, 42, // line 44
    //         457, 825, 26, 238, 205, 539, 109, 348, 837, 842, // line 45
    //         175, 925, 399, 560, 636, // line 46
    //         693, 447, 137, 679, 479, 619, 283, 458, 544, 802, 848, // line 47
    //         39, 141, // line 48
    //         471, 502, 663, 986, 633, 530, 598, 220, 542, 568, 219, 532, 15,  // line 49
    //         840, // line 50
    //         351, 993, 573, 865, 848, 239, 134, 64, 231, // line 51
    //         809, 925, 43, 277, 571, // line 52
    //         698, 355, 55, 847, 409, 78, 363, // line 53
    //         261, 591, 695, 678, 714, 364, 804, 156, 605, // line 54
    //         192, 957, 963, 447, 344, // line 55
    //         524, 568, 691, 169, 218, 10, 10, 399, 46, 488, 491, 16, // line 56
    //         824, 772, 265, 964, // line 57
    //         345, 161, 671, 414, 726, 564, // line 58
    //         155, 483, 546, 968, 591, // line 59
    //         806, 120, 813, 481, 593, 667, 815, 682, 579, 298, 668, 188, // line 60
    //         718, 469, 251, 52, 919, 846, 887, 637, // line 61
    //         81, 51, 236, 167, 338, 963, 258, 980, 816, // line 62
    //         150, 316, 389, 590, 291, 143, 284, // line 63
    //         390, 559, 116, 926, 779, // line 64
    //         500, 821, 594, 220, 830, 89, 915, 363, // line 65
    //         623, 337, 40, 827, 828, 294, 392, // line 66
    //         993, 565, 638, 307, 95, 535, 105, 632, 938, 116, 939, // line 67
    //         444, 378, 283, 971, 689, 937, 736,
    //         991, // line 68

    //              // 608, 362, 642, 262, 617, // line 140
    //     ];

    //     assert_eq!(expect_part_numbers, parts_next_to_symbols);
    //     Ok(())
    // }

    #[test]
    fn it_should_parse_lines_66_to_68() -> miette::Result<()> {
        let input = "
        ...*...623....337.......................40..........827..............*................828....$294....392....*....*.....%..............*.....
        .993............*....565........................638...............307.............95.......#..............535.105.........632..938.166..$939
        .....$..444@...378...*.......4...283...971@.......*...................689..937...*.......736......@...................991..@....*...........";

        let part_numbers = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| extract_part_numbers_from_line(line.trim(), i as u32))
            .collect::<Vec<_>>();

        let expected_part_numbers: Vec<u32> = vec![
            623, 337, 40, 827, 828, 294, 392, // line 66
            993, 565, 638, 307, 95, 535, 105, 632, 938, 166, 939, // line 67
            444, 378, 4, 283, 971, 689, 937, 736, 991, // line 68
        ];

        assert_eq!(expected_part_numbers, part_numbers.iter().map(|p| p.number).collect::<Vec<_>>());

        let symbols = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| extract_symbols_from_line(line.trim(), i as u32))
            .collect::<Vec<_>>();

        let parts_next_to_symbols = part_numbers_adaject_to_a_symbol(&part_numbers, &symbols);

        // There's some numbers missing from 66 and 68, that's because this is a slice so the row above doesn't trigger them
        let expect_part_numbers = vec![
            337, 294, // line 66
            993, 565, 638, 307, 95, 535, 105, 632, 938, 166, 939, // line 67
            444, 378, 971, 736 // line 68
        ];

        assert_eq!(expect_part_numbers, parts_next_to_symbols);
        Ok(())
    }

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
}
