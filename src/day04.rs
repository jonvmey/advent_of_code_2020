use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct IdInfo<'a> {
    birth_year: Option<&'a str>,
    issue_year: Option<&'a str>,
    expiration_year: Option<&'a str>,
    height: Option<&'a str>,
    hair_colour: Option<&'a str>,
    eye_colour: Option<&'a str>,
    passport_id: Option<&'a str>,
    country_id: Option<&'a str>,
}

impl<'a> IdInfo<'a> {
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

    fn new(s: &'a str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-z]{3}):(\#?([a-zA-Z0-9])+)").unwrap();
        }

        let mut id = Self::default();

        for captures in RE.captures_iter(s) {
            let value = Some(captures.get(2).unwrap().as_str());

            match &captures[1] {
                "byr" => id.birth_year = value,
                "iyr" => id.issue_year = value,
                "eyr" => id.expiration_year = value,
                "hgt" => id.height = value,
                "hcl" => id.hair_colour = value,
                "ecl" => id.eye_colour = value,
                "pid" => id.passport_id = value,
                "cid" => id.country_id = value,
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

    fn valid(&self) -> bool {
        self.valid_birth_year()
            && self.valid_issue_year()
            && self.valid_expiration_year()
            && self.valid_height()
            && self.valid_hair_colour()
            && self.valid_eye_colour()
            && self.valid_passport_id()
            && self.valid_country_id()
    }

    fn valid_birth_year(&self) -> bool {
        // Birth Year - four digits; at least 1920 and at most 2002.
        if let Some(s) = self.birth_year {
            if let Ok(year) = s.parse::<i32>() {
                return (s.len() == 4) && (year >= 1920) && (year <= 2002);
            }
        }

        false
    }

    fn valid_issue_year(&self) -> bool {
        // Issue Year - four digits; at least 2010 and at most 2020.
        if let Some(s) = self.issue_year {
            if let Ok(year) = s.parse::<i32>() {
                return (s.len() == 4) && (year >= 2010) && (year <= 2020);
            }
        }

        false
    }

    fn valid_expiration_year(&self) -> bool {
        // Expiration Year - four digits; at least 2020 and at most 2030.
        if let Some(s) = self.expiration_year {
            if let Ok(year) = s.parse::<i32>() {
                return (s.len() == 4) && (year >= 2020) && (year <= 2030);
            }
        }

        false
    }

    fn valid_height(&self) -> bool {
        // Height) - a number followed by either cm or in:
        // If cm, the number must be at least 150 and at most 193.
        // If in, the number must be at least 59 and at most 76.
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d{2,3})(in|cm)$").unwrap();
        }

        if let Some(s) = self.height {
            if let Some(s) = s.strip_suffix("in") {
                if let Ok(height) = s.parse::<i32>() {
                    return (height >= 59) && (height <= 76);
                }
            }

            if let Some(s) = s.strip_suffix("cm") {
                if let Ok(height) = s.parse::<i32>() {
                    return (height >= 150) && (height <= 193);
                }
            }
        }

        false
    }

    fn valid_hair_colour(&self) -> bool {
        // Hair Colour - a # followed by exactly six characters 0-9 or a-f.
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\#[0-9a-f]{6}$").unwrap();
        }

        if let Some(s) = self.hair_colour {
            RE.is_match(s)
        } else {
            false
        }
    }

    fn valid_eye_colour(&self) -> bool {
        // Eye Colour - exactly one of: amb blu brn gry grn hzl oth.
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
        }

        if let Some(s) = self.eye_colour {
            RE.is_match(s)
        } else {
            false
        }
    }

    fn valid_passport_id(&self) -> bool {
        // Passport ID - a nine-digit number, including leading zeroes.
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        if let Some(s) = self.passport_id {
            RE.is_match(s)
        } else {
            false
        }
    }

    fn valid_country_id(&self) -> bool {
        // Country ID - ignored, missing or not.
        true
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

#[aoc(day4, part2)]
fn part2(vals: &[String]) -> usize {
    vals.iter()
        .map(|v| IdInfo::new(v))
        .filter(|id| id.valid())
        .count()
}
