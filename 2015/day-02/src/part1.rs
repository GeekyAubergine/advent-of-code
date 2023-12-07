use crate::error::Error;
use crate::prelude::*;

#[tracing::instrument]
fn wrapping_paper_for_box(l: u64, w: u64, h: u64) -> u64 {
    let side1 = l * w;
    let side2 = w * h;
    let side3 = h * l;

    let smallest_side = side1.min(side2).min(side3);

    let total_area = 2 * side1 + 2 * side2 + 2 * side3;

    total_area + smallest_side
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64> {
    let box_areas = input
        .lines()
        .map(|line| {
            let mut dimensions = line.split('x');
            let l = dimensions
                .next()
                .ok_or(Error::ExpectedNumber)?
                .parse::<u64>()
                .map_err(Error::CouldNotParseNumber)?;
            
            let w = dimensions
                .next()
                .ok_or(Error::ExpectedNumber)?
                .parse::<u64>()
                .map_err(Error::CouldNotParseNumber)?;

            let h = dimensions
                .next()
                .ok_or(Error::ExpectedNumber)?
                .parse::<u64>()
                .map_err(Error::CouldNotParseNumber)?;
            
            Ok(wrapping_paper_for_box(l, w, h))
        })
        .collect::<Result<Vec<u64>>>()?;

    let total_area = box_areas.iter().sum();

    Ok(total_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_calculate_correct_area_of_box() -> miette::Result<()> {
        assert_eq!(wrapping_paper_for_box(2, 3, 4), 58);
        assert_eq!(wrapping_paper_for_box(1, 1, 10), 43);

        Ok(())
    }
}
