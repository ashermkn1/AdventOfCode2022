use crate::day9::Direction::*;
use std::collections::HashSet;
#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
struct Move {
    dir: Direction,
    num_steps: usize,
}
type Coordinate = (i32, i32);
struct Rope {
    knots: Vec<Coordinate>,
}
impl Rope {
    fn tail(&self) -> Coordinate {
        *self.knots.last().unwrap()
    }
    fn update_tail(&mut self) {
        let mut prev = *self.knots.first().unwrap();
        for cur in self.knots.iter_mut().skip(1) {
            let dx = prev.0 - cur.0;
            let dy = prev.1 - cur.1;

            if dx.abs() > 1 || dy.abs() > 1 {
                cur.0 += dx.signum();
                cur.1 += dy.signum();
            }
            prev = *cur;
        }
    }
    fn step(&mut self, dir: &Direction) {
        let mut head = self.knots.first_mut().unwrap();

        match dir {
            Left => head.0 -= 1,
            Right => head.0 += 1,
            Up => head.1 += 1,
            Down => head.1 -= 1,
        }
        self.update_tail()
    }
}
#[aoc_generator(day9)]
fn day9_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let (dir, len) = line.split_once(' ').unwrap();
            Move {
                dir: match dir {
                    "L" => Left,
                    "R" => Right,
                    "U" => Up,
                    _ => Down,
                },
                num_steps: len.parse().unwrap(),
            }
        })
        .collect()
}
#[aoc(day9, part1)]
fn part1(input: &[Move]) -> usize {
    let mut rope = Rope {
        knots: vec![(0, 0); 2],
    };
    let mut set = HashSet::with_capacity(input.len());
    set.insert((0, 0));
    for m in input {
        for _ in 0..m.num_steps {
            rope.step(&m.dir);
            set.insert(rope.tail());
        }
    }
    set.len()
}
#[aoc(day9, part2)]
fn part2(input: &[Move]) -> usize {
    let mut rope = Rope {
        knots: vec![(0, 0); 10],
    };
    let mut set = HashSet::with_capacity(input.len());
    set.insert((0, 0));
    for m in input {
        for _ in 0..m.num_steps {
            rope.step(&m.dir);
            set.insert(rope.tail());
        }
    }
    set.len()
}
