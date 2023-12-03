use day_03::part1::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    let file = include_str!("../../input1.txt");
    let result = process(file).context("process part 1")?;
    println!("{}", result);
    Ok(())
}