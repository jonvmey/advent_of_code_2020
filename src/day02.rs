use aoc_runner_derive::{aoc, aoc_generator};

use regex::Regex;

struct Rule {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

impl Rule {
    fn new(min: usize, max: usize, character: char, password: &str) -> Self {
        Self {
            min,
            max,
            character,
            password: password.to_string(),
        }
    }

    fn validate(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|&c| c == self.character)
            .count();
        self.min <= count && count <= self.max
    }
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Rule> {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();

    re.captures_iter(input)
        .map(|v| {
            let min: usize = v[1].parse().unwrap();
            let max: usize = v[2].parse().unwrap();
            let character = v[3].chars().next().unwrap();
            Rule::new(min, max, character, &v[4])
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(vals: &[Rule]) -> usize {
    vals.iter().filter(|v| v.validate()).count()
}
