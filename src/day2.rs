#[aoc_generator(day2)]
pub fn day2_input(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|line| {
            let (opp, me) = line.split_once(' ').unwrap();
            (
                match opp {
                    "A" => 0,
                    "B" => 1,
                    "C" => 2,
                    _ => unreachable!("unexpected input"),
                },
                match me {
                    "X" => 0,
                    "Y" => 1,
                    "Z" => 2,
                    _ => unreachable!("unexpected input"),
                },
            )
        })
        .collect()
}
#[aoc(day2, part1)]
pub fn part1(input: &[(u32, u32)]) -> u32 {
    input
        .iter()
        .map(|(opp, me)| {
            let score = me + 1;
            // we tie
            if me == opp {
                return score + 3;
            } else if (opp + 1) % 3 == *me {
                // we win
                return score + 6;
            }
            // we lose
            score
        })
        .sum()
}
#[aoc(day2, part2)]
pub fn part2(input: &[(u32, u32)]) -> u32 {
    input
        .iter()
        .map(|(opp, me)| {
            match me {
                // we lose
                0 => 1 + (opp + 2) % 3,
                // draw
                1 => 3 + 1 + opp,
                // we win
                2 => 6 + 1 + (opp + 1) % 3,
                _ => unreachable!("unexpected"),
            }
        })
        .sum()
}
