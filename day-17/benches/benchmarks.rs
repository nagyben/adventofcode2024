use day_17::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../input.txt")));
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!("../input.txt")));
}

#[divan::bench]
fn part2_example() {
    const INPUT: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;
    part2::process(divan::black_box(INPUT));
}
