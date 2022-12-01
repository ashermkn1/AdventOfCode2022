#[aoc_generator(day1)]
pub fn day1_input(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|l| {
            l.lines().map(|m| {
                m.parse::<u32>().unwrap()
            }).collect()
        }).collect()
}
#[aoc(day1, part1)]
pub fn part1(input: &[Vec<u32>]) -> u32 {
    let mut elfmax: u32 = 0;
    for elf in input {

        let s = elf.iter().sum();
        if s > elfmax {
            elfmax = s;
        }
    }
    elfmax
}
#[aoc(day1,part2)]
pub fn part2(input: &[Vec<u32>]) -> u32 {
    let mut calorie_counts: Vec<u32> = input.iter().map(|elf| elf.iter().sum()).collect();
    calorie_counts.sort();
    calorie_counts.reverse();
    calorie_counts[0..3].iter().sum()
}