use gcd::*;
use rayon::prelude::*;
use std::collections::HashMap;

use crate::{error::Error, prelude::*};

const Z: u32 = 0x0000005A;
const A: u32 = 0x00000041;

#[tracing::instrument]
fn letters_to_id(letters: &str) -> Result<u32> {
    if letters.len() != 3 {
        return Err(Error::InvalidNumberOfLettersForId(letters.to_string()));
    }

    let mut id: u32 = 0;

    for (i, letter) in letters.chars().rev().enumerate() {
        id |= (letter as u32) << (i * 8);
    }

    Ok(id)
}

#[tracing::instrument]
fn id_to_letters(id: u32) -> String {
    let mut letters = String::new();

    let letter_1 = ((id & 0x00FF0000) >> 16) as u8 as char;
    let letter_2 = ((id & 0x0000FF00) >> 8) as u8 as char;
    let letter_3 = (id & 0x000000FF) as u8 as char;

    letters.push(letter_1);
    letters.push(letter_2);
    letters.push(letter_3);

    letters
}

#[tracing::instrument]
fn id_ends_with_z(id: u32) -> bool {
    id & 0x000000FF == Z
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    id: u32,
    left: u32,
    right: u32,
}

impl Node {
    #[tracing::instrument]
    fn new(id: u32, left: u32, right: u32) -> Self {
        Self { id, left, right }
    }

    #[tracing::instrument]
    fn from_str(input: &str) -> Result<Self> {
        let id = input
            .get(0..=2)
            .ok_or_else(|| Error::CouldNotFindIdForInstruction(input.to_string()))?;

        let left = input
            .get(7..=9)
            .ok_or_else(|| Error::CouldNotFindLeftInstruction(input.to_string()))?;

        let right = input
            .get(12..=14)
            .ok_or_else(|| Error::CouldNotFindRightInstruction(input.to_string()))?;

        Ok(Self::new(
            letters_to_id(id)?,
            letters_to_id(left)?,
            letters_to_id(right)?,
        ))
    }

    #[tracing::instrument]
    fn ends_with_a(&self) -> bool {
        self.id & 0x000000FF == A
    }

    #[tracing::instrument]
    fn ends_with_z(&self) -> bool {
        self.id & 0x000000FF == Z
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    nodes: HashMap<u32, Node>,
}

impl Map {
    #[tracing::instrument]
    fn new(nodes: Vec<Node>) -> Self {
        let mut map = Self {
            nodes: HashMap::new(),
        };

        for node in nodes {
            map.nodes.insert(node.id, node);
        }

        map
    }

    #[tracing::instrument]
    fn from_str(input: &str) -> Result<Self> {
        let mut nodes = Vec::new();

        for line in input.lines() {
            nodes.push(Node::from_str(line)?);
        }

        Ok(Self::new(nodes))
    }

    #[tracing::instrument]
    fn get_node(&self, id: u32) -> Result<&Node> {
        self.nodes
            .get(&id)
            .ok_or_else(|| Error::CouldNotInspectionForId(id_to_letters(id)))
    }

    #[tracing::instrument]
    fn get_starting_nodes(&self) -> Vec<&Node> {
        self.nodes
            .values()
            .filter(|n| n.ends_with_a())
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    input: String,
    cursor: usize,
}

impl Input {
    #[tracing::instrument]
    fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            cursor: 0,
        }
    }

    #[tracing::instrument]
    fn skip(&mut self, n: usize) {
        self.cursor += n;
        self.cursor %= self.input.len();
    }

    #[tracing::instrument]
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.cursor)
    }

    #[tracing::instrument]
    fn next(&mut self) -> Option<char> {
        let next = self.peek();
        self.cursor += 1;
        self.cursor %= self.input.len();
        next
    }
}

#[tracing::instrument]
fn get_next_node(map: &Map, node: u32, mut input: Input) -> Result<(u32, Input)> {
    let node = map.get_node(node)?;

    match input.next() {
        Some('L') => Ok((map.get_node(node.left)?.id, input)),
        Some('R') => Ok((map.get_node(node.right)?.id, input)),
        Some(c) => Err(Error::UnexpectedInstruction(c.to_string())),
        None => Err(Error::UnexpectedEndOfInstructions),
    }
}

#[tracing::instrument]
fn steps_to_next_ending_in_z(map: &Map, node: u32, mut input: Input) -> Result<u64> {
    let mut steps = 0;
    let mut current_node = node;

    loop {
        if id_ends_with_z(current_node) {
            return Ok(steps);
        }

        let (next_node, next_input) = get_next_node(map, current_node, input)?;

        steps += 1;

        current_node = next_node;
        input = next_input;
    }
}

#[tracing::instrument]
fn lcm(numbers: &[u64]) -> u64 {
    let mut result = numbers[0];

    for &number in numbers.iter().skip(1) {
        result = result * number / result.gcd(number);
    }

    result
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<u64> {
    let mut lines = input.lines().map(|l| l.trim());

    let instructions = lines.next().ok_or_else(|| Error::NoInstructionsFound)?;

    let input = Input::new(instructions);

    lines.next();

    let remaining = lines.collect::<Vec<_>>().join("\n");

    let map = Map::from_str(&remaining)?;

    let current_nodes = map
        .get_starting_nodes()
        .iter()
        .map(|n| n.id)
        .collect::<Vec<_>>();

    let distances_to_next_z = current_nodes
        .par_iter()
        .map(|n| steps_to_next_ending_in_z(&map, *n, input.clone()))
        .collect::<Result<Vec<_>>>()?;

    dbg!(&distances_to_next_z);

    let lcm: u64 = lcm(&distances_to_next_z);

    Ok(lcm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_encode_the_id_correctly() -> miette::Result<()> {
        let input = "ABC";

        let expected = 0x00414243;
        let actual = letters_to_id(input)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn it_should_decode_the_id_correctly() -> miette::Result<()> {
        let input = 0x00414243;

        let expected = "ABC";
        let actual = id_to_letters(input);
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn it_should_parse_node() -> miette::Result<()> {
        let input = "AAA = (BBB, CCC)";

        let expected = Node::new(0x00414141, 0x00424242, 0x00434343);
        let actual = Node::from_str(input)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn it_should_find_distance_to_next_end() -> miette::Result<()> {
        let input = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

        let mut lines = input.lines().map(|l| l.trim());

        let instructions = lines.next().ok_or_else(|| Error::NoInstructionsFound)?;

        let instructions = Input::new(instructions);

        lines.next();

        let remaining = lines.collect::<Vec<_>>().join("\n");

        let map = Map::from_str(&remaining)?;

        assert_eq!(
            2,
            steps_to_next_ending_in_z(&map, letters_to_id("11A")?, instructions.clone())?
        );
        assert_eq!(
            3,
            steps_to_next_ending_in_z(&map, letters_to_id("22A")?, instructions.clone())?
        );

        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

        assert_eq!(6, process(input)?);
        Ok(())
    }
}
