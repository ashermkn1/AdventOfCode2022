use itertools::Itertools;
use std::collections::HashSet;
#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let mut count = 4;
    for (a, b, c, d) in input.chars().tuple_windows() {
        if HashSet::from([a, b, c, d]).len() == 4 {
            return count;
        }
        count += 1;
    }
    0
}
#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let mut count = 14;
    for a in input.as_bytes().windows(14) {
        let mut s = HashSet::new();
        for &x in a {
            s.insert(x);
        }
        if s.len() == 14 {
            return count;
        }
        count += 1;
    }
    0
}
