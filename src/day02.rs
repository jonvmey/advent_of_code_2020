use aoc_runner_derive::aoc;

use regex::Regex;

struct Rule<'a> {
    first: usize,
    second: usize,
    character: char,
    password: &'a str,
}

impl<'a> Rule<'a> {
    fn new(first: usize, second: usize, character: char, password: &'a str) -> Self {
        Self {
            first,
            second,
            character,
            password,
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

    fn validate_part2(&self) -> bool {
        (self.nth_char(self.first) == self.character)
            ^ (self.nth_char(self.second) == self.character)
    }

    fn nth_char(&self, n: usize) -> char {
        self.password.chars().nth(n - 1).unwrap()
    }
}

fn parse_input(input: &str) -> Vec<Rule> {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();

    re.captures_iter(input)
        .map(|cap| {
            Rule::new(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].chars().next().unwrap(),
                cap.get(4).unwrap().as_str(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|v| v.validate_part1())
        .count()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|v| v.validate_part2())
        .count()
}
