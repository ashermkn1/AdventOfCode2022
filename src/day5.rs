use regex::Regex;
pub struct Input {
    stacks: Vec<Vec<u8>>,
    moves: Vec<(usize, usize, usize)>,
}
#[aoc_generator(day5)]
pub fn day5_input(input: &str) -> Input {
    let stacks = Vec::from([
        Vec::from("WRF"),
        Vec::from("THMCDVWP"),
        Vec::from("PMZNL"),
        Vec::from("JCHR"),
        Vec::from("CPGHQTB"),
        Vec::from("GCWLFZ"),
        Vec::from("WVLQZJGC"),
        Vec::from("PNRFWTVC"),
        Vec::from("JWHGRSV"),
    ]);
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let moves = input
        .lines()
        .skip(10)
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
            )
        })
        .collect();
    Input { stacks, moves }
}
#[aoc(day5, part1)]
pub fn part1(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    for (amt, from, to) in &input.moves {
        for _ in 0..*amt {
            let val = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(val);
        }
    }
    stacks
        .iter()
        .map(|stack| *stack.last().unwrap())
        .map(|val| val as char)
        .collect::<String>()
}
#[aoc(day5, part2)]
pub fn part2(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    for (amt, from, to) in &input.moves {
        let len = stacks[from - 1].len();
        let mut vals = stacks[from - 1].split_off(len - *amt);
        stacks[to - 1].append(&mut vals);
    }
    stacks
        .iter()
        .map(|stack| *stack.last().unwrap())
        .map(|val| val as char)
        .collect::<String>()
}
