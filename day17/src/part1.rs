use std::collections::{HashMap, HashSet};

use crate::utils::read_data;

pub fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);

    let map = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let exit_cell = (map.len() - 1, map[0].len() - 1);
    // min value ever seen to reach a particular block
    let mut minblock: HashMap<(usize, usize), u32> = HashMap::new();
    minblock.insert((0, 0), 0);
    minblock.insert(exit_cell, u32::MAX);

    // Paths to visit
    let mut paths: Vec<Vec<(usize, usize, char, u32)>> = Vec::new();
    paths.push(vec![(0, 0, 'R', 0)]);

    while !paths.is_empty() {
        let current_path = paths.pop().unwrap();
        let (i, j, dir, heat) = current_path.last().unwrap().clone();
        let next_directions = get_next_directions(dir, can_continue_straight(&current_path));
        for next_direction in next_directions {
            match next_direction {
                'L' => {
                    if j > 0 {
                        let new_heat = heat + map[i][j - 1];
                        if new_heat > *minblock.get(&(i, j - 1)).unwrap_or(&u32::MAX)
                            || &new_heat >= minblock.get(&exit_cell).unwrap_or(&u32::MAX)
                        {
                            continue;
                        }
                        minblock.insert((i, j - 1), new_heat);
                        if (i, j - 1) == exit_cell {
                            println!(
                                "Current min value: {} - stack len {}",
                                minblock.get(&exit_cell).unwrap_or(&u32::MAX),
                                paths.len()
                            );

                            continue;
                        }
                        let mut new_path = current_path.clone();
                        new_path = new_path
                            .iter()
                            .rev()
                            .take(2)
                            .rev()
                            .map(|(i, j, d, h)| (*i, *j, *d, *h))
                            .collect();
                        new_path.push((i, j - 1, 'L', new_heat));

                        paths.push(new_path);
                    }
                }
                'U' => {
                    if i > 0 {
                        let new_heat = heat + map[i - 1][j];
                        if new_heat > *minblock.get(&(i - 1, j)).unwrap_or(&u32::MAX)
                            || &new_heat >= minblock.get(&exit_cell).unwrap_or(&u32::MAX)
                        {
                            continue;
                        }
                        minblock.insert((i - 1, j), new_heat);
                        if (i - 1, j) == exit_cell {
                            println!(
                                "Current min value: {} - stack len {}",
                                minblock.get(&exit_cell).unwrap_or(&u32::MAX),
                                paths.len()
                            );

                            continue;
                        }
                        let mut new_path = current_path.clone();
                        new_path = new_path
                            .iter()
                            .rev()
                            .take(2)
                            .rev()
                            .map(|(i, j, d, h)| (*i, *j, *d, *h))
                            .collect();
                        new_path.push((i - 1, j, 'U', new_heat));

                        paths.push(new_path);
                    }
                }
                'R' => {
                    if j < map[i].len() - 1 {
                        let new_heat = heat + map[i][j + 1];
                        if new_heat > *minblock.get(&(i, j + 1)).unwrap_or(&u32::MAX)
                            || &new_heat >= minblock.get(&exit_cell).unwrap_or(&u32::MAX)
                        {
                            continue;
                        }
                        minblock.insert((i, j + 1), new_heat);
                        if (i, j + 1) == exit_cell {
                            println!(
                                "Current min value: {} - stack len {}",
                                minblock.get(&exit_cell).unwrap_or(&u32::MAX),
                                paths.len()
                            );

                            continue;
                        }
                        let mut new_path = current_path.clone();
                        new_path = new_path
                            .iter()
                            .rev()
                            .take(2)
                            .rev()
                            .map(|(i, j, d, h)| (*i, *j, *d, *h))
                            .collect();
                        new_path.push((i, j + 1, 'R', new_heat));

                        paths.push(new_path);
                    }
                }
                'D' => {
                    if i < map.len() - 1 {
                        let new_heat = heat + map[i + 1][j];
                        if new_heat > *minblock.get(&(i + 1, j)).unwrap_or(&u32::MAX)
                            || &new_heat >= minblock.get(&exit_cell).unwrap_or(&u32::MAX)
                        {
                            continue;
                        }
                        minblock.insert((i + 1, j), new_heat);
                        if (i + 1, j) == exit_cell {
                            println!(
                                "Current min value: {} - stack len {}",
                                minblock.get(&exit_cell).unwrap_or(&u32::MAX),
                                paths.len()
                            );

                            continue;
                        }
                        let mut new_path = current_path.clone();
                        new_path = new_path
                            .iter()
                            .rev()
                            .take(2)
                            .rev()
                            .map(|(i, j, d, h)| (*i, *j, *d, *h))
                            .collect();
                        new_path.push((i + 1, j, 'D', new_heat));

                        paths.push(new_path);
                    }
                }
                _ => panic!("Unknown direction"),
            }
        }
    }

    minblock
        .get(&(&map.len() - 1, &map[0].len() - 1))
        .unwrap()
        .clone()
}

