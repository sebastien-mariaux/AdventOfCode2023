use std::{fs, collections::HashMap};

fn main() {
    let result = solve_puzzle("../input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);
    let mut lines = data.lines();
    let directions = lines.next().unwrap();
    lines.next();

    let mut maps =  HashMap::new();
    for  line in lines {
        if line.len() == 0 {
            break;
        }
        let fixed_line = line.replace("(", "").replace(")", "");
        let (source, destinations) = fixed_line.split_once(" = ").unwrap();
;        let (dest_left, dest_right) = destinations.split_once(", ").unwrap();
        maps.insert(source.to_string(), (dest_left.to_string(), dest_right.to_string()));
    }

    let mut position = String::from("AAA");

    println!("{:?}", maps);
    for (step, direction) in directions.chars().cycle().enumerate() {
        println!("{}",position);
        let (left, right) = maps.get(&position).unwrap();
        if direction == 'L' {
            position = left.to_string();
        } else {
            position = right.to_string();
        }
        if position == "ZZZ" {
            return step as u32 + 1;
        }
    }

    0
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(6, solve_puzzle("test_data"));
    }

    #[test]
    // #[ignore]
    fn test_solution() {
        assert_eq!(12083, solve_puzzle("../input"));
    }
}
