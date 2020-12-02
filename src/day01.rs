use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|s| s.trim().parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(_vals: &[usize]) -> usize {
    0
}
