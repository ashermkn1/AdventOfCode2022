use itertools::Itertools;

pub fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}
pub fn common_chars(a: &str, b: &str) -> Vec<char> {
    a.chars().filter(|&c| b.contains(c)).collect()
}
#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(r1, r2)| common_chars(r1, r2))
        .map(|c| priority(c[0]))
        .sum()
}
#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .tuples()
        .map(|(a, b, c)| {
            common_chars(
                a,
                &common_chars(b, c).into_iter().collect::<String>().as_str(),
            )
        })
        .map(|c| priority(c[0]))
        .sum()
}
