use itertools::Itertools;
use num_rational::Rational64;
use std::collections::HashMap;
use Expr::*;
type Number = Rational64;
use std::ops::{Add, Div, Mul, Sub};
enum Expr<'a> {
    Literal(Number),
    Unknown,
    Add(&'a str, &'a str),
    Subtract(&'a str, &'a str),
    Multiply(&'a str, &'a str),
    Divide(&'a str, &'a str),
}

fn parse_expr(s: &str) -> Expr {
    if let Ok(n) = s.parse::<Number>() {
        return Literal(n);
    }

    let (a, op, b) = s.split(' ').collect_tuple().unwrap();

    match op {
        "+" => Add(a, b),
        "-" => Subtract(a, b),
        "*" => Multiply(a, b),
        "/" => Divide(a, b),
        _ => unreachable!("OOH OOH AHH AHH"),
    }
}
type Monkeys<'a> = HashMap<&'a str, Expr<'a>>;

fn parse(input: &str) -> Monkeys {
    input
        .lines()
        .map(|l| {
            l.split_once(": ")
                .map(|(name, rest)| (name, parse_expr(rest)))
                .unwrap()
        })
        .collect()
}
fn eval1<'a>(
    monkeys: &Monkeys<'a>,
    which: &'a str,
    cache: &mut HashMap<&'a str, Number>,
) -> Number {
    match cache.get(which) {
        Some(n) => *n,
        None => {
            let n = match monkeys.get(which) {
                Some(Literal(num)) => *num,
                Some(Add(a, b)) => eval1(monkeys, a, cache) + eval1(monkeys, b, cache),
                Some(Subtract(a, b)) => eval1(monkeys, a, cache) - eval1(monkeys, b, cache),
                Some(Multiply(a, b)) => eval1(monkeys, a, cache) * eval1(monkeys, b, cache),
                Some(Divide(a, b)) => eval1(monkeys, a, cache) / eval1(monkeys, b, cache),
                Some(Unknown) => unreachable!(),
                None => unreachable!(),
            };
            cache.insert(which, n);
            n
        }
    }
}
#[aoc(day21, part1)]
fn part1(input: &str) -> Number {
    let monkeys = parse(input);
    eval1(&monkeys, "root", &mut HashMap::new())
}
#[derive(Debug, Copy, Clone)]
struct Formula {
    // ax + b
    a: Number,
    b: Number,
}
impl Add<Formula> for Formula {
    type Output = Self;

    fn add(self, rhs: Formula) -> Self::Output {
        Self {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}
impl Sub<Formula> for Formula {
    type Output = Self;

    fn sub(self, rhs: Formula) -> Self::Output {
        Self {
            a: self.a - rhs.a,
            b: self.b - rhs.b,
        }
    }
}
impl Mul<Formula> for Formula {
    type Output = Self;

    fn mul(self, rhs: Formula) -> Self::Output {
        let (a, b) = (self.a, self.b);
        let (c, d) = (rhs.a, rhs.b);
        Self {
            a: a * d + b * c,
            b: b * d,
        }
    }
}
impl Div<Formula> for Formula {
    type Output = Formula;

    fn div(self, rhs: Formula) -> Self::Output {
        let (a, b) = (self.a, self.b);
        let (_, d) = (rhs.a, rhs.b);
        Self { a: a / d, b: b / d }
    }
}
fn eval2<'a>(
    monkeys: &Monkeys<'a>,
    which: &'a str,
    cache: &mut HashMap<&'a str, Formula>,
) -> Formula {
    match cache.get(which) {
        Some(n) => *n,
        None => {
            let n = match monkeys.get(which) {
                Some(Literal(num)) => Formula {
                    a: 0.into(),
                    b: *num,
                },
                Some(Add(a, b)) => eval2(monkeys, a, cache) + eval2(monkeys, b, cache),
                Some(Subtract(a, b)) => eval2(monkeys, a, cache) - eval2(monkeys, b, cache),
                Some(Multiply(a, b)) => eval2(monkeys, a, cache) * eval2(monkeys, b, cache),
                Some(Divide(a, b)) => eval2(monkeys, a, cache) / eval2(monkeys, b, cache),
                Some(Unknown) => Formula {
                    a: 1.into(),
                    b: 0.into(),
                },
                None => unreachable!(),
            };
            cache.insert(which, n);
            n
        }
    }
}
#[aoc(day21, part2)]
fn part2(input: &str) -> Number {
    let mut monkeys = parse(input);
    let (a, b) = match monkeys.get("root") {
        Some(Add(a, b)) => (a, b),
        Some(Subtract(a, b)) => (a, b),
        Some(Divide(a, b)) => (a, b),
        Some(Multiply(a, b)) => (a, b),
        _ => unreachable!(),
    };
    monkeys.insert("root", Subtract(a, b));
    monkeys.insert("humn", Unknown);
    let eq = eval2(&monkeys, "root", &mut HashMap::new());
    -eq.b / eq.a
}
