use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::Ordering;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|s| s.trim().parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(vals: &[usize]) -> usize {
    let mut v = vals.to_vec();
    v.sort_unstable();

    let mut front_iter = v.iter();
    let mut back_iter = v.iter().rev();
    let mut front = front_iter.next().unwrap();
    let mut back = back_iter.next().unwrap();

    while front < back {
        let sum = front + back;

        match sum.cmp(&2020) {
            Ordering::Equal => return front * back,
            Ordering::Less => front = front_iter.next().unwrap(),
            Ordering::Greater => back = back_iter.next().unwrap(),
        }
    }

    0
}

#[aoc(day1, part2)]
fn part2(vals: &[usize]) -> usize {
    vals.iter()
        .combinations(3)
        .find(|v| (v[0] + v[1] + v[2]) == 2020)
        .map(|v| v[0] * v[1] * v[2])
        .unwrap()
}
