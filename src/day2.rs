use std::ops::Deref;

pub enum Action {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}
pub enum Outcome {
    WIN = 6,
    DRAW = 3,
    LOSE = 0,
}
#[aoc_generator(day2, part1)]
pub fn day2_input(input: &str) -> Vec<(Action, Action)> {
    input
        .lines()
        .map(|line| {
            let (opp, me) = line.split_once(' ').unwrap();
            (
                match opp {
                    "A" => Action::ROCK,
                    "B" => Action::PAPER,
                    "C" => Action::SCISSORS,
                    _ => panic!("unexpected input"),
                },
                match me {
                    "X" => Action::ROCK,
                    "Y" => Action::PAPER,
                    "Z" => Action::SCISSORS,
                    _ => panic!("unexpected input"),
                },
            )
        })
        .collect()
}
#[aoc_generator(day2, part2)]
pub fn day2_input_part2(input: &str) -> Vec<(Action, Outcome)> {
    input
        .lines()
        .map(|line| {
            let (opp, me) = line.split_once(' ').unwrap();
            (
                match opp {
                    "A" => Action::ROCK,
                    "B" => Action::PAPER,
                    "C" => Action::SCISSORS,
                    _ => panic!("unexpected input"),
                },
                match me {
                    "X" => Outcome::LOSE,
                    "Y" => Outcome::DRAW,
                    "Z" => Outcome::WIN,
                    _ => panic!("unexpected input"),
                },
            )
        })
        .collect()
}
#[aoc(day2, part1)]
pub fn part1(input: &[(Action, Action)]) -> u32 {
    input
        .iter()
        .map(|(opp, me)| match me {
            Action::ROCK => {
                Action::ROCK as u32
                    + match opp {
                        Action::ROCK => Outcome::DRAW as u32,
                        Action::PAPER => Outcome::LOSE as u32,
                        Action::SCISSORS => Outcome::WIN as u32,
                    }
            }
            Action::PAPER => {
                Action::PAPER as u32
                    + match opp {
                        Action::ROCK => Outcome::WIN as u32,
                        Action::PAPER => Outcome::DRAW as u32,
                        Action::SCISSORS => Outcome::LOSE as u32,
                    }
            }
            Action::SCISSORS => {
                Action::SCISSORS as u32
                    + match opp {
                        Action::ROCK => Outcome::LOSE as u32,
                        Action::PAPER => Outcome::WIN as u32,
                        Action::SCISSORS => Outcome::DRAW as u32,
                    }
            }
        })
        .sum()
}
#[aoc(day2, part2)]
pub fn part2(input: &[(Action, Outcome)]) -> u32 {
    input
        .iter()
        .map(|(opp, me)| match me {
            Outcome::WIN => {
                Outcome::WIN as u32
                    + match opp {
                        Action::ROCK => Action::PAPER as u32,
                        Action::PAPER => Action::SCISSORS as u32,
                        Action::SCISSORS => Action::ROCK as u32,
                    }
            }
            Outcome::DRAW => {
                Outcome::DRAW as u32
                    + match opp {
                        Action::ROCK => Action::ROCK as u32,
                        Action::PAPER => Action::PAPER as u32,
                        Action::SCISSORS => Action::SCISSORS as u32,
                    }
            }
            Outcome::LOSE => {
                Outcome::LOSE as u32
                    + match opp {
                        Action::ROCK => Action::SCISSORS as u32,
                        Action::PAPER => Action::ROCK as u32,
                        Action::SCISSORS => Action::PAPER as u32,
                    }
            }
        })
        .sum()
}
