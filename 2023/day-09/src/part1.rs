use crate::{error::Error, prelude::*};

#[tracing::instrument]
fn calculate_differences(values: &[i32]) -> Vec<i32> {
    let mut differences = Vec::new();
    for i in 0..values.len() - 1 {
        differences.push(values[i + 1] - values[i]);
    }
    differences
}

#[tracing::instrument]
fn extrapolate_value(input: &[i32]) -> Result<i32> {
    let mut values = vec![input.to_vec()];

    loop {
        let bottom = values
            .last()
            .ok_or_else(|| Error::CouldNotGetBottomRowOfValues)?;

        if bottom.iter().all(|n| *n == 0) {
            break;
        }

        values.push(calculate_differences(bottom));
    }

    for row_index in (0..values.len() - 1).rev() {
        let row_last_value = values[row_index]
            .last()
            .ok_or_else(|| Error::CouldNotGetLastValueOfRow(row_index))?;

        let row_below_last_value = values[row_index + 1]
            .last()
            .ok_or_else(|| Error::CouldNotGetLastValueOfRow(row_index + 1))?;

        let next_value = row_last_value + row_below_last_value;

        values[row_index].push(next_value);
    }

    Ok(values[0][values[0].len() - 1])
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<i32> {
    let input = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| {
                    n.parse::<i32>()
                        .map_err(Error::CouldNotParseNumber)
                })  
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    let extrapolations = input
        .iter()
        .map(|row| extrapolate_value(row))
        .collect::<Result<Vec<_>>>()?;

    let sum = extrapolations.iter().sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_calcualte_differences() -> miette::Result<()> {
        assert_eq!(
            vec![3, 3, 3, 3, 3],
            calculate_differences(&[0, 3, 6, 9, 12, 15])
        );

        assert_eq!(
            vec![3, 3, 5, 9, 15],
            calculate_differences(&[10, 13, 16, 21, 30, 45])
        );
        Ok(())
    }

    #[test]
    fn it_should_extrapolate_values() -> miette::Result<()> {
        assert_eq!(18, extrapolate_value(&[0, 3, 6, 9, 12, 15])?);
        assert_eq!(68, extrapolate_value(&[10, 13, 16, 21, 30, 45])?);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        assert_eq!(114, process(input)?);
        Ok(())
    }
}
