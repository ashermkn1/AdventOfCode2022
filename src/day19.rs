use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
#[derive(Debug, Copy, Clone)]
struct RobotCost {
    ores: usize,
    clay: usize,
    obsidian: usize,
}
#[derive(Debug, Copy, Clone)]
struct Inventory {
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
    ores: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
}
impl Inventory {
    fn new() -> Self {
        Inventory {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ores: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }
    fn num_robots(&self, robot: Material) -> usize {
        match robot {
            Material::Ore => self.ore_robots,
            Material::Clay => self.clay_robots,
            Material::Obsidian => self.obsidian_robots,
            Material::Geode => self.geode_robots,
        }
    }
    fn _mine(&mut self) {
        self.ores += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;
    }
    // takes ownership so we can chain
    fn mine(self) -> Self {
        let mut other = self;
        other._mine();
        other
    }
    fn unmine(self) -> Self {
        let mut other = self;
        other.ores -= other.ore_robots;
        other.clay -= other.clay_robots;
        other.obsidian -= other.obsidian_robots;
        other.geodes -= other.geode_robots;
        other
    }
    fn build(self, blueprint: &Blueprint, robot: Material) -> Self {
        let RobotCost {
            ores,
            clay,
            obsidian,
        } = blueprint.cost(robot);

        let mut other = self;
        other.ores -= ores;
        other.clay -= clay;
        other.obsidian -= obsidian;

        other._mine();
        match robot {
            Material::Ore => other.ore_robots += 1,
            Material::Clay => other.clay_robots += 1,
            Material::Obsidian => other.obsidian_robots += 1,
            Material::Geode => other.geode_robots += 1,
        }
        other
    }
}
#[derive(Debug)]
struct Blueprint {
    ore_robot: RobotCost,
    clay_robot: RobotCost,
    obsidian_robot: RobotCost,
    geode_robot: RobotCost,
}
impl Blueprint {
    fn cost(&self, robot: Material) -> RobotCost {
        match robot {
            Material::Ore => self.ore_robot,
            Material::Clay => self.clay_robot,
            Material::Obsidian => self.obsidian_robot,
            Material::Geode => self.geode_robot,
        }
    }
    fn can_build(&self, inv: &Inventory, robot: Material) -> bool {
        let RobotCost {
            ores,
            clay,
            obsidian,
        } = self.cost(robot);

        inv.ores >= ores && inv.clay >= clay && inv.obsidian >= obsidian
    }
    /// Rules we follow:
    ///
    /// 1. Always build a geode robot
    /// 2. If we skipped building this robot last turn, don't build it now
    /// 3. If we are already producing more than it costs to build, don't build
    fn should_build(&self, inv: &Inventory, robot: Material, built: bool) -> bool {
        // number 1
        if robot == Material::Geode {
            return true;
        }
        // we only need material enough to build any robot
        // if we are already producing more, don't build another robot
        let max_cost = [
            self.ore_robot,
            self.clay_robot,
            self.obsidian_robot,
            self.geode_robot,
        ]
        .into_iter()
        .map(|cost| match robot {
            Material::Ore => cost.ores,
            Material::Clay => cost.clay,
            Material::Obsidian => cost.obsidian,
            Material::Geode => unreachable!(),
        })
        .max()
        .unwrap_or(0);

        let need_more = inv.num_robots(robot) < max_cost;

        if built {
            need_more
        } else {
            // check if we skipped building this robot last time
            let prev = inv.unmine();
            let skipped = self.can_build(&prev, robot);
            need_more && !skipped
        }
    }
}
// max geodes that can be produced in the given minutes
// simple BFS with state = (inventory, time_taken, whether we just built a robot)
fn search(factory: &Blueprint, minutes: i32) -> usize {
    let mut queue = VecDeque::new();
    // initial state
    queue.push_back((Inventory::new(), 0, false));
    // initialize to all zeroes
    let mut cache: HashMap<i32, usize> = HashMap::new();
    for i in 0..=minutes {
        cache.insert(i, 0);
    }

    while let Some((inv, min, built)) = queue.pop_front() {
        let &best = cache.get(&min).unwrap();
        // we already have a better route
        if inv.geodes < best {
            continue;
        }
        // update best
        cache.insert(min, inv.geodes);
        // times up
        if min == minutes {
            continue;
        }
        // follow rule 1
        if factory.can_build(&inv, Material::Geode) {
            queue.push_back((inv.build(factory, Material::Geode), min + 1, true));
            continue;
        }
        // we don't build anything
        queue.push_back((inv.mine(), min + 1, false));
        // build all the things
        for material in [Material::Obsidian, Material::Clay, Material::Ore] {
            if factory.can_build(&inv, material) && factory.should_build(&inv, material, built) {
                queue.push_back((inv.build(factory, material), min + 1, true))
            }
        }
    }
    *cache.get(&minutes).unwrap()
}
#[aoc_generator(day19)]
fn parse_input(input: &str) -> Vec<Blueprint> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)").unwrap();
    }
    input
        .lines()
        .map(|line| {
            // get rid of id
            let line = line.split_once(':').unwrap().1;
            // all costs
            let costs: Vec<usize> = RE
                .captures_iter(line)
                .map(|c| c[1].parse::<usize>().unwrap())
                .collect_vec();

            let ore_robot = RobotCost {
                ores: costs[0],
                clay: 0,
                obsidian: 0,
            };

            let clay_robot = RobotCost {
                ores: costs[1],
                clay: 0,
                obsidian: 0,
            };

            let obsidian_robot = RobotCost {
                ores: costs[2],
                clay: costs[3],
                obsidian: 0,
            };

            let geode_robot = RobotCost {
                ores: costs[4],
                clay: 0,
                obsidian: costs[5],
            };

            Blueprint {
                ore_robot,
                clay_robot,
                obsidian_robot,
                geode_robot,
            }
        })
        .collect()
}
#[aoc(day19, part1)]
fn part1(input: &[Blueprint]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let score = search(b, 24);
            score * (i + 1)
        })
        .sum()
}
