use crate::utils::read_data;
use std::collections::HashMap;

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);

    let map = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let exit = (map.len() - 1, map[0].len() - 1);

    // create table with same dimension with value u32::MAX
    let mut inactive = Vec::new();
    let paths: HashMap<(usize, usize), Vec<char>> = HashMap::new();
    let mut dijkstra = HashMap::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            dijkstra.insert((i, j), u32::MAX);
        }
    }
    dijkstra.insert((0, 0), 0);

    while !inactive.contains(&exit) {
        let next_start = dijkstra
            .iter()
            .filter(|(k, _v)| !inactive.contains(k))
            .min_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k)
            .unwrap();
        println!("next_start: {:?}", next_start);
        inactive.push(*next_start);
        let next_start_value = dijkstra[next_start];

        let next_positions = get_next_positions(next_start, &map, &inactive);
        for next_position in next_positions {
            let cell_value = map[next_position.0][next_position.1];
            if next_start_value + cell_value < dijkstra[&next_position] {
                dijkstra.insert(next_position, next_start_value + cell_value);
            }
        }
    }
    // Find key with smallest value

    dijkstra[&exit] as usize
}

fn get_next_positions(
    cell: &(usize, usize),
    map: &Vec<Vec<u32>>,
    inactive: &Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut next: Vec<(usize, usize)> = Vec::new();
    if cell.0 > 0 {
        next.push((cell.0 - 1, cell.1));
    }
    if cell.1 > 0 {
        next.push((cell.0, cell.1 - 1));
    }
    if cell.0 < map.len() - 1 {
        next.push((cell.0 + 1, cell.1));
    }
    if cell.1 < map[0].len() - 1 {
        next.push((cell.0, cell.1 + 1));
    }
    next.iter()
        .filter(|x| !inactive.contains(x))
        .map(|x| *x)
        .collect()
}

fn get_previous(cell: (usize, usize), map: &Vec<Vec<u32>>) -> Vec<(usize, usize, char)> {
    let mut previous: Vec<(usize, usize, char)> = Vec::new();
    if cell.0 > 0 {
        previous.push((cell.0 - 1, cell.1, 'D'));
    }
    if cell.1 > 0 {
        previous.push((cell.0, cell.1 - 1, 'R'));
    }
    if cell.0 < map.len() - 1 {
        previous.push((cell.0 + 1, cell.1, 'U'));
    }
    if cell.1 < map[0].len() - 1 {
        previous.push((cell.0, cell.1 + 1, 'L'));
    }
    previous
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_example_data() {
        assert_eq!(102, solve_puzzle("test_data"));
    }

    #[test]
    #[ignore]
    fn test_solution() {
        assert_eq!(0, solve_puzzle("input"));
    }
}
