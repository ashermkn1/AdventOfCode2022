use crate::day19::Material::*;
use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::cmp::max;
type RecipePart = (u32, Material);
#[derive(Copy, Clone)]
enum Material {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

struct Blueprint {
    id: u32,
    recipes: [Vec<RecipePart>; 4],
}
impl Blueprint {
    fn from_line(line: &str) -> Self {
        lazy_static! {
            static ref re: Regex = Regex::new(r"(\d+)").unwrap();
        }
        let nums: Vec<u32> = re
            .captures_iter(line)
            .map(|c| c[1].parse::<u32>().unwrap())
            .collect_vec();
        Self {
            id: nums[0],
            recipes: [
                vec![(nums[1], Ore)],
                vec![(nums[2], Ore)],
                vec![(nums[3], Ore), (nums[4], Clay)],
                vec![(nums[5], Ore), (nums[6], Obsidian)],
            ],
        }
    }
}
#[derive(Copy, Clone)]
struct SearchState {
    time_remaining: u32,
    robots: [u32; 4],
    materials: [u32; 4],
}
fn optimistic_max(state: &SearchState, mat: Material) -> u32 {
    let mat = mat as usize;
    let i = state.time_remaining;
    // assume we can build a new robot every turn
    state.materials[mat] + state.robots[mat] * i + i * (i - 1) / 2
}
// Finds the maximum of any material needed in any recipe
fn get_max_materials(blueprint: &Blueprint) -> [u32; 4] {
    let mut maxs = [0, 0, 0, u32::MAX];

    for recipe in &blueprint.recipes {
        for &(amount, material) in recipe {
            let i = material as usize;
            maxs[i] = max(maxs[i], amount);
        }
    }

    return maxs;
}
impl SearchState {
    fn can_build_robot(
        &self,
        robot_type: usize,
        blueprint: &Blueprint,
        max_materials: &[u32],
    ) -> bool {
        let recipe = &blueprint.recipes[robot_type];

        let maxed_out = self.robots[robot_type] >= max_materials[robot_type];

        !maxed_out
            && recipe
                .iter()
                .all(|&(amt, mat)| self.materials[mat as usize] >= amt)
    }
    // Updates self to deduct materials and add one robot of the specified type
    // We assume that the robot can be built
    fn build_robot(&mut self, robot_type: usize, blueprint: &Blueprint) {
        self.robots[robot_type] += 1;
        for &(amount, material) in &blueprint.recipes[robot_type] {
            self.materials[material as usize] -= amount;
        }
    }
    fn unbuild_robot(&mut self, robot_type: usize, blueprint: &Blueprint) {
        self.robots[robot_type] -= 1;
        for &(amount, material) in &blueprint.recipes[robot_type] {
            self.materials[material as usize] += amount;
        }
    }
}
fn score(blueprint: &Blueprint, time_remaining: u32) -> u32 {
    let state = SearchState {
        time_remaining,
        robots: [1, 0, 0, 0],
        materials: [0, 0, 0, 0],
    };
    let max_materials = get_max_materials(blueprint);
    search(&state, blueprint, &max_materials, None, 0)
}
fn search(
    state: &SearchState,
    blueprint: &Blueprint,
    max_mat: &[u32],
    skipped: Option<&Vec<usize>>,
    best: u32,
) -> u32 {
    if state.time_remaining == 1 {
        return state.robots[3] + state.materials[3];
    }

    if optimistic_max(state, Geode) < best {
        return 0;
    }
    if optimistic_max(state, Obsidian) < max_mat[2] {
        return state.materials[3] + state.robots[3] * state.time_remaining;
    }

    let mut new_state = *state;
    new_state.time_remaining -= 1;
    (0..4).for_each(|i| new_state.materials[i] += new_state.robots[i]);

    if state.can_build_robot(Geode as usize, blueprint, max_mat) {
        new_state.build_robot(Geode as usize, blueprint);
        return search(&new_state, blueprint, max_mat, None, best);
    }
    let robots_available = (0..3)
        .filter(|i| state.can_build_robot(*i, blueprint, max_mat))
        .collect_vec();
    let mut best = best;

    for &robot in &robots_available {
        if skipped.map(|l| l.contains(&robot)).unwrap_or(false) {
            continue;
        }

        new_state.build_robot(robot, blueprint);
        best = max(best, search(&new_state, blueprint, max_mat, None, best));
        new_state.unbuild_robot(robot, blueprint);
    }
    let score = search(
        &new_state,
        blueprint,
        max_mat,
        Some(&robots_available),
        best,
    );
    best = max(score, best);

    best
}
#[aoc_generator(day19)]
fn parse(input: &str) -> Vec<Blueprint> {
    input.lines().map(Blueprint::from_line).collect()
}
#[aoc(day19, part1)]
fn part1(input: &[Blueprint]) -> u32 {
    input.par_iter().map(|bp| bp.id * score(bp, 24)).sum()
}
#[aoc(day19, part2)]
fn part2(input: &[Blueprint]) -> u32 {
    input[..3].par_iter().map(|bp| score(bp, 32)).product()
}
