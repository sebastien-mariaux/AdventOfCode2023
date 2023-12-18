use crate::utils::read_data;
use std::collections::HashSet;

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);

    let mut trenches: Vec<(isize, isize)> = Vec::new();
    trenches.push((0, 0));

    let mut sides:Vec<((isize, isize), (isize, isize))> = Vec::new();

    for line in data.lines() {
        let  mut instructions = line.split(" ");
        instructions.next();
        instructions.next();
        let hexa = instructions.next().unwrap();
        let five_first = &hexa[2..7];
        let one_last = &hexa[7..8];
        // println!("hexa: {}, five_first: {}, one_last: {}", hexa, five_first, one_last);
        let direction = match one_last {
            "0" => "R",
            "1" => "D",
            "2" => "L",
            "3" => "U",
            _ => panic!("Unknown direction: {}", one_last),
        };

        let distance = isize::from_str_radix(five_first, 16).unwrap();
        // println!("hexa {} direction: {}, distance: {}", hexa, direction, distance);


        dig_trenches(&mut sides, direction, distance);
    }

    println!("sides: {:?}", sides);

    // Fill trench
    let min_i = trenches.iter().min_by_key(|(x, _)| x).unwrap().0;
    let max_i = trenches.iter().max_by_key(|(x, _)| x).unwrap().0;
    let min_j = trenches.iter().min_by_key(|(_, y)| y).unwrap().1;
    let max_j = trenches.iter().max_by_key(|(_, y)| y).unwrap().1;
    // println!("trenches: {:?}", trenches);
    // println!("min_i: {}, max_i: {}, min_j: {}, max_j: {}", min_i, max_i, min_j, max_j);

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

    // println!("outsiders: {:?}", outsiders);

    let mut result = trenches.len() as usize;
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

fn dig_trenches(sides: &mut Vec<((isize, isize), (isize, isize))>, direction: &str, distance: isize) {
    let mut last_angle;
    if sides.is_empty() {
        last_angle = (0, 0);
    } else {
        last_angle = sides.last().unwrap().clone().1;
    }

    let new_angle;
    match direction {
        "R" => {
            new_angle = (last_angle.0, last_angle.1 + distance);
        }
        "D" => {
            new_angle = (last_angle.0 + distance, last_angle.1);
        }
        "L" => {
            new_angle = (last_angle.0, last_angle.1 - distance);
        }
        "U" => {
            new_angle = (last_angle.0 - distance, last_angle.1);
        }
        _ => panic!("Unknown direction: {}", direction),
    }
    sides.push((last_angle, new_angle));


}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(952408144115, solve_puzzle("test_data"));
    }

    #[test]
    #[ignore]
    fn test_solution() {
        assert_eq!(1, solve_puzzle("input"));
    }
}
