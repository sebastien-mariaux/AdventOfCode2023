use crate::utils::read_data;
use std::{collections::{HashSet, HashMap}, vec};

pub fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);
    let contraption = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut visited: HashMap<(usize, usize, char), HashSet<(usize, usize)>> = HashMap::new();
    let mut beams: Vec<(usize, usize, char)> = Vec::new(); // Next pos row,  next pos col, next pos direction
    beams.push((0, 0, '>'));

    let result = get_energized(0, 0, '>', &contraption, &mut visited);

    result.len() as u32
}

fn get_energized(i: usize, j: usize, d: char, contraption: &Vec<Vec<char>>, mut visited: &mut HashMap<(usize, usize, char), HashSet<(usize, usize)>>) -> HashSet<(usize, usize)> {
    if visited.contains_key(&(i, j, d)) {
        return visited.get(&(i, j, d)).unwrap().clone();
    }

    let mut energized: HashSet<(usize, usize)> = HashSet::new();
    let next_cells = get_next_cells(i, j, d, &contraption);
    for next_cell in next_cells {
        energized.extend(get_energized(next_cell.0, next_cell.1, next_cell.2, &contraption, &mut visited));
    }
    visited.insert((i, j, d), energized.clone());
    energized

}

fn get_next_cells(
    i: usize,
    j: usize,
    entry_direction: char,
    contraption: &Vec<Vec<char>>,
) -> Vec<(usize, usize, char)> {
    let current_cell = contraption[i][j];
    let exit_directions = get_exit_directions(current_cell, entry_direction);
    let mut next_cells: Vec<(usize, usize, char)> = Vec::new();
    for exit_direction in exit_directions {
        if let Some(next_cell) = get_next_cell(i, j, exit_direction, contraption) {
            next_cells.push(next_cell);
        }
    }
    next_cells
}

fn get_next_cell(
    i: usize,
    j: usize,
    exit_direction: char,
    contraption: &Vec<Vec<char>>,
) -> Option<(usize, usize, char)> {
    match exit_direction {
        '>' => {
            if j < contraption[i].len() - 1 {
                Some((i, j + 1, exit_direction))
            } else {
                None
            }
        }
        '<' => {
            if j > 0 {
                Some((i, j - 1, exit_direction))
            } else {
                None
            }
        }
        '^' => {
            if i > 0 {
                Some((i - 1, j, exit_direction))
            } else {
                None
            }
        }
        'v' => {
            if i < contraption.len() - 1 {
                Some((i + 1, j, exit_direction))
            } else {
                None
            }
        }
        _ => panic!("Invalid exit_direction: {}", exit_direction),
    }
}

fn get_exit_directions(current_cell: char, entry_direction: char) -> Vec<char> {
    match entry_direction {
        '>' => match current_cell {
            '/' => vec!['^'],
            '\\' => vec!['v'],
            '|' => vec!['^', 'v'],
            _ => vec!['>'],
        },
        '<' => match current_cell {
            '/' => vec!['v'],
            '\\' => vec!['^'],
            '|' => vec!['^', 'v'],
            _ => vec!['<'],
        },
        '^' => match current_cell {
            '/' => vec!['>'],
            '\\' => vec!['<'],
            '-' => vec!['<', '>'],
            _ => vec!['^'],
        },
        'v' => match current_cell {
            '/' => vec!['<'],
            '\\' => vec!['>'],
            '-' => vec!['<', '>'],
            _ => vec!['v'],
        },
        _ => panic!("Invalid entry_direction: {}", entry_direction),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(46, solve_puzzle("test_data"));
    }

    #[test]
    // #[ignore]
    fn test_solution() {
        assert_eq!(7951, solve_puzzle("input"));
    }
}

// empty space .
// mirrors / and \
// splitters | and -
