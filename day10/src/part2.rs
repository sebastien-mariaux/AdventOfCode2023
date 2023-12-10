use crate::utils::read_data;
use core::panic;
use std::{collections::{HashMap, HashSet}, fs, };

pub fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);
    let map = data.lines().collect::<Vec<&str>>();

    let mut new_map: Vec<String> = Vec::new();
    let animal = get_animal(&map);

    let mut main_loop: HashSet<(usize, usize)> = HashSet::new();
    get_main_loop(&map, animal.0, animal.1, &mut main_loop);

    // Get NEW MAP
    for i in 0..map.len() {
        let target_row = i * 2;
        new_map.push(String::new()); // Current line with extra columns
        new_map.push(String::new()); // Line with new values
        for j in 0..map[i].len() {
            let current_value = if main_loop.contains(&(i, j)) {
                map[i].chars().nth(j).unwrap()
            } else {
                '.'
            };

            // Put current value
            new_map[target_row].push(current_value);

            // Insert new value to the right
            if j <= map[i].len() - 1 {
                let right_value = map[i].chars().nth(j + 1).unwrap_or(' ');
                match current_value {
                    'F' | 'S' | '-' | 'L' => {
                        if ['7', 'J', '-', 'S'].contains(&right_value) {
                            new_map[target_row].push('-');
                        } else {
                            new_map[target_row].push(' ');
                        }
                    }
                    '|' | '.' | 'J' | '7' => {
                        new_map[target_row].push(' ');
                    }
                    _ => panic!("Unknown combination: {} - {}", current_value, right_value),
                 }
            }

            if i == map.len() - 1 {
                continue;
            }

            // Insert new value to the bottom
            let bottom_value = map[i + 1].chars().nth(j).unwrap_or(' ');
            match current_value {
                'F' | 'S' | '|' | '7' => {
                    if ['L', '|', 'J', 'S'].contains(&bottom_value) {
                        new_map[target_row + 1].push('|');
                        new_map[target_row + 1].push(' ');
                    } else {
                        new_map[target_row + 1].push(' ');
                        new_map[target_row + 1].push(' ');
                    }
                }
                '-' | '.' | 'L' | 'J' => {
                    new_map[target_row + 1].push(' ');
                    new_map[target_row + 1].push(' ');
                }
                _ => panic!("Unknown combination: {} - {}", current_value, bottom_value),
            }
        }
    }

    // Remove last element of new_map (empty line)
    new_map.pop();
    for line in new_map.clone() {
        println!("{}", line);
    }


    // For each ground point on the new map, check if it can reach the border of the map
    let mut count = 0;
    let mut outside_points: HashSet<(usize, usize)> = HashSet::new();
    let mut inside_points: HashSet<(usize, usize)> = HashSet::new();
    // let mut visited_false: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..new_map.len() {
        for j in 0..new_map[i].len() {
            if is_inside_ground(&new_map, i, j, &mut outside_points, &mut inside_points) {
                count += 1;
            }
        }
    }


    count
}

fn is_inside_ground(new_map: &[String], i: usize, j: usize, outside_points: &mut HashSet<(usize, usize)>, inside_points:  &mut HashSet<(usize, usize)>   ) -> bool {
   let value = new_map[i].chars().nth(j).unwrap();
   if value != '.' {
       return false;
   }

   if i == 0 || i == new_map.len() - 1 || j == 0 || j == new_map[i].len() - 1 {
       return false;
   }

   if can_reach_border(new_map, i, j, outside_points, inside_points) {
       return false;
   }

   true
}

fn can_reach_border(new_map: &[String], i: usize, j: usize, outside_points: &mut HashSet<(usize, usize)>, inside_points: &mut HashSet<(usize, usize)>) -> bool {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push((i, j));
    println!("outside points length: {}", outside_points.len());
    println!("inside points length: {}", inside_points.len());

    while !stack.is_empty() {
        let current = stack.pop().unwrap();
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        // println!("Visited lenght: {}", visited.len());
        let neighbors = get_empty_neighbors(&new_map, current.0, current.1);
        for neighbor in neighbors {
            if inside_points.contains(&neighbor) {
                return false;
            }

            if outside_points.contains(&neighbor) || neighbor.0 == 0 || neighbor.0 == new_map.len() - 1 || neighbor.1 == 0 || neighbor.1 == new_map[neighbor.0].len() - 1 {
                outside_points.insert(neighbor);
                return true;
            }
            if !visited.contains(&neighbor) {
                stack.push(neighbor);
                // println!("Stack lenght: {}", stack.len());
            }
        }
    }
    inside_points.extend(visited);
    false
}

fn get_empty_neighbors(new_map: &[String], i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if i > 0 && ['.', ' '].contains(&new_map[i - 1].chars().nth(j).unwrap()) {
        neighbors.push((i - 1, j));
    }
    if j < new_map[i].len() - 1 && ['.', ' '].contains(&new_map[i].chars().nth(j + 1).unwrap()) {
        neighbors.push((i, j + 1));
    }
    if i < new_map.len() - 1 && ['.', ' '].contains(&new_map[i + 1].chars().nth(j).unwrap()) {
        neighbors.push((i + 1, j));
    }
    if j > 0 && ['.', ' '].contains(&new_map[i].chars().nth(j - 1).unwrap()) {
        neighbors.push((i, j - 1));
    }
    neighbors
}

fn get_main_loop(map: &[&str], i: usize, j: usize, main_loop: &mut HashSet<(usize, usize)>){
    let neighbors = get_neighbors(map, i, j);
    for neighbor in neighbors {
        if !main_loop.contains(&neighbor) {
            main_loop.insert(neighbor);
            get_main_loop(map, neighbor.0, neighbor.1, main_loop);
        }
    }

}

fn get_animal(map: &[&str]) -> (usize, usize) {
    for (i, line) in map.iter().enumerate() {

        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                return (i, j);
            }
        }
    }
    panic!("No animal found");
}

fn get_neighbors(map: &[&str], i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let current = map[i].chars().nth(j).unwrap();
    if i > 0 && ['S', 'L', 'J', '|'].contains(&current) {
        let top = map[i - 1].chars().nth(j).unwrap();
        if ['F', '|', '7', 'S'].contains(&top) {
            neighbors.push((i - 1, j));
        }
    }
    if j < map[i].len() - 1 && ['S', 'F', 'L', '-'].contains(&current){
        let right = map[i].chars().nth(j + 1).unwrap();
        if ['J', '-', '7', 'S'].contains(&right) {
            neighbors.push((i, j + 1));
        }
    }
    if i < map.len() - 1 && ['S', 'F', '7', '|'].contains(&current){
        let bottom = map[i + 1].chars().nth(j).unwrap();
        if ['L', '|', 'J', 'S'].contains(&bottom) {
            neighbors.push((i + 1, j));
        }
    }
    if j > 0 && ['S', '7', 'J', '-'].contains(&current) {
        let left = map[i].chars().nth(j - 1).unwrap();
        if ['F', '-', 'L', 'S'].contains(&left) {
            neighbors.push((i, j - 1));
        }
    }
    if  neighbors.len() !=2 {
        panic!("There is only two directions in a pipe!");
    }
    neighbors
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(10, solve_puzzle("test_data"));
    }

    #[test]
    #[ignore]
    fn test_solution() {
        assert_eq!(0, solve_puzzle("input"));
    }
}
