use crate::{error::Error, prelude::*};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i64> {
    let mut floor = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {},
        }

        if floor < 0 {
            return Ok(i as i64 + 1);
        }
    }

    Ok(floor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_work_for_examples() -> miette::Result<()> {
        assert_eq!(1, process(")")?);
        assert_eq!(5, process("()())")?);
        Ok(())
    }

}