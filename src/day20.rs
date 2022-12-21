use itertools::Itertools;

#[aoc_generator(day20)]
fn parse(input: &str) -> Vec<(usize, i64)> {
    input
        .lines()
        .filter_map(|l| l.parse::<i64>().ok())
        .enumerate()
        .collect()
}
fn mix(markers: &[(usize, i64)], mixer: &mut Vec<(usize, i64)>, n: usize) {
    let len = markers.len();
    for _ in 0..n {
        for &(marker, num) in markers {
            let curr_index = mixer.iter().position(|&i| i == (marker, num)).unwrap();
            mixer.remove(curr_index);
            // do modulus not remainder
            let new_index = (curr_index as i64 + num).rem_euclid(len as i64 - 1) as usize;
            mixer.insert(new_index, (marker, num));
        }
    }

    let zero_index = mixer.iter().position(|i| i.1 == 0).unwrap();
    mixer.rotate_left(zero_index);
}
#[aoc(day20, part1)]
fn part1(input: &[(usize, i64)]) -> i64 {
    let mut mixer = input.to_vec();
    let len = input.len();
    mix(input, &mut mixer, 1);
    mixer[1000 % len].1 + mixer[2000 % len].1 + mixer[3000 % len].1
}
#[aoc(day20, part2)]
fn part2(input: &[(usize, i64)]) -> i64 {
    let input = input
        .iter()
        .map(|&(_i, num)| (_i, num * 811589153))
        .collect_vec();
    let mut mixer = input.clone();
    let len = input.len();
    mix(&input, &mut mixer, 10);
    mixer[1000 % len].1 + mixer[2000 % len].1 + mixer[3000 % len].1
}
