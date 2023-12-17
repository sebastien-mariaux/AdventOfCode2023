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
    // minblock.insert(exit_cell, 1248);

    // let mut visited_cells: HashMap<(usize, usize), u32> = HashMap::new();

    let mut visited: HashMap<Vec<(usize, usize)>, u32> = HashMap::new();
    // Paths to visit
    let mut paths: Vec<Vec<(usize, usize, char, u32)>> = Vec::new();
    paths.push(vec![(0, 0, 'S', 0)]);

    while !paths.is_empty() {
        let current_path = paths.pop().unwrap();
        let (i, j, dir, heat) = current_path.last().unwrap().clone();

        // compare with existing min value
        let min_heat = *minblock.get(&exit_cell).unwrap_or(&u32::MAX);
        if heat >= min_heat {
            continue;
        }

        // Check if path already visited with lower heat
        let path_without_heat = current_path
            .iter()
            .map(|(i, j, _, _)| (*i, *j))
            .collect::<Vec<(usize, usize)>>();
        let min_heat_for_path = visited.get(&path_without_heat).unwrap_or(&u32::MAX);
        if heat >= *min_heat_for_path {
            continue;
        }
        visited.insert(path_without_heat, heat);

        let can_continue_straight = can_continue_straight(&current_path);
        let can_turn = can_turn(&current_path);
        // check if cell already visited
        // if can_continue_straight {
        //     if visited_cells.get(&(i, j)).unwrap_or(&u32::MAX) < &heat {
        //         continue;
        //     }
        //     visited_cells.insert((i, j), heat);
        // }
        // Continuing path...
        let next_directions = get_next_directions(dir, can_continue_straight, can_turn);
        for next_direction in next_directions {
            match next_direction {
                'L' => {
                    if j > 0 {
                        let new_heat = heat + map[i][j - 1];
                        if new_heat >= min_heat {
                            continue;
                        }
                        // if can_continue_straight {
                        //     if visited_cells.get(&(i, j - 1)).unwrap_or(&u32::MAX) < &new_heat {
                        //         continue;
                        //     }
                        //     visited_cells.insert((i, j - 1), new_heat);
                        // }
                        minblock.insert((i, j - 1), new_heat);

                        let mut new_path = current_path.clone();
                        if new_path.len() == 10 {
                            new_path.remove(0);
                        }
                        new_path.push((i, j - 1, 'L', new_heat));

                        paths.push(new_path);
                    }
                }
                'U' => {
                    if i > 0 {
                        let new_heat = heat + map[i - 1][j];
                        if new_heat >= min_heat {
                            continue;
                        }
                        // if can_continue_straight {
                        //     if visited_cells.get(&(i-1, j )).unwrap_or(&u32::MAX) < &new_heat {
                        //         continue;
                        //     }
                        //     visited_cells.insert((i-1, j), new_heat);
                        // }
                        minblock.insert((i - 1, j), new_heat);

                        let mut new_path = current_path.clone();
                        if new_path.len() == 10 {
                            new_path.remove(0);
                        }
                        new_path.push((i - 1, j, 'U', new_heat));

                        paths.push(new_path);
                    }
                }
                'R' => {
                    if j < map[i].len() - 1 {
                        let new_heat = heat + map[i][j + 1];
                        if new_heat >= min_heat {
                            continue;
                        }
                        // if can_continue_straight {
                        //     if visited_cells.get(&(i, j+1)).unwrap_or(&u32::MAX) < &new_heat {
                        //         continue;
                        //     }
                        //     visited_cells.insert((i, j+1), new_heat);
                        // }
                        minblock.insert((i, j + 1), new_heat);
                        if (i, j + 1) == exit_cell {
                            println!(
                                "Current min value: {} - stack len {}",
                                min_heat,
                                paths.len()
                            );

                            continue;
                        }
                        let mut new_path = current_path.clone();
                        if new_path.len() == 10 {
                            new_path.remove(0);
                        }
                        new_path.push((i, j + 1, 'R', new_heat));

                        paths.push(new_path);
                    }
                }
                'D' => {
                    if i < map.len() - 1 {
                        let new_heat = heat + map[i + 1][j];
                        if new_heat >= min_heat {
                            continue;
                        }
                        // if can_continue_straight {
                        //     if visited_cells.get(&(i+1, j)).unwrap_or(&u32::MAX) < &new_heat {
                        //         continue;
                        //     }
                        //     visited_cells.insert((i+1, j), new_heat);
                        // }
                        minblock.insert((i + 1, j), new_heat);
                        if (i + 1, j) == exit_cell {
                            println!(
                                "Current min value: {} - stack len {}",
                                min_heat,
                                paths.len()
                            );

                            continue;
                        }
                        let mut new_path = current_path.clone();
                        if new_path.len() == 10 {
                            new_path.remove(0);
                        }
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
    if path.len() < 10 {
        return true;
    }
    let unique_values: HashSet<&char> = HashSet::from_iter(path.iter().map(|(_, _, d, _)| d));
    unique_values.len() > 1
}

fn can_turn(path: &Vec<(usize, usize, char, u32)>) -> bool {
    if path.len() < 4 {
        return false;
    }
    let last_directions = path
        .iter()
        .rev()
        .take(4)
        .map(|(_, _, d, _)| d)
        .collect::<Vec<&char>>();
    let unique_values: HashSet<&char> = HashSet::from_iter(last_directions);
    unique_values.len() == 1

}

fn get_next_directions(dir: char, can_continue_straight: bool, can_turn: bool) -> Vec<char> {
    let mut next_directions = Vec::new();
    match dir {
        'R' => {
            if can_turn {
                next_directions.push('U');
                next_directions.push('D');
            }
            if can_continue_straight {
                next_directions.push('R');
            }
        }
        'L' => {
            if can_continue_straight {
                next_directions.push('L');
            }
            if can_turn {
                next_directions.push('U');
                next_directions.push('D');
            }
        }
        'U' => {
            if can_continue_straight {
                next_directions.push('U');
            }
            if can_turn {
                next_directions.push('L');
                next_directions.push('R');
            }
        }
        'D' => {
            if can_turn{
                next_directions.push('L');
                next_directions.push('R');
            }
            if can_continue_straight {
                next_directions.push('D');
            }
        }
        // Starting point
        'S' => {
            next_directions.push('R');
            next_directions.push('D');
        }
        _ => panic!("Unknown direction"),
    }
    next_directions
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // #[ignore]
    fn test_example_data() {
        assert_eq!(94, solve_puzzle("test_data"));
    }

    #[test]
    #[ignore]
    fn test_solution() {
        assert_eq!(0, solve_puzzle("input"));
    }

}
