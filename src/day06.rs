use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.trim().to_string()).collect()
}

#[aoc(day6, part1)]
fn part1(vals: &[String]) -> usize {
    vals.join("\n")
        .split("\n\n")
        .map(|s| s.chars().filter(|&c| c != '\n').unique().count())
        .sum()
}
