use std::collections::{HashMap, HashSet};

use crate::utils::read_data;

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);

    let map = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let exit_cell = (map.len() - 1, map[0].len() - 1);

    // i, j, direction, steps_count, heat
    let start = (0, 0, 'S', 0, 0);
    let mut min_value = usize::MAX;

    let mut visited = HashMap::new();
    let mut stack = Vec::new();
    stack.push(start);

    while !stack.is_empty() {
        if min_value < usize::MAX {
            stack = stack.iter().filter(|(_, _, _, _, heat)| heat < &min_value).cloned().collect();
        }
        stack.sort_by(|(_, _, _, _, heat1), (_, _, _, _, heat2)| heat2.cmp(heat1));
        println!("Stack : {} - Min: {}", stack.len(), min_value);
        // let next_position = stack.iter().position(|x| x.2 == 'R' || x.2 == 'D');

        // let (i, j, dir, steps_count, heat) = if let Some(next_position) = next_position {
        //     stack.remove(next_position)
        // } else {
        //     stack.pop().unwrap()
        // };

        let (ii, jj, diir, sterps_count, heeat) = stack.pop().unwrap();
        if heeat >= min_value {
            continue;
        }

        let next_points = get_next_points(&map, ii, jj, diir, sterps_count, heeat, &min_value);
        for (i, j, dir, steps_count, heat) in next_points {
            if (i, j) == exit_cell && steps_count >= 4 {
                min_value = min_value.min(heat);
                println!("Min value: {}", min_value);
            }
            if let Some(visited_heat) = visited.get(&(i, j, dir, steps_count)) {
                if heat >= *visited_heat {
                    continue;
                }
            }
            stack.push((i, j, dir, steps_count, heat));
            visited.insert((i, j, dir, steps_count), heat);
        }


    }

 min_value
}

fn get_next_points(map: &Vec<Vec<usize>>, i: usize, j: usize, dir: char, steps_count: usize, heat: usize, min_value: &usize) -> Vec<(usize, usize, char, usize, usize)> {
    let mut next_points = Vec::new();

    let can_turn = steps_count >= 4;
    let can_continue_straight = steps_count < 10;

    match dir {
        'R' => {
            if can_turn {
                if i > 0 {
                    let next_value = heat + map[i as usize - 1][j as usize];
                    if next_value < *min_value {
                        next_points.push((i - 1, j, 'U', 1, next_value));
                    }
                }
                if i < map.len() as usize - 1 {
                    let next_value = heat + map[i as usize + 1][j as usize];
                    if next_value < *min_value {
                        next_points.push((i + 1, j, 'D', 1, next_value));
                    }
                }
            }
            if can_continue_straight {
                if j < map[0].len() as usize - 1 {
                    let next_value = heat + map[i as usize][j as usize + 1];
                    if next_value < *min_value {
                        next_points.push((i, j + 1, 'R', steps_count + 1, next_value));
                    }
                }
            }
        }
        'L' => {
            if can_continue_straight {
                if j > 0 {
                    let next_value = heat + map[i as usize][j as usize - 1];
                    if next_value < *min_value {
                        next_points.push((i, j - 1, 'L', steps_count + 1, next_value));
                    }
                }
            }
            if can_turn {
                if i > 0 {
                    let next_value = heat + map[i as usize - 1][j as usize];
                    if next_value < *min_value {
                        next_points.push((i - 1, j, 'U', 1, next_value));
                    }
                }
                if i < map.len() as usize - 1 {
                    let next_value = heat + map[i as usize + 1][j as usize];
                    if next_value < *min_value {
                        next_points.push((i + 1, j, 'D', 1, next_value));
                    }
                }
            }
        }
        'U' => {
            if can_continue_straight {
                if i > 0 {
                    let next_value = heat + map[i as usize - 1][j as usize];
                    if next_value < *min_value {
                        next_points.push((i - 1, j, 'U', steps_count + 1, next_value));
                    }
                }
            }
            if can_turn {
                if j > 0 {
                    let next_value = heat + map[i as usize][j as usize - 1];
                    if next_value < *min_value {
                        next_points.push((i, j - 1, 'L', 1, next_value));
                    }
                }
                if j < map[0].len() as usize - 1 {
                    let next_value = heat + map[i as usize][j as usize + 1];
                    if next_value < *min_value {
                        next_points.push((i, j + 1, 'R', 1, next_value));
                    }
                }
            }
        }
        'D' => {
            if can_turn {
                if j > 0 {
                    let next_value = heat + map[i as usize][j as usize - 1];
                    if next_value < *min_value {
                        next_points.push((i, j - 1, 'L', 1, next_value));
                    }
                }
                if j < map[0].len() as usize - 1 {
                    let next_value = heat + map[i as usize][j as usize + 1];
                    if next_value < *min_value {
                        next_points.push((i, j + 1, 'R', 1, next_value));
                    }
                }
            }
            if can_continue_straight {
                if i < map.len() as usize - 1 {
                    let next_value = heat + map[i as usize + 1][j as usize];
                    if next_value < *min_value {
                        next_points.push((i + 1, j, 'D', steps_count + 1, next_value));
                    }
                }
            }
        }
        // Starting point
        'S' => {
            next_points.push((i, j+1, 'R', 1, heat + map[i as usize][j as usize + 1]));
            next_points.push((i+1, j, 'D', 1, heat + map[i as usize + 1][j as usize]));
        }
        _ => panic!("Unknown direction"),
    }

    next_points
}

// fn can_continue_straight(path: &Vec<(usize, usize, char, usize)>) -> bool {
//     if path.len() < 10 {
//         return true;
//     }
//     let unique_values: HashSet<&char> = HashSet::from_iter(path.iter().map(|(_, _, d, _)| d));
//     unique_values.len() > 1
// }

// fn can_turn(path: &Vec<(usize, usize, char, usize)>) -> bool {
//     if path.len() < 4 {
//         return false;
//     }
//     let last_directions = path
//         .iter()
//         .rev()
//         .take(4)
//         .map(|(_, _, d, _)| d)
//         .collect::<Vec<&char>>();
//     let unique_values: HashSet<&char> = HashSet::from_iter(last_directions);
//     unique_values.len() == 1

// }

// fn get_next_directions(dir: char, can_continue_straight: bool, can_turn: bool) -> Vec<char> {
//     let mut next_directions = Vec::new();
//     match dir {
//         'R' => {
//             if can_turn {
//                 next_directions.push('U');
//                 next_directions.push('D');
//             }
//             if can_continue_straight {
//                 next_directions.push('R');
//             }
//         }
//         'L' => {
//             if can_continue_straight {
//                 next_directions.push('L');
//             }
//             if can_turn {
//                 next_directions.push('U');
//                 next_directions.push('D');
//             }
//         }
//         'U' => {
//             if can_continue_straight {
//                 next_directions.push('U');
//             }
//             if can_turn {
//                 next_directions.push('L');
//                 next_directions.push('R');
//             }
//         }
//         'D' => {
//             if can_turn{
//                 next_directions.push('L');
//                 next_directions.push('R');
//             }
//             if can_continue_straight {
//                 next_directions.push('D');
//             }
//         }
//         // Starting point
//         'S' => {
//             next_directions.push('R');
//             next_directions.push('D');
//         }
//         _ => panic!("Unknown direction"),
//     }
//     next_directions
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(94, solve_puzzle("test_data"));
    }

    #[test]
    fn test_example_data_2() {
        assert_eq!(71, solve_puzzle("test_data_2"));
    }

    #[test]
    #[ignore]
    fn test_solution() {
        assert_eq!(1037, solve_puzzle("input"));
    }

}

// 1032