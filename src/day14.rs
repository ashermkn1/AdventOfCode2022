use itertools::Itertools;
use std::cmp::{max, min};
const STARTX: usize = 500;
const STARTY: usize = 0;
const DIMX: usize = 1000;
const DIMY: usize = 1000;
type Grid = Box<[Vec<bool>]>;
#[aoc_generator(day14)]
fn parse_input(input: &str) -> (Grid, usize) {
    let mut grid = vec![vec![false; DIMX]; DIMY].into_boxed_slice();
    let mut floor: usize = 0;
    for line in input.lines() {
        let pairs: Vec<(usize, usize)> = line
            .split(" -> ")
            .map(|s| {
                s.split_once(',')
                    .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                    .unwrap()
            })
            .collect();
        for (&(sx, sy), &(ex, ey)) in pairs.iter().tuple_windows() {
            // vertical line
            if sx == ex {
                // update floor
                let maxy = max(sy, ey);
                floor = max(floor, maxy);
                (min(sy, ey)..=maxy).for_each(|i| grid[sx][i] = true);
            } else {
                // update floor
                floor = max(floor, sy);
                (min(sx, ex)..=max(sx, ex)).for_each(|i| grid[i][sy] = true);
            }
        }
    }
    floor += 2;
    println!("Floor is {floor}");
    (grid, floor)
}
fn drop_sand(grid: &mut Grid) -> usize {
    let mut x = STARTX;
    let mut y = STARTY;
    let mut count = 0;
    loop {
        // sand has fallen into the abyss
        if y + 1 == DIMY {
            break;
        }
        // check down
        if !grid[x][y + 1] {
            y += 1;
        } else if x > 0 && !grid[x - 1][y + 1] {
            x -= 1;
            y += 1;
        } else if x < DIMX - 1 && !grid[x + 1][y + 1] {
            x += 1;
            y += 1;
        } else {
            grid[x][y] = true;
            count += 1;
            y = STARTY;
            x = STARTX;
        }
    }
    count
}
#[aoc(day14, part1)]
fn part1(input: &(Grid, usize)) -> usize {
    let mut grid = input.0.clone();
    drop_sand(&mut grid)
}
#[aoc(day14, part2)]
fn part2(input: &(Grid, usize)) -> usize {
    let mut sand = 0;
    let mut grid = input.0.clone();
    let floor = input.1;
    grid.iter_mut().for_each(|c| c[floor] = true);
    'sand: loop {
        let (mut x, mut y): (usize, usize) = (500, 0);
        'fall: loop {
            if !grid[x][y + 1] {
                y += 1;
            } else if !grid[x - 1][y + 1] {
                x -= 1;
                y += 1;
            } else if !grid[x + 1][y + 1] {
                x += 1;
                y += 1;
            } else if (x, y) == (500, 0) {
                sand += 1;
                break 'sand;
            } else {
                grid[x][y] = true;
                break 'fall;
            }
        }
        sand += 1;
    }
    sand
}
