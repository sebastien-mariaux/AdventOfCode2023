use std::{collections::HashMap, fs, os::unix::process};

fn main() {
    let result = solve_puzzle("../input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> i128 {
    let data = read_data(file_name);
    let mut split_data = data.split("\n\n");

    // Create seed ranges
    let seed_ranges: Vec<(i128, i128)> = split_data
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>()
        .chunks(2)
        .map(|range| {
            let start = range[0];
            let count = range[1];
            (start, start + count)
        })
        .collect();

    // Create successive maps
    let mut maps: Vec<HashMap<(i128, i128), i128>> = Vec::new();
    split_data.for_each(|x| {
        let mut map = HashMap::new();
        x.split('\n').skip(1).for_each(|y| {
            let numbers = y
                .split(' ')
                .map(|x| x.parse::<i128>().unwrap())
                .collect::<Vec<i128>>();
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

    let mut current_ranges = seed_ranges;
    for map in &maps {
        current_ranges = process_ranges(&current_ranges, &map);
        println!("Iter: {:?}", current_ranges);
    }

    *current_ranges.iter().map(|(min, _)| min).min().unwrap()

}

fn process_ranges(
    ranges: &Vec<(i128, i128)>,
    map: &HashMap<(i128, i128), i128>,
) -> Vec<(i128, i128)> {
    let mut new_ranges: Vec<(i128, i128)> = Vec::new();

    for range in ranges {
        let processed_ranges = process_range(range, map);
        new_ranges.extend(processed_ranges);
    }

    new_ranges
}

fn process_range(
    range: &(i128, i128),
    map: &HashMap<(i128, i128), i128>,
) -> Vec<(i128, i128)> {
    let new_ranges =  split_range(range, map);
    transform_range(&new_ranges, &map)
}

fn split_range(
    range: &(i128, i128),
    map: &HashMap<(i128, i128), i128>,
) -> Vec<(i128, i128)> {
    let start = range.0;
    let end = range.1;
    let map = map;
    // println!("Seed range: {:?}", range);
    // println!("Map: {:?}", map);
    let included_limits = map.keys().fold(Vec::new(), |mut acc, (min, max)| {
        if min >= &start && min <= &end {
            acc.push(*min);
        };
        if max >= &start && max <= &end {
            acc.push(*max);
        };
        acc
    });

    let mut new_ranges: Vec<(i128, i128)> = Vec::new();
    let mut previous_limit = start;
    for limit in included_limits {
        new_ranges.push((previous_limit, limit));
        previous_limit = limit;
    }
    new_ranges.push((previous_limit, end));
    new_ranges
}

fn transform_range(
    ranges: &Vec<(i128, i128)>,
    map: &HashMap<(i128, i128), i128>,
) -> Vec<(i128, i128)> {
    let mut new_ranges: Vec<(i128, i128)> = Vec::new();

    for range in ranges {
        let containing_range = map
            .keys()
            .find(|(min, max)| min <= &range.0 && max >= &range.1);
        let value = match containing_range {
            None => 0 as i128,
            Some(range) => *map.get(range).unwrap(),
        };

        let new_range = (range.0 + value, range.1 + value);
        new_ranges.push(new_range);
    }

    new_ranges
}

fn get_from_source_to_destination(map: &HashMap<(i128, i128), i128>, source: i128) -> i128 {
    let range = map
        .keys()
        .find(|(min, max)| min <= &source && max >= &source);
    let result = match range {
        None => source,
        Some(range) => (source as i128 + map.get(range).unwrap()) as i128,
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
        assert_eq!(46, solve_puzzle("../test_data"));
    }

    #[test]
    // #[ignore]
    fn test_solution() {
        assert_eq!(51399228, solve_puzzle("../input"));
    }
}
