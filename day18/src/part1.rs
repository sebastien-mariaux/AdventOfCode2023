use crate::utils::read_data;
use std::collections::HashSet;

pub fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);

    let mut trenches: Vec<(isize, isize)> = Vec::new();
    trenches.push((0, 0));

    for line in data.lines() {
        let  mut instructions = line.split(" ");
        let direction = instructions.next().unwrap();
        let distance = instructions.next().unwrap().parse::<isize>().unwrap();

        dig_trenches(&mut trenches, direction, distance);
    }

    // Fill trench
    let min_i = trenches.iter().min_by_key(|(x, _)| x).unwrap().0;
    let max_i = trenches.iter().max_by_key(|(x, _)| x).unwrap().0;
    let min_j = trenches.iter().min_by_key(|(_, y)| y).unwrap().1;
    let max_j = trenches.iter().max_by_key(|(_, y)| y).unwrap().1;
    println!("trenches: {:?}", trenches);
    println!("min_i: {}, max_i: {}, min_j: {}, max_j: {}", min_i, max_i, min_j, max_j);

    // let mut fill: HashSet<(isize, isize)> = HashSet::new();
    // let mut insiders: HashSet<(isize, isize)> = HashSet::new();
    let mut outsiders: HashSet<(isize, isize)> = HashSet::new();

    for i in min_i..=max_i {
        for j in min_j..=max_j {
            if (i == min_i || i == max_i || j == min_j || j == max_j) && !trenches.contains(&(i, j)) {
                outsiders.insert((i, j));
            }
        }
    }


    let mut stack: Vec<(isize, isize)> = outsiders.iter()
    .map(|(i,j)| (*i, *j))
    .collect::<Vec<_>>();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    while !stack.is_empty() {
        let point = stack.pop().unwrap();
        if visited.contains(&point) {
            continue;
        }
        let neighbors: Vec<(isize, isize)> = vec![
            (point.0 - 1, point.1),
            (point.0 + 1, point.1),
            (point.0, point.1 - 1),
            (point.0, point.1 + 1),
        ].iter().filter(|(ii,jj)|  {
            ii >= &min_i && ii <= &max_i && jj >= &min_j && jj <= &max_j && !trenches.contains(&(*ii, *jj))
        })
        .map(|(ii,jj)| (*ii, *jj))
        .collect();
        outsiders.extend(neighbors.clone());
        stack.extend(neighbors.clone());
        visited.insert(point);
    }

    println!("outsiders: {:?}", outsiders);

    let mut result = trenches.len() as u32;
    for i in min_i..=max_i {
        for j in min_j..=max_j {
            if !outsiders.contains(&(i, j)) && !trenches.contains(&(i, j)) {
                result += 1;
            }
        }
    }




    //print map
    // for i in min_i..=max_i {
    //     for j in min_j..=max_j {
    //         if trenches.contains(&(i, j)) {
    //             print!("X");
    //         } else if fill.contains(&(i, j)) {
    //             print!("O");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    result
}

// fn is_inside_trench(trenches: &Vec<(isize, isize)>, point: &(isize, isize), insiders: &mut Vec<(isize, isize)>, outsiders: &mut Vec<(isize, isize)>) -> bool {
//     if outsiders.contains(point) {
//         return false;
//     }

//     if insiders.contains(point) {
//         return true;
//     }

//     let min_i = trenches.iter().min_by_key(|(x, _)| x).unwrap().0;
//     let max_i = trenches.iter().max_by_key(|(x, _)| x).unwrap().0;
//     let min_j = trenches.iter().min_by_key(|(_, y)| y).unwrap().1;
//     let max_j = trenches.iter().max_by_key(|(_, y)| y).unwrap().1;
//     let (i,j) = *point;

//     if i == min_i || i == max_i || j == min_j || j == max_j {
//         outsiders.push(*point);
//         return false;
//     }

//     let neighbors = vec![
//         (point.0 - 1, point.1),
//         (point.0 + 1, point.1),
//         (point.0, point.1 - 1),
//         (point.0, point.1 + 1),
//     ];

//     for neighbor in neighbors.iter().filter(|n| !trenches.contains(n)) {
//         if !is_inside_trench(trenches, &neighbor, insiders, outsiders) {
//             outsiders.push(*point);
//             return false;
//         }

//     }
//     insiders.push(*point);
//     true

// }

fn dig_trenches(trenches: &mut Vec<(isize, isize)>, direction: &str, distance: isize) {
    let mut new_trench = trenches.last().unwrap().clone();

    for _ in 0..distance {
        match direction {
            "U" => new_trench.0 -= 1,
            "D" => new_trench.0 += 1,
            "R" => new_trench.1 += 1,
            "L" => new_trench.1 -= 1,
            _ => panic!("Unknown direction: {}", direction),
        }
        if !trenches.contains(&new_trench) {
            trenches.push(new_trench.clone());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_example_data() {
        assert_eq!(62, solve_puzzle("test_data"));
    }

    #[test]
    #[ignore]
    fn test_solution() {
        assert_eq!(50603, solve_puzzle("input"));
    }
}
