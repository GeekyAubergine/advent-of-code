use crate::{error::Error, prelude::*};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i64> {
    let opens = input.chars().filter(|c| c == &'(').count();
    let closes = input.chars().filter(|c| c == &')').count();

    Ok(opens as i64 - closes as i64)
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
