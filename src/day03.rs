use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.trim().to_string()).collect()
}

fn calculate(vals: &[String], right: usize, down: usize) -> usize {
    vals.iter()
        .step_by(down)
        .enumerate()
        .filter(|(index, line)| line.chars().nth((index * right) % line.len()).unwrap() == '#')
        .count()
}

#[aoc(day3, part1)]
fn part1(vals: &[String]) -> usize {
    calculate(vals, 3, 1)
}

#[aoc(day3, part2)]
fn part2(vals: &[String]) -> usize {
    let counts = [
        calculate(vals, 1, 1),
        calculate(vals, 3, 1),
        calculate(vals, 5, 1),
        calculate(vals, 7, 1),
        calculate(vals, 1, 2),
    ];

    counts.iter().product()
}
