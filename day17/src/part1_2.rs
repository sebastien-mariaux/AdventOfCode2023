// use crate::utils::read_data;

// pub fn solve_puzzle(file_name: &str) -> usize {
//     let data = read_data(file_name);

//     let map = data
//         .lines()
//         .map(|line| {
//             line.chars()
//                 .map(|x| x.to_digit(10).unwrap())
//                 .collect::<Vec<u32>>()
//         })
//         .collect::<Vec<Vec<u32>>>();

//     let exit = (map.len() - 1, map[0].len() - 1);

//     let mut stack:Vec<(usize, usize, char)> = Vec::new();
//     stack.extend(get_previous(exit, &map));

//     while !stack.is_empty() {
//         let current_cell = stack.pop().unwrap();
//         let previous_cells = get_previous(current_cell, &map);
//         for previous_cell in previous_cells {
//             stack.push(previous_cell);
//         }
//     }


//     1
// }

// fn get_previous(cell: (usize, usize), map: &Vec<Vec<u32>>) -> Vec<(usize, usize, char)> {
//     let mut previous: Vec<(usize, usize, char)> = Vec::new();
//     if cell.0 > 0 {
//         previous.push((cell.0 - 1, cell.1, 'D'));
//     }
//     if cell.1 > 0 {
//         previous.push((cell.0, cell.1 - 1, 'R'));
//     }
//     if cell.0 < map.len() - 1 {
//         previous.push((cell.0 + 1, cell.1, 'U'));
//     }
//     if cell.1 < map[0].len() - 1 {
//         previous.push((cell.0, cell.1 + 1, 'L'));
//     }
//     previous
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     #[ignore]
//     fn test_example_data() {
//         assert_eq!(0, solve_puzzle("test_data"));
//     }

//     #[test]
//     #[ignore]
//     fn test_solution() {
//         assert_eq!(0, solve_puzzle("input"));
//     }
// }
