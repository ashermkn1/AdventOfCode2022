#[derive(Debug)]
pub struct Directory {
    name: String,
    dirs: Vec<Directory>,
    files: Vec<usize>,
    total_size: usize,
}
impl Directory {
    pub fn new(name: String) -> Self {
        Directory {
            name,
            dirs: Vec::new(),
            files: Vec::new(),
            total_size: 0,
        }
    }
}
// do op at path
pub fn path_operation<F: Fn(&mut Directory)>(node: &mut Directory, path: &[String], op: F) {
    if path.is_empty() {
        op(node);
    } else {
        path_operation(
            node.dirs
                .iter_mut()
                .find(|dir| dir.name == path[0])
                .unwrap(),
            &path[1..],
            op,
        );
    }
}
pub fn compute_total_size(dir: &mut Directory) -> usize {
    dir.total_size = dir.dirs.iter_mut().map(compute_total_size).sum::<usize>()
        + dir.files.iter().sum::<usize>();
    dir.total_size
}
#[aoc_generator(day7)]
pub fn day7_input(input: &str) -> Directory {
    let mut root = Directory::new("".to_string());

    let mut pwd = Vec::new();
    // split on $ giving us each instruction (and its outputs)
    for cmd in input.split('$').skip(1) {
        match cmd.trim().lines().next().unwrap() {
            "ls" => cmd.lines().skip(1).for_each(|res| {
                match res.split_once(' ').unwrap() {
                    // add all sub directories
                    ("dir", dir_name) => {
                        path_operation(&mut root, &pwd, |node: &mut Directory| {
                            node.dirs.push(Directory::new(dir_name.to_string()))
                        });
                    }
                    // add all files
                    (size, _) => {
                        path_operation(&mut root, &pwd, |node: &mut Directory| {
                            node.files.push(size.parse::<usize>().unwrap())
                        });
                    }
                };
                // apply operation to current directory
            }),
            // go up one directory
            "cd .." => {
                pwd.pop();
            }
            "cd /" => {}
            // go to directory
            cd_dir => pwd.push(cd_dir.split_once(' ').unwrap().1.to_string()),
        }
    }
    // compute total sizes for each directory
    compute_total_size(&mut root);
    root
}
// helper for below function
pub fn totals_helper(dir: &Directory, vec: &mut Vec<usize>) {
    dir.dirs.iter().for_each(|d| totals_helper(d, vec));
    vec.push(dir.total_size)
}
// recursively compute total size of directory
pub fn totals(dir: &Directory) -> Vec<usize> {
    let mut v = Vec::new();
    totals_helper(dir, &mut v);
    v
}
#[aoc(day7, part1)]
pub fn part1(input: &Directory) -> usize {
    let totals = totals(input);
    totals
        .into_iter()
        .filter(|&size| size <= 100_000_usize)
        .sum()
}
#[aoc(day7, part2)]
pub fn part2(input: &Directory) -> usize {
    let totals = totals(input);
    let total_size = *totals.last().unwrap();
    totals
        .into_iter()
        .filter(|&size| 40000000 + size >= total_size)
        .min()
        .unwrap()
}
