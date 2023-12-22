use crate::utils::read_data;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Cube {
    x: u32,
    y: u32,
    z: u32,
    xx: u32,
    yy: u32,
    zz: u32,
}

impl Cube {
    fn new(x: u32, y: u32, z: u32, xx: u32, yy: u32, zz: u32) -> Self {
        Self {
            x,
            y,
            z,
            xx,
            yy,
            zz,
        }
    }

    pub fn has_settled(&self, cubes: &Vec<Cube>) -> bool {
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

    fn next_support_level(&self, cubes: &Vec<Cube>) -> u32 {
        cubes
            .iter()
            .filter(|other| other.zz < self.z && self.superposed(other))
            .map(|other| other.zz)
            .max()
            .unwrap_or(0)
            + 1
    }

    pub fn settle_at_next_support_level(&mut self, cubes: &Vec<Cube>) {
        if self.has_settled(cubes) {
            return;
        }
        self.settle_at_z(self.next_support_level(cubes));
    }

    fn has_several_support(&self, cubes: &Vec<Cube>) -> bool {
        cubes
            .iter()
            .filter(|other| other.zz == self.z - 1 && self.superposed(other))
            .count()
            > 1
    }

    pub fn can_destroy(&self, cubes: &Vec<Cube>) -> bool {
        if !cubes.iter().any(|other| other.z == self.z + 1 && self.superposed(other)) {
            return true;
        }
        let supported_cubes = cubes
            .iter()
            .filter(|other| other.z == self.zz + 1 && self.superposed(other))
            .collect::<Vec<&Cube>>();
        if supported_cubes.iter().all(|cube| cube.has_several_support(cubes)) {
            return true;
        }
        false
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
        .map(|line| {
            println!("{}", line);
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
            Cube::new(x, y, z, xx, yy, zz)
        })
        .collect::<Vec<Cube>>();

    cubes.sort();
    println!("{:?}", cubes);

    for i in 0..cubes.len() {
        let cubes_ref = cubes.clone();
        let cube = &mut cubes.get_mut(i).unwrap();
        cube.settle_at_next_support_level(&cubes_ref);
    }
    println!("{:?}", cubes);

    let cubes_copy = cubes.clone();
    cubes.iter().filter(|c| c.can_destroy(&cubes_copy)).count() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(5, solve_puzzle("test_data"));
    }

    #[test]
    // #[ignore]
    fn test_solution() {
        assert_eq!(0, solve_puzzle("input"));
    }
}
