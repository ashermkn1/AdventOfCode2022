use crate::day13::Packet::*;
use std::cmp::Ordering;
#[derive(Eq, PartialEq, Clone)]
enum Packet {
    Node(u8),
    List(Vec<Packet>),
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // simply compare ints
            (Node(x), Node(y)) => x.cmp(y),
            // compare each element or lengths
            (List(a), List(b)) => match a.iter().cmp(b) {
                Ordering::Equal => a.len().cmp(&b.len()),
                ord => ord,
            },
            // turn node to list
            (List(_), Node(x)) => self.cmp(&List(vec![Node(*x)])),
            // just use above pattern
            (Node(_), List(_)) => other.cmp(self).reverse(),
        }
    }
}
impl Packet {
    fn from_str(s: &str) -> Packet {
        if s.starts_with('[') {
            let s = &s[1..s.len() - 1];
            // empty list
            let mut inner = Vec::new();
            let (mut depth, mut substr_start) = (0, 0);
            if s.is_empty() {
                return List(vec![]);
            }
            for (i, c) in s.chars().enumerate() {
                match c {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' if depth == 0 => {
                        inner.push(Self::from_str(&s[substr_start..i]));
                        substr_start = i + 1;
                    }
                    _ => {}
                }
            }
            inner.push(Self::from_str(&s[substr_start..]));
            List(inner)
        } else {
            println!("Parsing {s}");
            Node(s.parse().unwrap())
        }
    }
}
#[aoc_generator(day13)]
fn day13_input(input: &str) -> Vec<Packet> {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(Packet::from_str)
        .collect()
}
#[aoc(day13, part1)]
fn part1(input: &[Packet]) -> usize {
    input.chunks(2).enumerate().fold(
        0,
        |acc, (i, pair)| {
            if pair[0] < pair[1] {
                acc + i + 1
            } else {
                acc
            }
        },
    )
}
#[aoc(day13, part2)]
fn part2(input: &[Packet]) -> usize {
    let mut input = input.to_vec();
    let div1 = Packet::from_str("[[2]]");
    let div2 = Packet::from_str("[[6]]");
    input.push(div1.clone());
    input.push(div2.clone());
    input.sort_unstable();
    (input.iter().position(|p| *p == div1).unwrap() + 1)
        * (input.iter().position(|p| *p == div2).unwrap() + 1)
}
