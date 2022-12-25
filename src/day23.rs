use fxhash::FxHashSet as HashSet;
use itertools::Itertools;

type Pos = (i32, i32);
const SURROUNDING: [Pos; 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

const DIRECTIONS: [[Pos; 3]; 4] = [
    [(1, -1), (0, -1), (-1, -1)],
    [(1, 1), (0, 1), (-1, 1)],
    [(-1, 1), (-1, 0), (-1, -1)],
    [(1, 1), (1, 0), (1, -1)],
];

#[aoc_generator(day23)]
fn parse(input: &str) -> HashSet<Pos> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes()
                .enumerate()
                .filter(|&(_, c)| c == b'#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect()
}
fn destination(elf: Pos, step: usize, elves: &HashSet<Pos>) -> Pos {
    if SURROUNDING
        .iter()
        .any(|dir| elves.contains(&(elf.0 + dir.0, elf.1 + dir.1)))
    {
        for dir in step..step + 4 {
            let dirs = DIRECTIONS[dir & 3];

            // move if no elves
            if !dirs
                .iter()
                .any(|d| elves.contains(&(elf.0 + d.0, elf.1 + d.1)))
            {
                return (elf.0 + dirs[1].0, elf.1 + dirs[1].1);
            }
        }
    }
    elf
}
#[aoc(day23, part1)]
fn part1(input: &HashSet<Pos>) -> usize {
    let mut elves = input.clone();

    for round in 0..10 {
        let mut new_elves: HashSet<Pos> =
            HashSet::with_capacity_and_hasher(elves.capacity(), Default::default());

        for &elf in elves.iter() {
            let new_pos = destination(elf, round, &elves);
            if new_pos == elf {
                new_elves.insert(elf);
            } else if !new_elves.insert(new_pos) {
                new_elves.remove(&new_pos);
                new_elves.insert(elf);
                // conflict must come from opposite direction
                new_elves.insert((new_pos.0 * 2 - elf.0, new_pos.1 * 2 - elf.1));
            }
        }
        elves = new_elves;
    }
    let (minx, maxx) = elves
        .iter()
        .map(|(x, _)| *x)
        .minmax()
        .into_option()
        .unwrap();
    let (miny, maxy) = elves
        .iter()
        .map(|(_, y)| *y)
        .minmax()
        .into_option()
        .unwrap();

    (minx..=maxx)
        .cartesian_product(miny..=maxy)
        .filter(|p| !elves.contains(p))
        .count()
}
#[aoc(day23, part2)]
fn part2(input: &HashSet<Pos>) -> usize {
    let mut elves = input.clone();

    for round in 0.. {
        let mut new_elves: HashSet<Pos> =
            HashSet::with_capacity_and_hasher(elves.capacity(), Default::default());

        let mut num_moves = 0;
        for &elf in elves.iter() {
            let new_pos = destination(elf, round, &elves);
            if new_pos == elf {
                new_elves.insert(elf);
            } else if !new_elves.insert(new_pos) {
                new_elves.remove(&new_pos);
                new_elves.insert(elf);
                // conflict must come from opposite direction
                new_elves.insert((new_pos.0 * 2 - elf.0, new_pos.1 * 2 - elf.1));
                num_moves -= 1;
            } else {
                num_moves += 1;
            }
        }
        if num_moves == 0 {
            return round + 1;
        }
        elves = new_elves;
    }
    0
}
