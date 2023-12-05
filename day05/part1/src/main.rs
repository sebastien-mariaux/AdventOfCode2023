use std::{
    collections::{btree_map::Range, HashMap},
    fs,
};

fn main() {
    let result = solve_puzzle("../input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u128 {
    let data = read_data(file_name);
    let mut lines = data.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<u128>().unwrap());
    lines.next();
    lines.next();
    let mut seed_to_soil = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        }
        let numbers = line
            .split(' ')
            .map(|x| x.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        let destination_range_start = numbers[0];
        let source_range_start = numbers[1];
        let length = numbers[2];

        seed_to_soil.insert(
            (source_range_start, source_range_start + length - 1),
            (destination_range_start as i128 - source_range_start as i128),
        );
    }

    lines.next();
    let mut soil_to_fertilizer = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        }
        let numbers: Vec<u128> = line
            .split(' ')
            .map(|x| x.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        let destination_range_start = numbers[0];
        let source_range_start = numbers[1];
        let length = numbers[2];

        soil_to_fertilizer.insert(
            (source_range_start, source_range_start + length - 1),
            (destination_range_start as i128 - source_range_start as i128),
        );
    }
    println!("{:?}", soil_to_fertilizer);

    lines.next();
    let mut fertilizer_to_water = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        }
        let numbers: Vec<u128> = line
            .split(' ')
            .map(|x| x.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        let destination_range_start = numbers[0];
        let source_range_start = numbers[1];
        let length = numbers[2];

        fertilizer_to_water.insert(
            (source_range_start, source_range_start + length - 1),
            (destination_range_start as i128 - source_range_start as i128),
        );
    }

    lines.next();
    let mut water_to_light = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        }
        let numbers: Vec<u128> = line
            .split(' ')
            .map(|x| x.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        let destination_range_start = numbers[0];
        let source_range_start = numbers[1];
        let length = numbers[2];

        water_to_light.insert(
            (source_range_start, source_range_start + length - 1),
            (destination_range_start as i128 - source_range_start as i128),
        );
    }

    lines.next();
    let mut light_to_temperature = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        }
        let numbers: Vec<u128> = line
            .split(' ')
            .map(|x| x.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        let destination_range_start = numbers[0];
        let source_range_start = numbers[1];
        let length = numbers[2];

        light_to_temperature.insert(
            (source_range_start, source_range_start + length - 1),
            (destination_range_start as i128 - source_range_start as i128),
        );
    }

    lines.next();
    let mut temperature_to_humidity = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        }
        let numbers: Vec<u128> = line
            .split(' ')
            .map(|x| x.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        let destination_range_start = numbers[0];
        let source_range_start = numbers[1];
        let length = numbers[2];

        temperature_to_humidity.insert(
            (source_range_start, source_range_start + length - 1),
            (destination_range_start as i128 - source_range_start as i128),
        );
    }

    lines.next();
    let mut humidity_to_location = HashMap::new();
    loop {
        let line = match lines.next() {
            None => break,
            Some(line) => line,
        };
        if line == "" {
            break;
        }
        let numbers: Vec<u128> = line
            .split(' ')
            .map(|x| x.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        let destination_range_start = numbers[0];
        let source_range_start = numbers[1];
        let length = numbers[2];

        humidity_to_location.insert(
            (source_range_start, source_range_start + length - 1),
            (destination_range_start as i128 - source_range_start as i128),
        );
    }

    seeds
        .map(|seed| {
            // get_from_source_to_destination(&seed_to_soil, seed)

            get_from_source_to_destination(
                &humidity_to_location,
                get_from_source_to_destination(
                    &temperature_to_humidity,
                    get_from_source_to_destination(
                        &light_to_temperature,
                        get_from_source_to_destination(
                            &water_to_light,
                            get_from_source_to_destination(
                                &fertilizer_to_water,
                                get_from_source_to_destination(
                                    &soil_to_fertilizer,
                                    get_from_source_to_destination(&seed_to_soil, seed),
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
