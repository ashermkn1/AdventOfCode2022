use std::collections::{HashMap, HashSet};

const MAX_ROOMS: usize = 60;
type DistanceMatrix = [[usize; MAX_ROOMS]; MAX_ROOMS];
#[derive(Debug, Default, Eq, PartialEq, Hash, Clone)]
struct Valve {
    flow_rate: usize,
    neighbors: Vec<usize>,
}
#[derive(Debug)]
struct Cave {
    start_idx: usize,
    valves: Vec<Valve>,
    valve_map: HashMap<String, usize>,
}
impl Cave {
    fn new() -> Self {
        Cave {
            start_idx: usize::MAX,
            valves: Vec::with_capacity(MAX_ROOMS),
            valve_map: HashMap::new(),
        }
    }
    // bfs type thing to calculate all distances
    fn calc_distances(&self) -> DistanceMatrix {
        let mut matrix = [[usize::MAX; MAX_ROOMS]; MAX_ROOMS];
        let mut seen = HashSet::new();

        for (i, _) in self
            .valves
            .iter()
            .enumerate()
            .filter(|&(i, r)| r.flow_rate > 0 || i == self.start_idx)
        {
            matrix[i][i] = 0;

            let mut current = HashSet::new();
            current.insert(i);
            let mut next = HashSet::new();
            let mut dist = 0;

            while !current.is_empty() {
                dist += 1;
                for pos in &current {
                    for new_valve in &self.valves[*pos].neighbors {
                        if !seen.contains(&(i, *new_valve)) {
                            next.insert(*new_valve);
                            matrix[i][*new_valve] = dist;
                            seen.insert((i, *new_valve));
                        }
                    }
                }
                current.clear();
                current.extend(next.drain());
            }
        }
        matrix
    }
}
#[aoc_generator(day16)]
fn parse_input(input: &str) -> Cave {
    let (_, name_idx, mut idx_valve) = input.lines().fold(
        (0usize, HashMap::new(), HashMap::new()),
        |(mut idx, mut name_idx, mut idx_valve), line| {
            let (a, b) = line.split_once(';').unwrap();
            let valve = &a[6..8];
            let flow_rate = a[a.find('=').unwrap() + 1..].parse::<usize>().unwrap();

            let valve_idx = *name_idx.entry(valve.to_string()).or_insert_with(|| {
                idx += 1;
                idx
            });

            let mut neighbors = Vec::new();
            b.split_ascii_whitespace()
                .skip(4)
                .map(|x| x.trim().replace(',', ""))
                .for_each(|n| {
                    let z = *name_idx.entry(n).or_insert_with(|| {
                        idx += 1;
                        idx
                    });
                    neighbors.push(z);
                });
            idx_valve.insert(
                valve_idx,
                Valve {
                    flow_rate,
                    neighbors,
                },
            );
            (idx, name_idx, idx_valve)
        },
    );
    let mut res = Cave::new();
    res.start_idx = *name_idx.get("AA").unwrap();
    res.valve_map = name_idx;

    res.valves.push(Valve {
        flow_rate: usize::MAX,
        neighbors: vec![],
    });
    let n = idx_valve.len();
    for idx in 1..=n {
        res.valves.push(idx_valve.remove(&idx).unwrap())
    }
    res
}
fn max_release(
    dist: &DistanceMatrix,
    cave: &Cave,
    current: usize,
    time: usize,
    targets: &mut HashSet<usize>,
) -> (usize, HashSet<usize>) {
    targets.remove(&current);

    let mut max = 0;
    let mut best_path = HashSet::new();

    for t in targets.iter() {
        // bound at 0
        let remaining = time.saturating_sub(dist[current][*t]).saturating_sub(1);
        if remaining > 0 {
            let mut flow = cave.valves[*t].flow_rate * remaining;
            let (newflow, p) = max_release(dist, cave, *t, remaining, &mut targets.clone());
            flow += newflow;

            if flow > max {
                max = flow;
                best_path = p.clone();
                best_path.insert(current);
            }
        }
    }
    (max, best_path)
}
#[aoc(day16, part1)]
fn part1(input: &Cave) -> usize {
    let distances = input.calc_distances();

    let mut targets = HashSet::from_iter(
        input
            .valves
            .iter()
            .enumerate()
            .filter(|&(i, r)| r.flow_rate > 0 || i == input.start_idx)
            .map(|(i, _)| i),
    );

    max_release(&distances, input, input.start_idx, 30, &mut targets).0
}
#[aoc(day16, part2)]
fn part2(input: &Cave) -> usize {
    let distances = input.calc_distances();

    let mut targets = HashSet::from_iter(
        input
            .valves
            .iter()
            .enumerate()
            .filter(|&(i, r)| r.flow_rate > 0 || i == input.start_idx)
            .map(|(i, _)| i),
    );

    let (human_release, human_path) =
        max_release(&distances, input, input.start_idx, 26, &mut targets);

    let mut cave_elephant = Cave::new();
    cave_elephant.start_idx = input.start_idx;
    cave_elephant.valve_map = input.valve_map.clone();

    for (idx, valve) in input.valves.iter().enumerate() {
        let v = Valve {
            flow_rate: valve.flow_rate,
            neighbors: valve
                .neighbors
                .iter()
                .filter(|n| !human_path.contains(n))
                .copied()
                .collect(),
        };

        if human_path.contains(&idx) && idx != input.start_idx {
            cave_elephant.valves.push(Valve {
                flow_rate: 0,
                neighbors: vec![],
            });
        } else {
            cave_elephant.valves.push(v);
        }
    }

    let mut elephant_targets = HashSet::from_iter(
        cave_elephant
            .valves
            .iter()
            .enumerate()
            .filter(|&(i, r)| r.flow_rate > 0 || i == cave_elephant.start_idx)
            .map(|(i, _)| i),
    );

    let (elephant_release, _) = max_release(
        &distances,
        &cave_elephant,
        cave_elephant.start_idx,
        26,
        &mut elephant_targets,
    );

    human_release + elephant_release
}
