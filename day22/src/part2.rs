use std::collections::HashMap;
use std::collections::HashSet;

use crate::utils::read_data;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Cube {
    index: u32,
    x: u32,
    y: u32,
    z: u32,
    xx: u32,
    yy: u32,
    zz: u32,
}

impl Cube {
    fn new(index: u32, x: u32, y: u32, z: u32, xx: u32, yy: u32, zz: u32) -> Self {
        Self {
            index,
            x,
            y,
            z,
            xx,
            yy,
            zz,
        }
    }

    pub fn has_settled(&self, cubes: &[Cube]) -> bool {
        self.z == 1
            || cubes
                .iter()
                .any(|other| other.zz == self.z - 1 && self.superposed(other))
    }

    fn superposed(&self, other: &Cube) -> bool {
        !(other.x > self.xx || other.xx < self.x || other.y > self.yy || other.yy < self.y)
    }

    fn settle_at_z(&mut self, z: u32) {
        let height = self.zz - self.z;
        self.z = z;
        self.zz = z + height;
    }

    fn next_support_level(&self, cubes: &[Cube]) -> u32 {
        cubes
            .iter()
            .filter(|other| other.zz < self.z && self.superposed(other))
            .map(|other| other.zz)
            .max()
            .unwrap_or(0)
            + 1
    }

    pub fn settle_at_next_support_level(&mut self, cubes: &[Cube]) {
        if self.has_settled(cubes) {
            return;
        }
        self.settle_at_z(self.next_support_level(cubes));
    }

    fn dependencies(&self, cubes: &[Cube]) -> Vec<u32> {
        cubes
            .iter()
            .filter(|other| other.zz == self.z - 1 && self.superposed(other))
            .map(|other| other.index)
            .collect::<Vec<u32>>()
    }
}

impl Ord for Cube {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z.cmp(&other.z)
    }
}

impl PartialOrd for Cube {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.z == other.z
    }
}

impl Eq for Cube {}

pub fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);

    let mut cubes = data
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (start, end) = line.split_once('~').unwrap();
            let (x, y, z) = start
                .splitn(3, ',')
                .map(|el| el.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();
            let (xx, yy, zz) = end
                .splitn(3, ',')
                .map(|el| el.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();
            Cube::new(i as u32, x, y, z, xx, yy, zz)
        })
        .collect::<Vec<Cube>>();

    cubes.sort();

    for i in 0..cubes.len() {
        let cubes_ref = cubes.clone();
        let cube = &mut cubes.get_mut(i).unwrap();
        cube.settle_at_next_support_level(&cubes_ref);
    }

    let dependencies = cubes
        .iter()
        .map(|cube| (cube.index, cube.dependencies(&cubes)))
        .collect::<HashMap<u32, Vec<u32>>>();

    cubes
        .iter()
        .map(|c| destroy_cube(c.index, &dependencies))
        .sum()
}

fn destroy_cube(cube: u32, dependencies: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut destroyed: HashSet<u32> = HashSet::new();
    let mut stack = vec![cube];

    while let Some(current_cube) = stack.pop() {
        let current_index = current_cube;
        destroyed.insert(current_cube);

        let dependent_indexes = dependencies
            .iter()
            .filter(|(_k, v)| v.contains(&current_index))
            .map(|(k, _v)| k)
            .collect::<Vec<&u32>>();

        for d in dependent_indexes {
            if dependencies
                .get(d)
                .unwrap()
                .iter()
                .all(|i| destroyed.contains(i))
            {
                stack.push(*d);
            }
        }
    }

    destroyed.len() as u32 - 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(7, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(101541, solve_puzzle("input"));
    }
}
