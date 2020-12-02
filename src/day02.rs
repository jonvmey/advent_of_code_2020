use aoc_runner_derive::{aoc, aoc_generator};

use regex::Regex;

struct Rule {
    first: usize,
    second: usize,
    character: char,
    password: String,
}

impl Rule {
    fn new(first: usize, second: usize, character: char, password: &str) -> Self {
        Self {
            first,
            second,
            character,
            password: password.to_string(),
        }
    }

    fn validate_part1(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|&c| c == self.character)
            .count();
        self.first <= count && count <= self.second
    }
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Rule> {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();

    re.captures_iter(input)
        .map(|v| {
            Rule::new(
                v[1].parse().unwrap(),
                v[2].parse().unwrap(),
                v[3].chars().next().unwrap(),
                &v[4],
            )
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(vals: &[Rule]) -> usize {
    vals.iter().filter(|v| v.validate_part1()).count()
}
