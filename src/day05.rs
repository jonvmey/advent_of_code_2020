use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.trim().to_string()).collect()
}

fn calculate_seat_index(s: &str) -> usize {
    let mut seat_index = 0;
    let mut weight = 64 * 8;

    for c in s.chars() {
        if (c == 'B') || (c == 'R') {
            // Upper half of range
            seat_index += weight;
        }

        weight /= 2;
    }

    seat_index
}

#[aoc(day5, part1)]
fn part1(vals: &[String]) -> usize {
    vals.iter().map(|s| calculate_seat_index(s)).max().unwrap()
}

#[aoc(day5, part2)]
fn part2(vals: &[String]) -> usize {
    let mut indexes: Vec<usize> = vals.iter().map(|s| calculate_seat_index(s)).collect();
    indexes.sort_unstable();

    indexes
        .iter()
        .zip(indexes[1..].iter())
        .find(|(&first, &second)| (second - first) > 1)
        .map(|(first, _)| first + 1)
        .unwrap()
}
