#[aoc(day10, part1)]
pub fn part1(input: &str) -> i32 {
    let mut clock = 1;
    let mut x = 1;
    let mut signal_strength = 0;
    for l in input.lines() {
        if (clock + 20) % 40 == 0 {
            signal_strength += clock * x;
        }
        clock += 1;
        if &l[0..4] == "addx" {
            if (clock + 20) % 40 == 0 {
                signal_strength += clock * x;
            }
            clock += 1;
            x += l[5..].parse::<i32>().unwrap();
        }
    }
    signal_strength
}
#[aoc(day10, part2)]
pub fn part2(input: &str) -> u32 {
    let mut clock = 0;
    let mut x = 1;
    let mut text_buffer: Vec<char> = Vec::new();

    for l in input.lines() {
        // if we are overlapping the clock pixel
        if x - 1 <= clock % 40 && x + 1 >= clock % 40 {
            text_buffer.push('#');
        } else {
            text_buffer.push(' ');
        }
        clock += 1;
        if &l[0..4] == "addx" {
            if x - 1 <= clock % 40 && x + 1 >= clock % 40 {
                text_buffer.push('#');
            } else {
                text_buffer.push(' ');
            }
            clock += 1;
            x += l[5..].parse::<i32>().unwrap();
        }
    }
    text_buffer
        .chunks(40)
        .for_each(|c| println!("{}", c.iter().collect::<String>()));
    0
}
