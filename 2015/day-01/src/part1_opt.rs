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
    }

    Ok(floor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_work_for_examples() -> miette::Result<()> {
        assert_eq!(0, process("(())")?);
        assert_eq!(0, process("()()")?);
        assert_eq!(3, process("(((")?);
        assert_eq!(3, process("(()(()(")?);
        assert_eq!(3, process("))(((((")?);
        assert_eq!(-1, process("())")?);
        assert_eq!(-1, process("))(")?);
        assert_eq!(-3, process(")))")?);
        assert_eq!(-3, process(")())())")?);
        Ok(())
    }

}
