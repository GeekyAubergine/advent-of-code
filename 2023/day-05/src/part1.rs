use crate::{prelude::*, error::Error};

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParserOutput<'a, T> {
    output: T,
    rest: &'a str,
}

struct Seeds {
    seeds: Vec<u32>,
}

impl Seeds {
    #[tracing::instrument]
    fn from_str(input: &str) -> Result<ParserOutput<Seeds>> {
        let first_line = input
            .lines()
            .next()
            .ok_or_else(|| Error::CannotFindSeedsHeader(input.to_string()))?;

        if !first_line.starts_with("seeds:") {
            return Err(Error::CannotFindSeedsHeader(first_line.to_string()));
        }

        let seeds = first_line
            .split(':')
            .last()
            .ok_or_else(|| Error::CannotFindSeedsHeader(first_line.to_string()))?
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.trim()
                    .parse::<u32>()
                    .map_err(|_| Error::CouldNotParseSeed(s.to_string()))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(ParserOutput {
            output: Seeds { seeds },
            rest: input,
        })
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<u32> {
    todo!("day 01 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_parse_seed() -> miette::Result<()> {
        let input = "seeds: 79 14 55 13";
        let output = Seeds::from_str(input)?;
        assert_eq!(vec![79, 14, 55, 13], output.output.seeds);

        Ok(())
    }

    // #[test]
    // fn test_process() -> miette::Result<()> {
    //     let input = include_str!("../example1.txt");
    //     assert_eq!(35, process(input)?);
    //     Ok(())
    // }
}
