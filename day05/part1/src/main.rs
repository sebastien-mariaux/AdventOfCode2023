use std::{collections::HashMap, fs};

fn main() {
    let result = solve_puzzle("../input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u128 {
    let data = read_data(file_name);

    let mut split_data = data.split("\n\n");
    let seeds = split_data.next().unwrap().split(": ").nth(1).unwrap().split(' ').map(|x| x.parse::<u128>().unwrap());

    let mut maps: Vec<HashMap<(u128, u128), i128>> = Vec::new();

    split_data.for_each(|x| {
        let mut map = HashMap::new();
        x.split('\n').skip(1).for_each(|y| {
            let numbers = y
                .split(' ')
                .map(|x| x.parse::<u128>().unwrap())
                .collect::<Vec<u128>>();
            let destination_range_start = numbers[0];
            let source_range_start = numbers[1];
            let length = numbers[2];

            map.insert(
                (source_range_start, source_range_start + length - 1),
                destination_range_start as i128 - source_range_start as i128,
            );
        });
        maps.push(map);
    });

    println!("{:?}", maps);

    println!("{:?}", seeds);
    // return 0;

    seeds
        .map(|seed| {
            // get_from_source_to_destination(&seed_to_soil, seed)

            get_from_source_to_destination(
                &maps[6],
                get_from_source_to_destination(
                    &maps[5],
                    get_from_source_to_destination(
                        &maps[4],
                        get_from_source_to_destination(
                            &maps[3],
                            get_from_source_to_destination(
                                &maps[2],
                                get_from_source_to_destination(
                                    &maps[1],
                                    get_from_source_to_destination(&maps[0], seed),
                                ),
                            ),
                        ),
                    ),
                ),
            )
        })
        .min()
        .unwrap()
}

fn get_from_source_to_destination(map: &HashMap<(u128, u128), i128>, source: u128) -> u128 {
    let range = map
        .keys()
        .find(|(min, max)| min <= &source && max >= &source);
    let result = match range {
        None => source,
        Some(range) => (source as i128 + map.get(range).unwrap()) as u128,
    };

    result
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(35, solve_puzzle("../test_data"));
    }

    #[test]
    #[ignore]
    fn test_solution() {
        assert_eq!(535088217, solve_puzzle("../input"));
    }
}

// seeds: 79 14 55 13

// seed-to-soil map:
// destination range start at 50 - Source range start at 98 - length 2
// seed 98 -> soil 50
// seed 99 -> soil 51
// 50 98 2

// seed number 53 correspond to soil number 55
// 52 50 48
