use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct IdInfo {
    birth_year: Option<()>,
    issue_year: Option<()>,
    expiration_year: Option<()>,
    height: Option<()>,
    hair_colour: Option<()>,
    eye_colour: Option<()>,
    passport_id: Option<()>,
    country_id: Option<()>,
}

impl IdInfo {
    fn default() -> Self {
        Self {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_colour: None,
            eye_colour: None,
            passport_id: None,
            country_id: None,
        }
    }

    fn new(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-z]{3}):(\#?([a-zA-Z0-9])+)").unwrap();
        }

        let mut id = Self::default();

        for captures in RE.captures_iter(s) {
            match &captures[1] {
                "byr" => id.birth_year = Some(()),
                "iyr" => id.issue_year = Some(()),
                "eyr" => id.expiration_year = Some(()),
                "hgt" => id.height = Some(()),
                "hcl" => id.hair_colour = Some(()),
                "ecl" => id.eye_colour = Some(()),
                "pid" => id.passport_id = Some(()),
                "cid" => id.country_id = Some(()),
                _ => {}
            }
        }

        id
    }

    fn required_fields_present(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_colour.is_some()
            && self.eye_colour.is_some()
            && self.passport_id.is_some()
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<String> {
    input.split("\n\n").map(|s| s.replace("\n", " ")).collect()
}

#[aoc(day4, part1)]
fn part1(vals: &[String]) -> usize {
    vals.iter()
        .map(|v| IdInfo::new(v))
        .filter(|id| id.required_fields_present())
        .count()
}
