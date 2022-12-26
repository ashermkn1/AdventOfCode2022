use fxhash::FxHashSet as Hashset;
use std::collections::HashSet;

const START: (usize, usize) = (0, 1);
const END: (usize, usize) = (26, 120);
const COLS: usize = 122;
const ROWS: usize = 27;
#[aoc_generator(day24)]
fn parse(input: &str) -> Vec<(usize, usize, char)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(r, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(c, b)| "<^>v".contains(b).then_some((r, c, b)))
        })
        .collect()
}
fn update_positions(
    positions: Hashset<(usize, usize)>,
    blizzards: &mut Vec<(usize, usize, char)>,
) -> Hashset<(usize, usize)> {
    // update blizzard positions
    for (row, col, c) in blizzards.iter_mut() {
        match c {
            '<' => {
                if *col != 1 {
                    *col -= 1;
                } else {
                    // wrap around
                    *col = COLS - 2;
                }
            }
            '>' => {
                if *col != COLS - 2 {
                    *col += 1;
                } else {
                    *col = 1;
                }
            }
            '^' => {
                if *row != 1 {
                    *row -= 1;
                } else {
                    *row = ROWS - 2;
                }
            }
            'v' => {
                if *row != ROWS - 2 {
                    *row += 1;
                } else {
                    *row = 1;
                }
            }
            _ => unreachable!("Bad blizzard"),
        }
    }
    // faster contains method
    let blizzard_pos = blizzards
        .iter()
        .map(|&(x, y, _)| (x, y))
        .collect::<Hashset<_>>();
    // faster than updating in place
    let mut new_pos = Hashset::with_capacity_and_hasher(positions.len(), Default::default());

    for &(x, y) in &positions {
        for (dx, dy) in [(1, 0), (0, 1), (0, 0), (-1, 0), (0, -1)] {
            if x as i32 + dx < 0 || x as i32 + dx >= ROWS as i32 {
                continue;
            }
            let (x, y) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);

            if valid_pos(x, y) && !blizzard_pos.contains(&(x, y)) {
                new_pos.insert((x, y));
            }
        }
    }
    new_pos
}
fn valid_pos(x: usize, y: usize) -> bool {
    (x != 0 || y == 1) && (x != ROWS - 1 || y == COLS - 2) && y != 0 && y != COLS - 1
}
#[aoc(day24, part1)]
fn part1(input: &[(usize, usize, char)]) -> usize {
    let mut blizzards = input.to_vec();
    let mut positions: Hashset<(usize, usize)> = Hashset::from_iter([(0, 1)]);

    for step in 0.. {
        positions = update_positions(positions, &mut blizzards);
        if positions.contains(&END) {
            return step + 1;
        }
    }
    0
}
#[aoc(day24, part2)]
fn part2(input: &[(usize, usize, char)]) -> usize {
    let mut blizzards = input.to_vec();
    let mut positions: Hashset<(usize, usize)> = Hashset::from_iter([(0, 1)]);
    let mut legs = 0;

    for step in 0.. {
        positions = update_positions(positions, &mut blizzards);

        match legs {
            0 => {
                if positions.contains(&END) {
                    legs += 1;
                    positions = HashSet::from_iter([END]);
                }
            }
            1 => {
                if positions.contains(&START) {
                    legs += 1;
                    positions = HashSet::from_iter([START]);
                }
            }
            2 => {
                if positions.contains(&END) {
                    return step + 1;
                }
            }
            _ => unreachable!(),
        }
    }
    0
}
