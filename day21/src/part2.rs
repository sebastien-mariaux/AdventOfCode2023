use std::collections::{HashMap, HashSet};

use crate::utils::read_data;

pub fn solve_puzzle(file_name: &str, steps: u32) -> u32 {
    let data = read_data(file_name);

    let map = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start: (isize, isize) = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, cell)| {
                if *cell == 'S' {
                    Some((x as isize, y as isize))
                } else {
                    None
                }
            })
        })
        .unwrap();

    // let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut starts: HashSet<(isize, isize)> = HashSet::from([start]);

    for step in 0..steps {
        let mut next_starts = HashSet::new();
        // println!("Starts {:?}", starts);
        for start in &starts {
            let next_cells = get_next_cells(&map, *start);
            // visited.insert(*start);
            next_starts.extend(next_cells);
        }
        // println!("Diff {}", &next_starts.len() - starts.len());
        starts = next_starts;

        // Look for some kind of pattern
        // let mut stats = HashMap::new();
        // for start in &starts {
        //     let fixed_start = (start.0.rem_euclid(map.len() as isize), start.1.rem_euclid(map[0].len() as isize));
        //     let count = stats.entry(fixed_start).or_insert(0);
        //     *count += 1;
        // }
        // println!("Step {} starts {:?}, {}", step, stats.get(&(1,10)),  step.rem_euclid((stats.get(&(1,10)).unwrap_or(&1) as i32).try_into().unwrap() ));
    }

    starts.len() as u32
}

fn get_next_cells(
    map: &Vec<Vec<char>>,
    start: (isize, isize),
    // visited: &HashSet<(isize, isize)>
) -> Vec<(isize, isize)> {
    let mut next_cells: Vec<(isize, isize)> = Vec::new();

    let (i, j) = start;

    let up_coord = (i - 1, j);
    let up_value = map[(up_coord.0.rem_euclid(map.len() as isize)) as usize]
        [(up_coord.1.rem_euclid(map[0].len() as isize)) as usize];
    if up_value != '#' {
        next_cells.push(up_coord);
    }
    let down_coord = (i + 1, j);
    let down_value = map[(down_coord.0.rem_euclid(map.len() as isize)) as usize]
        [(down_coord.1.rem_euclid(map[0].len() as isize)) as usize];
    if down_value != '#' {
        next_cells.push(down_coord);
    }
    let left_coord = (i, j - 1);
    let left_value = map[(left_coord.0.rem_euclid(map.len() as isize)) as usize]
        [(left_coord.1.rem_euclid(map[0].len() as isize)) as usize];
    if left_value != '#' {
        next_cells.push(left_coord);
    }
    let right_coord = (i, j + 1);
    let right_value = map[(right_coord.0.rem_euclid(map.len() as isize)) as usize]
        [(right_coord.1.rem_euclid(map[0].len() as isize)) as usize];
    if right_value != '#' {
        next_cells.push(right_coord);
    }

    next_cells
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(16733044, solve_puzzle("test_data", 5000));
    }

    #[test]
    #[ignore]
    fn test_solution() {
        assert_eq!(0, solve_puzzle("input", 26501365));
    }
}
