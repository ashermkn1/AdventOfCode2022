use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Ground,
    Wall,
    Empty,
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum Action {
    Rotate(usize),
    Move(i32),
}

struct Actions<'a>(&'a str);

impl<'a> Iterator for Actions<'a> {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .chars()
            .position(|c| !c.is_ascii_digit())
            .map(|i| {
                if i > 0 {
                    let (steps, rest) = self.0.split_at(i);
                    self.0 = rest;
                    Action::Move(steps.parse().unwrap())
                } else {
                    let (rotate, rest) = self.0.split_at(1);
                    self.0 = rest;
                    match rotate {
                        "R" => Action::Rotate(1),
                        "L" => Action::Rotate(3),
                        _ => unreachable!(),
                    }
                }
            })
            .or_else(|| {
                if !self.0.is_empty() {
                    let num = self.0.parse().unwrap();
                    self.0 = "";
                    Some(Action::Move(num))
                } else {
                    None
                }
            })
    }
}

const DX: &[i32; 4] = &[0, 1, 0, -1];
const DY: &[i32; 4] = &[1, 0, -1, 0];
fn parse_grid(input: &str) -> (Vec<Vec<Tile>>, &str) {
    let (map, commands): (&str, &str) = input.split("\n\n").collect_tuple().unwrap();

    let mut grid = map
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Ground,
                    ' ' => Tile::Empty,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();
    // total width of the grid, max of any row's length
    let width = grid.iter().map(|row| row.len()).max().unwrap();

    for row in grid.iter_mut() {
        row.resize_with(width, || Tile::Empty);
    }

    (grid, commands)
}
#[aoc(day22, part1)]
fn part1(input: &str) -> i32 {
    let (map, commands) = parse_grid(input);

    let mut facing = 0;
    let mut x = 0;
    let mut y = map[0]
        .iter()
        .position(|&tile| tile == Tile::Ground)
        .unwrap() as i32;
    // map direction to dx, dy

    let (length, width) = (
        map.len() as i32,
        map.iter().map(|row| row.len()).max().unwrap() as i32,
    );

    for action in Actions(commands) {
        match action {
            Action::Rotate(dir) => {
                facing = (facing + dir) % 4;
            }
            Action::Move(steps) => {
                for _ in 0..steps {
                    // step
                    let mut nx = (x + DX[facing] + length) % length;
                    let mut ny = (y + DY[facing] + width) % width;
                    // handle wrapping over empty space
                    // keep walking until we reach solid ground
                    while map[nx as usize][ny as usize] == Tile::Empty {
                        nx = (nx + DX[facing] + length) % length;
                        ny = (ny + DY[facing] + width) % width;
                    }
                    // only update position if we reach solid ground
                    if map[nx as usize][ny as usize] == Tile::Wall {
                        break;
                    } else {
                        x = nx;
                        y = ny;
                    }
                }
            }
        }
    }
    (x + 1) * 1000 + (y + 1) * 4 + facing as i32
}
#[derive(Debug)]
struct Cube {
    side_length: i32,
    map: Vec<Vec<Tile>>,
    face_transitions: [[(usize, usize); 4]; 6],
    face_id: HashMap<(i32, i32), usize>,
}
// id of face
// basically the 50x50 area it is in
fn face(x: i32, y: i32, side_length: i32) -> (i32, i32) {
    (
        (x + side_length) / side_length,
        (y + side_length) / side_length,
    )
}
impl Cube {
    fn new(map: Vec<Vec<Tile>>) -> Self {
        // compute dimension of each face
        let face_area = map
            .iter()
            .flat_map(|row| row.iter().filter(|&t| *t != Tile::Empty))
            .count()
            / 6;

        let side_length = (face_area as f64).sqrt() as u32;
        // map from face id to face number
        let mut face_id = HashMap::new();
        for (row_id, row) in map.iter().enumerate() {
            for (col_id, cell) in row.iter().enumerate() {
                if cell != &Tile::Empty {
                    let id = face(row_id as i32, col_id as i32, side_length as i32);
                    let n = face_id.len();
                    face_id.entry(id).or_insert_with(|| n);
                }
            }
        }
        // build transitions from each face
        // transitions[i][dir] will be the face and direction you get to by going "dir"
        // on the ith face
        let mut transitions = [[None; 4]; 6];
        // build trivial transitions
        // ones that are apparent from input
        for (&(x, y), &num) in face_id.iter() {
            for dir in 0..4 {
                let nx = x + DX[dir];
                let ny = y + DY[dir];

                if let Some(&id) = face_id.get(&(nx, ny)) {
                    transitions[num][dir] = Some((id, dir));
                }
            }
        }
        // build other transitions
        // we use the fact that
        // going left and forward, then right and forward, then left
        // brings you to the same face and direction as just going forward
        let mut need_more = true;
        while need_more {
            for face in 0..6 {
                for dir in 0..4 {
                    if transitions[face][dir].is_some() {
                        continue;
                    }

                    let left = (dir + 3) & 3;

                    if let Some((left_face, left_facing)) = transitions[face][left] {
                        let right = (left_facing + 1) & 3;

                        if let Some((right_face, right_facing)) = transitions[left_face][right] {
                            let left = (right_facing + 3) & 3;
                            transitions[face][dir] = Some((right_face, left));
                        }
                    }
                }
            }
            need_more = transitions.iter().flatten().any(|i| i.is_none());
        }
        // have to do this the hard way bc can't collect into a slice
        let face_transitions = {
            let mut target = [[(0, 0); 4]; 6];
            for face in 0..6 {
                for dir in 0..4 {
                    target[face][dir] = transitions[face][dir].unwrap();
                }
            }
            target
        };

        Self {
            side_length: side_length as i32,
            map,
            face_transitions,
            face_id,
        }
    }

