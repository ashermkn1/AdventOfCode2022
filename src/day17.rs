use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Wind {
    Left,
    Right,
}
#[derive(Copy, Clone)]
struct Shape(u32);
const LEFT_EDGE: u32 = 0x40404040;
const RIGHT_EDGE: u32 = 0x01010101;
impl Shape {
    const fn all_shapes() -> [Self; 5] {
        [
            Self(0x0000001E),
            Self(0x00081C08),
            Self(0x0004041C),
            Self(0x10101010),
            Self(0x00001818),
        ]
    }
    const fn intersects(&self, mask: u32) -> bool {
        self.0 & mask != 0
    }
    fn blow(&mut self, wind: Wind, mask: u32) {
        let new_pos = match wind {
            Wind::Left => {
                // make sure we arent at the left edge yet
                if self.0 & LEFT_EDGE == 0 {
                    self.0 << 1
                } else {
                    return;
                }
            }
            Wind::Right => {
                // make sure we aren't at the right edge yet
                if self.0 & RIGHT_EDGE == 0 {
                    self.0 >> 1
                } else {
                    return;
                }
            }
        };
        // make sure we won't collide into any rocks
        if new_pos & mask == 0 {
            self.0 = new_pos;
        }
    }
    fn into_bytes(self) -> impl Iterator<Item = u8> {
        self.0.to_le_bytes().into_iter().take_while(|b| *b != 0)
    }
}
// bitmask representing row of tower
fn tower_mask(tower: &[u8], height: usize) -> u32 {
    if height >= tower.len() {
        0
    } else {
        tower[height..]
            .iter()
            .take(4)
            .rev()
            .fold(0u32, |acc, b| (acc << 8) | *b as u32)
    }
}
fn drop_rock(tower: &mut Vec<u8>, wind: &[Wind], mut wind_idx: usize, mut shape: Shape) -> usize {
    let mut height = tower.len() + 3;

    loop {
        let wind_dir = wind[wind_idx];
        // wrap wind_idx back to 0
        wind_idx = (wind_idx + 1) % wind.len();

        shape.blow(wind_dir, tower_mask(tower, height));
        // the shape comes to rest
        if height == 0 || shape.intersects(tower_mask(tower, height - 1)) {
            // work from bottom to top
            for byte in shape.into_bytes() {
                // or into the tower
                if height < tower.len() {
                    tower[height] |= byte;
                } else {
                    tower.push(byte);
                }
                height += 1;
            }
            return wind_idx;
        } else {
            height -= 1;
        }
    }
}
#[aoc_generator(day17)]
fn parse(input: &str) -> Vec<Wind> {
    input
        .chars()
        .map(|c| match c {
            '<' => Wind::Left,
            '>' => Wind::Right,
            _ => unreachable!(),
        })
        .collect()
}
#[aoc(day17, part1)]
fn part1(input: &[Wind]) -> usize {
    let num_rocks = 2022;
    let mut tower = Vec::with_capacity(num_rocks * 4);

    let mut wind_idx = 0;
    for shape in Shape::all_shapes().into_iter().cycle().take(num_rocks) {
        wind_idx = drop_rock(&mut tower, input, wind_idx, shape);
    }
    tower.len()
}
#[aoc(day17, part2)]
fn part2(input: &[Wind]) -> usize {
    let num_rocks = 1000000000000;
    let mut seen: HashMap<(u64, usize, usize), (usize, usize)> = HashMap::with_capacity(1024);
    let mut tower = Vec::with_capacity(1024);

    let mut cycled_height = 0;
    let mut wind_idx = 0;
    let shapes = Shape::all_shapes().into_iter().collect_vec();
    let mut rock_count = 0;
    while rock_count < num_rocks {
        let shape_idx = rock_count % shapes.len();
        let shape = shapes[shape_idx];

        wind_idx = drop_rock(&mut tower, input, wind_idx, shape);
        rock_count += 1;
        // no need to memoize if we are tiny tower
        if tower.len() < 8 {
            continue;
        }
        // skyline is last 8 rows, nicely forms a 64 bit int
        let skyline = u64::from_ne_bytes(tower[tower.len() - 8..].try_into().unwrap());

        let state = (skyline, shape_idx, wind_idx);

        match seen.entry(state) {
            Entry::Occupied(e) => {
                let (old_count, old_len) = e.get();
                let rocks_in_cycle = rock_count - old_count;
                let num_cycles = (num_rocks - rock_count) / rocks_in_cycle;
                rock_count += num_cycles * rocks_in_cycle;
                cycled_height += num_cycles * (tower.len() - old_len);
                seen.clear();
            }
            Entry::Vacant(e) => {
                e.insert((rock_count, tower.len()));
            }
        }
    }
    tower.len() + cycled_height
}
