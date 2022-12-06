type Range = (u32, u32);
// range1 fully contains range2
pub fn fully_contains(range1: &Range, range2: &Range) -> bool {
    range2.0 >= range1.0 && range2.1 <= range1.1
}
pub fn overlaps(range1: &Range, range2: &Range) -> bool {
    range1.0 <= range2.1 && range2.0 <= range1.1
}
#[aoc_generator(day4)]
pub fn day4_input(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(p1, p2)| (p1.split_once('-').unwrap(), p2.split_once('-').unwrap()))
        .map(|((s1, e1), (s2, e2))| {
            (
                (s1.parse().unwrap(), e1.parse().unwrap()),
                (s2.parse().unwrap(), e2.parse().unwrap()),
            )
        })
        .collect()
}
#[aoc(day4, part1)]
pub fn part1(input: &[(Range, Range)]) -> u32 {
    input
        .iter()
        .filter(|(r1, r2)| fully_contains(r1, r2) || fully_contains(r2, r1))
        .count() as u32
}
#[aoc(day4, part2)]
pub fn part2(input: &[(Range, Range)]) -> u32 {
    input.iter().filter(|(r1, r2)| overlaps(r1, r2)).count() as u32
}