fn can_continue_straight(path: &Vec<(usize, usize, char, u32)>) -> bool {
    if path.len() < 3 {
        return true;
    }
    let unique_values: HashSet<&char> = HashSet::from_iter(path.iter().map(|(_, _, d, _)| d));
    unique_values.len() > 1
}

fn get_next_directions(dir: char, can_continue_straight: bool) -> Vec<char> {
    let mut next_directions = Vec::new();
    match dir {
        'R' => {
            next_directions.push('U');
            next_directions.push('D');
            if can_continue_straight {
                next_directions.push('R');
            }
        }
        'L' => {
            next_directions.push('U');
            next_directions.push('D');
            if can_continue_straight {
                next_directions.push('L');
            }
        }
        'U' => {
            next_directions.push('L');
            next_directions.push('R');
            if can_continue_straight {
                next_directions.push('U');
            }
        }
        'D' => {
            next_directions.push('L');
            next_directions.push('R');
            if can_continue_straight {
                next_directions.push('D');
            }
        }
        _ => panic!("Unknown direction"),
    }
    next_directions
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(102, solve_puzzle("test_data"));
    }

    #[test]
    // #[ignore]
    fn test_solution() {
        assert_eq!(0, solve_puzzle("input"));
    }

    #[test]
    fn test_can_continue_straight_false() {
        assert_eq!(
            false,
            can_continue_straight(&vec![(0, 0, 'R', 1), (0, 1, 'R', 1), (0, 2, 'R', 1)])
        );
        assert_eq!(
            false,
            can_continue_straight(&vec![(0, 0, 'U', 1), (0, 1, 'U', 1), (0, 2, 'U', 1)])
        );
        assert_eq!(
            false,
            can_continue_straight(&vec![(0, 0, 'D', 1), (0, 1, 'D', 1), (0, 2, 'D', 1)])
        );
        assert_eq!(
            false,
            can_continue_straight(&vec![(0, 0, 'L', 1), (0, 1, 'L', 1), (0, 2, 'L', 1)])
        );
    }

    #[test]
    fn test_can_continue_straight_true() {
        assert_eq!(
            true,
            can_continue_straight(&vec![(0, 0, 'L', 1), (0, 1, 'R', 1), (0, 2, 'R', 1)])
        );
        assert_eq!(
            true,
            can_continue_straight(&vec![(0, 0, 'U', 1), (0, 1, 'U', 1), (0, 2, 'R', 1)])
        );
        assert_eq!(
            true,
            can_continue_straight(&vec![(0, 0, 'L', 1), (0, 1, 'D', 1), (0, 2, 'D', 1)])
        );
        assert_eq!(
            true,
            can_continue_straight(&vec![(0, 0, 'L', 1), (0, 1, 'U', 1), (0, 2, 'R', 1)])
        );
    }

    #[test]
    fn test_can_continue_straight_true_short_path() {
        assert_eq!(
            true,
            can_continue_straight(&vec![(0, 0, 'L', 1), (0, 1, 'L', 1)])
        );
        assert_eq!(true, can_continue_straight(&vec![(0, 0, 'L', 1)]));
    }
}
