use itertools::Itertools;
use std::collections::VecDeque;
#[derive(Debug, Clone)]
struct Grid {
    start: Vec<(usize, usize)>,
    end: (usize, usize),
    grid: Vec<Vec<u8>>,
}
#[aoc_generator(day12)]
fn day12_input(input: &str) -> Grid {
    let mut grid: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let (sx, sy) = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(x, y)| grid[x][y] == b'S')
        .unwrap();
    let (gx, gy) = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(x, y)| grid[x][y] == b'E')
        .unwrap();
    grid[sx][sy] = b'a';
    grid[gx][gy] = b'z';
    Grid {
        start: Vec::from([(sx, sy)]),
        end: (gx, gy),
        grid,
    }
}

fn bfs(forest: &Grid) -> Option<usize> {
    let grid = &forest.grid;
    let mut mark = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = forest
        .start
        .iter()
        .map(|&(x, y)| (x, y, 0))
        .collect::<VecDeque<_>>();
    while let Some((x, y, len)) = queue.pop_front() {
        if (x, y) == forest.end {
            return Some(len);
        }
        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let (x2, y2) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            // bounds checking
            let Some(&next) = grid.get(x2).and_then(|row| row.get(y2)) else {continue};
            if grid[x][y] + 1 >= next && !mark[x2][y2] {
                mark[x2][y2] = true;
                queue.push_back((x2, y2, len + 1));
            }
        }
    }
    None
}
#[aoc(day12, part1)]
fn part1(input: &Grid) -> usize {
    bfs(input).unwrap()
}
#[aoc(day12, part2)]
fn part2(input: &Grid) -> usize {
    let mut input_2 = input.clone();
    input_2.start = (0..input.grid.len())
        .cartesian_product(0..input.grid[0].len())
        .filter(|&(x, y)| input.grid[x][y] == b'a')
        .collect();
    bfs(&input_2).unwrap()
}
