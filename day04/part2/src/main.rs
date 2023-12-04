use std::{collections::HashMap, fs};

fn main() {
    let result = solve_puzzle("../input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);

    let mut cards = HashMap::new();
    let mut my_numbers = HashMap::new();

    for (i, line) in data.lines().enumerate() {
        let card_number = (i + 1) as u32;
        let card_data = line.split(": ").nth(1).unwrap();
        let mut values = card_data.split(" | ");
        let winnings = values
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let numbers = values
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        cards.insert(card_number, winnings);
        my_numbers.insert(card_number, numbers);
    }

    let mut cards_count = cards
        .keys()
        .map(|x| (x.to_owned(), 1))
        .collect::<HashMap<u32, u32>>();

    for (i, line) in data.lines().enumerate() {
        let card_number = (i + 1) as u32;
        println!("Playing card {}", card_number);
        let matching_count = my_numbers
            .get(&(&card_number))
            .unwrap()
            .iter()
            .filter(|x| cards.get(&(i as u32 + 1)).unwrap().contains(x))
            .count();
        println!("Matching count {}", matching_count);
        for n in card_number + 1..card_number + matching_count as u32 + 1 {
            println!("Incrementing card {}", n);
            cards_count.insert(
                n,
                cards_count.get(&n).unwrap() + cards_count.get(&card_number).unwrap(),
            );
        }
    }
    println!("{:?}", cards_count);

    cards_count.values().sum()
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(30, solve_puzzle("../test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(13114317, solve_puzzle("../input"));
    }
}
