use std::collections::HashMap;
use std::collections::HashSet;

use crate::utils::read_data;

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);

    let (part1, part2) = data
        .split_once("\n\n")
        .map(|(x, y)| (x.to_string(), y.to_string()))
        .unwrap();
    let workflows = format_workflows(&part1);

    let mut ranges = HashMap::new();
    ranges.insert('a', (1, 4001));
    ranges.insert('m', (1, 4001));
    ranges.insert('s', (1, 4001));
    ranges.insert('x', (1, 4001));

    let mut accepted_ranges: Vec<HashMap<char, (usize, usize)>> = Vec::new();

    process_ranges(
        &workflows,
        &mut ranges,
        "in".to_string(),
        &mut accepted_ranges,
    );

    0
}

fn process_ranges(
    workflows: &HashMap<String, Vec<String>>,
    ranges: &mut HashMap<char, (usize, usize)>,
    name: String,
    accepted_ranges: &mut Vec<HashMap<char, (usize, usize)>>,
) {
    let conditions = workflows.get(&name).unwrap();
    for condition in conditions {
        if condition == "R" {
            continue;
        }
        if condition == "A" {
            accepted_ranges.push(ranges.clone());
        }
        if condition.contains('>') {
            let (key, value) = condition
                .split_once('>')
                .map(|(x, y)| {
                    (
                        x.chars().next().unwrap(),
                        y.split(':').next().unwrap().parse::<usize>().unwrap(),
                    )
                })
                .unwrap();
            let current_range_for_char = ranges.get(&key).unwrap();
            // let mut new_ranges = HashMap::new();
        }
    }
}

// fn format_part(part: &str) -> HashMap<char, usize> {
//     part[1..part.len() - 1]
//         .split(',')
//         .map(|x| {
//             let (key, value) = x.split_once('=').unwrap();
//             (key.chars().next().unwrap(), value.parse::<usize>().unwrap())
//         })
//         .collect()
// }

fn format_workflows(workflows: &String) -> HashMap<String, Vec<String>> {
    let mut all_workflows: HashMap<String, Vec<String>> = HashMap::new();
    for mut workflow in workflows.lines() {
        let mut chars = workflow.chars();
        chars.next_back();
        workflow = chars.as_str();
        let (name, conditions) = workflow.split_once("{").unwrap();
        all_workflows.insert(
            name.to_string(),
            conditions.split(',').map(|x| x.to_string()).collect(),
        );
    }
    all_workflows
}

fn is_accepted(part: &HashMap<char, usize>, workflows: &HashMap<String, Vec<String>>) -> bool {
    // apply_rule(part, workflows, String::from("in"))
    false
}

// fn apply_rule(
//     part: &HashMap<char, usize>,
//     workflows: &HashMap<String, Vec<String>>,
//     name: String,
// ) -> bool {
//     if name == "R" {
//         return false;
//     }
//     if name == "A" {
//         return true;
//     }
//     let rule = workflows.get(&name).unwrap().clone();
//     for condition in rule.iter() {
//         if condition == "R"  {
//             return false;
//         }
//         if condition == "A"  {
//             return true;
//         }
//         if condition.contains('>')  {
//             let (key, value) = condition
//                 .split_once('>')
//                 .map(|(x, y)| {
//                     (
//                         x.chars().next().unwrap(),
//                         y.split(':').next().unwrap().parse::<usize>().unwrap(),
//                     )
//                 })
//                 .unwrap();
//             if part.get(&key).unwrap() > &value {
//                 return apply_rule(
//                     part,
//                     workflows,
//                     condition.split_once(':').unwrap().1.to_string(),
//                 );
//             } else {
//                 continue;
//             }
//         }
//         if condition.contains('<') {
//             let (key, value) = condition
//                 .split_once('<')
//                 .map(|(x, y)| {
//                     (
//                         x.chars().next().unwrap(),
//                         y.split(':').next().unwrap().parse::<usize>().unwrap(),
//                     )
//                 })
//                 .unwrap();
//             if part.get(&key).unwrap() < &value {
//                 return apply_rule(
//                     part,
//                     workflows,
//                     condition.split_once(':').unwrap().1.to_string(),
//                 );
//             } else {
//                 continue;
//             }
//         }
//         println!("condition {:?}", condition);

//         return apply_rule(part, workflows, condition.to_string());
//     }

//     false
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_example_data() {
        assert_eq!(167409079868000, solve_puzzle("test_data"));
    }

    #[test]
    #[ignore]
    fn test_solution() {
        assert_eq!(0, solve_puzzle("input"));
    }
}

// machine parts
// x: Extremely cool looking
// m: Musical (it makes a noise when you hit it)
// a: Aerodynamic
// s: Shiny

// workflow : name + rules
// rejected (R)
// accepted (A).

// in =  initial workflow
