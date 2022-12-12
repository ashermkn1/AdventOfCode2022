use itertools::Itertools;
use std::collections::VecDeque;
use std::str::{FromStr, Lines};
#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u32>,
    operation: Operation,
    test: Test,
    inspections: u64,
}
impl Monkey {
    fn inspect(&mut self, divisor: Option<u32>) -> Option<(u32, usize)> {
        // get next item, if there is any
        let Some(mut curr) = self.items.pop_front() else {return None;};
        // apply operation
        curr = self.operation.apply(curr);
        self.inspections += 1;
        // reduce worry
        if let Some(div) = divisor {
            curr %= div;
        } else {
            curr /= 3;
        }

        // return item and where it goes
        Some((curr, self.test.apply(curr)))
    }
}
#[derive(Debug, Clone)]
enum Operation {
    Add(u32),
    Mult(u32),
    Square(),
}
impl Operation {
    fn apply(&self, old: u32) -> u32 {
        match self {
            Operation::Add(val) => old + val,
            Operation::Mult(val) => old * val,
            Operation::Square() => old * old,
        }
    }
}
#[derive(Debug, Clone)]
struct Test {
    test_value: u32,
    if_true: usize,
    if_false: usize,
}
impl Test {
    fn apply(&self, val: u32) -> usize {
        if val % self.test_value == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}
fn error() -> &'static str {
    "Failed to parse monke"
}
impl FromStr for Monkey {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut next = { || lines.next().ok_or_else(error) };

        next()?.chars().nth(7).unwrap().to_digit(10).unwrap();
        let items = parse_items(next()?)?;
        let op = parse_op(next()?)?;
        let test = parse_test(lines);
        Ok(Monkey {
            items,
            operation: op,
            test,
            inspections: 0,
        })
    }
}
fn parse_test(lines: Lines) -> Test {
    let (div, t, f) = lines
        .map(|l| l.split(' ').last().unwrap())
        .map(|num| num.parse().unwrap())
        .collect_tuple()
        .unwrap();
    Test {
        test_value: div as u32,
        if_true: t,
        if_false: f,
    }
}
fn parse_op(s: &str) -> Result<Operation, &'static str> {
    s.split_once("old ")
        .and_then(|(_, r)| r.split_once(' '))
        .and_then(|(l, r)| {
            let Some(val) = r.parse::<u32>().ok() else {
                return Some(Operation::Square());
            };
            match l {
                "*" => Some(Operation::Mult(val)),
                "+" => Some(Operation::Add(val)),
                _ => unreachable!(),
            }
        })
        .ok_or_else(error)
}
fn parse_items(s: &str) -> Result<VecDeque<u32>, &'static str> {
    s.split_once(':')
        .map(|(_, items)| {
            items
                .trim()
                .split(',')
                .filter_map(|r| r.trim().parse::<u32>().ok())
                .collect::<VecDeque<u32>>()
        })
        .ok_or_else(error)
}
fn monkey_business(monkeys: &[Monkey]) -> u64 {
    monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product()
}
fn print_monkeys(monkeys: &[Monkey]) {
    for i in 0..monkeys.len() {
        println!(
            "Monkey {i} inspected items {} times",
            monkeys[i].inspections
        );
    }
    println!()
}
fn solve(monkeys: &mut [Monkey], rounds: u32, divisor: Option<u32>) -> u64 {
    println!("{monkeys:#?}");
    for i in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some((item, to)) = monkeys[i].inspect(divisor) {
                monkeys[to].items.push_back(item);
            }
        }
        if (i + 1) % 20 == 0 {
            println!("After round {}", i + 1);
            print_monkeys(monkeys)
        }
    }
    monkey_business(monkeys)
}
#[aoc_generator(day11)]
fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monke| monke.parse().unwrap())
        .collect()
}
#[aoc(day11, part1)]
fn part1(input: &[Monkey]) -> u64 {
    solve(&mut input.to_vec(), 20, None)
}
#[aoc(day11, part2)]
fn part2(input: &[Monkey]) -> u64 {
    let divisor: u32 = input.iter().map(|m| m.test.test_value).product();
    solve(&mut input.to_vec(), 10_000, Some(divisor))
}
