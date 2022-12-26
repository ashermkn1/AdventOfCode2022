use std::iter::once;
const DIGITS: [u8; 5] = [b'=', b'-', b'0', b'1', b'2'];
fn from_snafu(snafu: &str) -> i64 {
    snafu.chars().fold(0, |acc, elem| {
        acc * 5
            + match elem {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => unreachable!(),
            }
    })
}
fn to_snafu(val: i64) -> Vec<u8> {
    if val != 0 {
        let x = val + 2;

        let (div, rem) = (x.div_euclid(5), x.rem_euclid(5));

        return to_snafu(div)
            .iter()
            .copied()
            .chain(once(DIGITS[rem as usize]))
            .collect();
    }
    Vec::new()
}
#[aoc_generator(day25)]
fn parse(input: &str) -> Vec<i64> {
    input.lines().map(from_snafu).collect()
}
#[aoc(day25, part1)]
fn part1(input: &[i64]) -> String {
    String::from_utf8(to_snafu(input.iter().sum())).unwrap()
}
