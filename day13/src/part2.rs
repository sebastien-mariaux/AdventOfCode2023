use crate::utils::read_data;

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);
    let mut left_columns = 0;
    let mut top_rows = 0;

    for (pattern_number, pattern) in data.split("\n\n").enumerate() {
        println!("Pattern {}\n{}", pattern_number, pattern);
        let mut reflection_found = false;
        let map:Vec<Vec<char>> = pattern.lines().map(|line| line.chars().collect()).collect();
        // HORIZONTAL LINE
        let mut horizontal_reflection: Option<usize> = None;
        for i in 0..map.len() - 1 {
            let mut counter = 0;
            let mut is_reflection = true;
            while i >= counter && i + counter < map.len() - 1 {
                if map[i - counter] == map[i + 1 + counter] {
                    counter += 1;
                } else {
                    is_reflection = false;
                    break;
                }
            }
            if is_reflection {
                reflection_found = true;
                horizontal_reflection = Some(i);
                // println!("Found reflection at row {}", i);
                // println!("Top rows: {}", i + 1);
                // top_rows += i + 1;
                // break;
            }
        }

        // run again with one error possible
        for i in 0..map.len() - 1 {
            if horizontal_reflection.is_some() && horizontal_reflection.unwrap() == i {
                continue;
            }
            let mut error_found = false;
            let mut counter = 0;
            let mut is_reflection = true;
            while i >= counter && i + counter < map.len() - 1 {
                let difference_count = map[i - counter].iter().zip(map[i + 1 + counter].iter()).filter(|(a, b)| a != b).count();
                if map[i - counter] == map[i + 1 + counter]  {
                    counter += 1;
                } else if !error_found && difference_count == 1 {
                    error_found = true;
                    counter += 1;
                }  else {
                    is_reflection = false;
                    break;
                }
            }
            if is_reflection {
                reflection_found = true;
                println!("Found reflection at row {}", i);
                println!("Top rows: {}", i + 1);
                top_rows += i + 1;
                break;
            }
        }

        // VERITCAL LINE
        let mut vertical_reflection: Option<usize> = None;
        for j in 0..map[0].len() - 1 {

            let mut counter = 0;
            let mut is_reflection = true;
            while j >= counter && j + counter < map[0].len() - 1 {
                if map.iter().map(|row| row[j - counter]).collect::<Vec<char>>() == map.iter().map(|row| row[j + 1 + counter]).collect::<Vec<char>>() {
                    counter += 1;
                } else {
                    is_reflection = false;
                    break;
                }
            }
            if is_reflection {
                reflection_found = true;
                vertical_reflection = Some(j);
                // println!("Found reflection at column {}", j);
                // println!("Left columns: {}", j + 1);
                // left_columns += j + 1;
                // break;
            }
        }

        // run again with one error possible
        for j in 0..map[0].len() - 1 {
            if vertical_reflection.is_some() && vertical_reflection.unwrap() == j {
                continue;
            }
            let mut error_found = false;
            let mut counter = 0;
            let mut is_reflection = true;
            while j >= counter && j + counter < map[0].len() - 1 {
                let difference_count = map.iter().map(|row| row[j - counter]).zip(map.iter().map(|row| row[j + 1 + counter])).filter(|(a, b)| a != b).count();
                if map.iter().map(|row| row[j - counter]).collect::<Vec<char>>() == map.iter().map(|row| row[j + 1 + counter]).collect::<Vec<char>>() {
                    counter += 1;
                } else if !error_found && difference_count == 1 {
                    error_found = true;
                    counter += 1;
                } else {
                    is_reflection = false;
                    break;
                }
            }
            if is_reflection {
                reflection_found = true;
                println!("Found reflection at column {}", j);
                println!("Left columns: {}", j + 1);
                left_columns += j + 1;
                break;
            }
        }


        if !reflection_found {
            panic!("No reflection found");
        }
    }
    println!("Top rows: {}", top_rows);

    top_rows * 100 + left_columns
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(400, solve_puzzle("test_data"));
    }

    #[test]
    #[ignore]
    fn test_solution() {
        assert_eq!(0, solve_puzzle("input"));
    }
}
