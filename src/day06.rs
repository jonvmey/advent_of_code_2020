use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<String> {
    input.split("\n\n").map(|s| s.to_string()).collect()
}

#[aoc(day6, part1)]
fn part1(vals: &[String]) -> usize {
    vals.iter()
        .map(|s| s.chars().filter(|&c| c != '\n').unique().count())
        .sum()
}

#[aoc(day6, part2)]
fn part2(vals: &[String]) -> usize {
    vals.iter()
        .map(|v| {
            let char_counts = v.chars().fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            });

            let num_responses = char_counts.get(&'\n').unwrap_or(&0) + 1;

            char_counts
                .values()
                .filter(|&v| *v == num_responses)
                .count()
        })
        .sum()
}
