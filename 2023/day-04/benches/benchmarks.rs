use day_04::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}

// #[divan::bench]
// fn part1_opt() {
//     part1_opt::process(divan::black_box(include_str!(
//         "../input1.txt",
//     )))
//     .unwrap();
// }

// #[divan::bench]
// fn part2_opt() {
//     part2_opt::process(divan::black_box(include_str!(
//         "../input2.txt",
//     )))
//     .unwrap();
// }