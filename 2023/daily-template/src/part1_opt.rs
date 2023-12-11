use crate::prelude::*;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String> {
    todo!("day 01 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
