use std::collections::HashSet;

use crate::utils::read_data;

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);

    let mut plateform = data.lines().map(|l | l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let tilted_north = tilt_north(plateform);


    tilted_north.iter().rev().enumerate().map(|(y, line)| {
        line.iter().filter(|c| c == &&'O').count() * (y + 1)
    }).sum()
}

fn tilt_north(plateform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    //  Vec of same dimensions with only points
    let rows_count = plateform.len();
    let cols_count = plateform[0].len();
    let mut tilted_north = vec![vec!['.'; cols_count]; rows_count];

    for (i, row) in plateform.iter().enumerate() {
        for (j,  char) in row.iter().enumerate() {
            if char == &'O' {
                let new_position = get_new_position(&plateform, i, j);
                println!("{} {} -> {} {}", i, j, new_position.0, new_position.1);
                tilted_north[new_position.0][new_position.1] = 'O';
            } else if char == &'#' {
                tilted_north[i][j] = '#';
            }
        }
    }
    for row in tilted_north.iter() {
        println!("{:?}", row);
    }
    tilted_north
}

fn get_new_position(plateform: &Vec<Vec<char>>, i: usize, j:usize) -> (usize, usize) {
    let current_col = plateform.iter().map(|row| row[j]).collect::<Vec<char>>();
    println!("{}", current_col[0..i].iter().collect::<String>());
    let closest_cube = current_col[0..i].iter().enumerate().filter(|(_, c)| c == &&'#')
    .map(|(ix, _)| ix).max();
    match closest_cube {
        Some(pos) => {
            println!("closest cube {}", pos);
            let rounded_below = current_col[pos..i].iter().filter(|c| c == &&'O').count();
            return (pos + rounded_below + 1, j)
        },
        None => {
            let rounded_below = current_col[0..i].iter().filter(|c| c == &&'O').count();
            return (rounded_below, j)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(136, solve_puzzle("test_data"));
    }

    #[test]
    // #[ignore]
    fn test_solution() {
        assert_eq!(0, solve_puzzle("input"));
    }
}
