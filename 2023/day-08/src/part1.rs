use std::collections::HashMap;

use crate::{error::Error, prelude::*};

const ZZZ_ID: u32 = 0x005A5A5A;

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
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<u32> {
    let mut lines = input.lines().map(|l| l.trim());

    let instructions = lines.next().ok_or_else(|| Error::NoInstructionsFound)?;

    lines.next();

    let remaining = lines.collect::<Vec<_>>().join("\n");

    let map = Map::from_str(&remaining)?;

    let mut steps = 0;
    let mut current_node = map.get_node(letters_to_id("AAA")?)?;

    loop {
        for direction in instructions.chars() {
            if current_node.id == ZZZ_ID {
                return Ok(steps);
            }

            match direction {
                'L' => current_node = map.get_node(current_node.left)?,
                'R' => current_node = map.get_node(current_node.right)?,
                _ => return Err(Error::UnexpectedInstruction(direction.to_string())),
            }

            steps += 1;
        }
    }
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
    fn it_should_work_for_example_a() -> miette::Result<()> {
        let input = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";

        assert_eq!(2, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";

        assert_eq!(6, process(input)?);
        Ok(())
    }
}
