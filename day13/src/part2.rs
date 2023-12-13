use crate::utils::read_data;
use num::integer::lcm;
use std::{collections::HashMap, fs};

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);

    1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_example_data() {
        assert_eq!(0, solve_puzzle("test_data"));
    }

    #[test]
    #[ignore]
    fn test_solution() {
        assert_eq!(0, solve_puzzle("input"));
    }
}
