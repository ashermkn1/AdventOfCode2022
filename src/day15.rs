use itertools::Itertools;
use regex::Regex;
use std::cmp::max;
use std::ops::Range;

type Input = (Vec<Sensor>, Vec<Beacon>);
struct Beacon {
    x: isize,
    y: isize,
}
struct Sensor {
    x: isize,
    y: isize,
    range: usize,
}
impl Sensor {
    fn range_at_row(&self, row: isize) -> Option<Range<isize>> {
        let reach = (self.range - row.abs_diff(self.y)) as isize;
        if reach >= 0 {
            let length = 2 * reach + 1;
            let start = self.x - reach;
            let end = start + length;
            Some(start..end)
        } else {
            None
        }
    }
    // where the lines extending from each edge of the manhattan rhombus have x = 0
    fn intercepts(&self) -> Vec<isize> {
        let range = self.range as isize;
        vec![
            self.x + (self.y + range), // bottom right edge
            self.x + (self.y - range), // top left edge
            self.x - (self.y - range), // top right edge
            self.x - (self.y + range), // bottom left edge
        ]
    }
}
fn manhattan(from: (isize, isize), to: (isize, isize)) -> usize {
    from.0.abs_diff(to.0) + from.1.abs_diff(to.1)
}
#[aoc_generator(day15)]
fn parse_input(input: &str) -> Input {
    let re = Regex::new(r".*x=(-?\d+), y=(-?\d+).*x=(-?\d+), y=(-?\d+)").unwrap();

    input
        .lines()
        .map(|l| re.captures(l))
        .map(|cap| {
            let cap = cap.unwrap();
            (
                (
                    cap.get(1).unwrap().as_str().parse::<isize>().unwrap(),
                    cap.get(2).unwrap().as_str().parse::<isize>().unwrap(),
                ),
                (
                    cap.get(3).unwrap().as_str().parse::<isize>().unwrap(),
                    cap.get(4).unwrap().as_str().parse::<isize>().unwrap(),
                ),
            )
        })
        .map(|(s, b)| {
            (
                Sensor {
                    x: s.0,
                    y: s.1,
                    range: manhattan(s, b),
                },
                Beacon { x: b.0, y: b.1 },
            )
        })
        .unzip()
}
// make non-overlapping ranges on row (where beacons can't be)
fn ranges_at_row(sensors: &[Sensor], row: isize) -> Vec<Range<isize>> {
    let mut ranges: Vec<Range<isize>> = sensors
        .iter()
        .filter_map(|s| s.range_at_row(row))
        .sorted_by_key(|r| r.start)
        .rev()
        .collect();

    match ranges.pop() {
        Some(mut current) => {
            let mut res = Vec::new();
            loop {
                if let Some(next) = ranges.pop() {
                    // collapse neighbouring ranges together if they overlap
                    if next.start <= current.end {
                        current.end = next.end.max(current.end);
                    } else {
                        res.push(current);
                        current = next;
                    }
                } else {
                    res.push(current);
                    break res;
                }
            }
        }
        None => Vec::new(),
    }
}
#[aoc(day15, part1)]
fn part1(input: &Input) -> usize {
    let row = 2_000_000;
    let beacons_at_row = input
        .1
        .iter()
        .filter(|b| b.y == row)
        .map(|b| b.x)
        .sorted()
        .dedup()
        .count();
    ranges_at_row(&input.0, row)
        .into_iter()
        .map(|r| r.len())
        .sum::<usize>()
        - beacons_at_row
}
fn find_free(points: &mut [((isize, isize), &Sensor)], row: isize) -> Option<isize> {
    points.iter_mut().for_each(|p| {
        p.0 =
            p.1.range_at_row(row)
                .map(|r| (r.start, r.end - 1))
                .unwrap_or((0, 0));
    });

    points.sort_unstable_by(|a, b| a.0 .0.cmp(&b.0 .0));
    let mut max_val: isize = 0;
    for ((start, end), _) in &*points {
        let start = *start;
        let end = *end;
        if (start, end) == (0, 0) {
            continue;
        }
        if start > max_val {
            return Some(start - 1);
        }
        if end > 4_000_000 {
            return None;
        }
        max_val = max(max_val, end);
    }
    None
}
#[aoc(day15, part2)]
fn part2(input: &Input) -> i64 {
    let sensors = &input.0;
    let mut ranges: Vec<_> = vec![(0isize, 0isize); sensors.len()]
        .into_iter()
        .zip(sensors)
        .collect();
    for y in 0..=4_000_000 {
        if let Some(avail) = find_free(&mut ranges, y) {
            return avail as i64 * 4_000_000 + y as i64;
        }
    }
    0
}
