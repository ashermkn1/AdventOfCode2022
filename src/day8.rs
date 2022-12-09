use itertools::Itertools;

#[aoc_generator(day8)]
pub fn day8_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}
#[aoc(day8, part1)]
pub fn part1(input: &Vec<Vec<u8>>) -> usize {
    let rows = input.len();
    let cols = input[0].len();
    let mut visible = rows * 2 + cols * 2 - 4;
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            let height = input[row][col];
            let (l, r) = input[row].split_at(col);
            // do cool things with r bc it has current tree as r[0]
            if l.iter().all(|&h| h < height) || r.iter().position_max().unwrap() == 0 {
                visible += 1;
                continue;
            }
            // up down
            if (0..row).all(|h| input[h][col] < height)
                || (row + 1..rows).all(|h| input[h][col] < height)
            {
                visible += 1;
                continue;
            }
        }
    }
    visible
}
// call this once on input and its transpose
pub fn score_lr(forest: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> usize {
    let height = forest[row][col];
    let mut score_r = 0;
    for &h in forest[row].iter().skip(col + 1) {
        score_r += 1;
        if h >= height {
            break;
        }
    }
    let mut score_l = 0;
    for &h in forest[row].iter().rev().skip(forest[row].len() - col) {
        score_l += 1;
        if h >= height {
            break;
        }
    }
    score_l * score_r
}
#[aoc(day8, part2)]
pub fn part2(input: &Vec<Vec<u8>>) -> usize {
    let input_trans: Vec<Vec<u8>> = (0..input[0].len())
        .map(|i| input.iter().map(|row| row[i].clone()).collect::<Vec<u8>>())
        .collect();
    let score = |(row, col): (usize, usize)| -> usize {
        score_lr(input, (row, col)) * score_lr(&input_trans, (col, row))
    };
    input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, _)| (i, j)))
        .map(score)
        .max()
        .unwrap()
}