    fn face(&self, x: i32, y: i32) -> (i32, i32) {
        face(x, y, self.side_length)
    }

    fn step(&self, x: i32, y: i32, dir: usize) -> (i32, i32, usize) {
        let start_face = self.face(x, y);
        let nx = x + DX[dir];
        let ny = y + DY[dir];

        let end_face = self.face(nx, ny);

        if start_face == end_face {
            (nx, ny, dir)
        } else {
            let temp = self.face_id[&start_face];
            let (end_faceid, end_dir) = self.face_transitions[temp][dir];
            // handle rotation across faces
            let (mut localx, mut localy) = (x % self.side_length, y % self.side_length);

            let mut direction = dir;

            while direction != end_dir {
                direction = (direction + 1) & 3;

                let nx = localy;
                let ny = self.side_length - localx - 1;

                localx = nx;
                localy = ny;
            }

            let &(x_id, y_id) = self
                .face_id
                .iter()
                .find(|&(_, y)| *y == end_faceid)
                .unwrap()
                .0;

            let x_id = (x_id - 1) * self.side_length;
            let y_id = (y_id - 1) * self.side_length;

            (
                x_id + localx + (1 - self.side_length) * DX[end_dir],
                y_id + localy + (1 - self.side_length) * DY[end_dir],
                end_dir,
            )
        }
    }

    fn get(&self, x: i32, y: i32) -> Tile {
        self.map[x as usize][y as usize]
    }
}

#[aoc(day22, part2)]
fn part2(input: &str) -> i32 {
    let (map, commands) = parse_grid(input);

    let mut facing = 0;
    let mut x = 0;
    let mut y = map[0]
        .iter()
        .position(|&tile| tile == Tile::Ground)
        .unwrap() as i32;
    // map direction to dx, dy

    let width = map.iter().map(|row| row.len()).max().unwrap() as i32;

    let cube = Cube::new(map);

    for action in Actions(commands) {
        match action {
            Action::Rotate(dir) => {
                facing = (facing + dir) & 3;
            }
            Action::Move(steps) => {
                for _ in 0..steps {
                    let (nx, ny, next_dir) = cube.step(x, y, facing);
                    match cube.get(nx, ny) {
                        Tile::Ground => {
                            x = nx;
                            y = ny;
                            facing = next_dir;
                        }
                        Tile::Wall => {
                            break;
                        }
                        Tile::Empty => unreachable!(),
                    }
                }
            }
        }
    }
    (x + 1) * 1000 + (y + 1) * 4 + facing as i32
}
