use std::collections::HashMap;
use std::collections::HashSet;

use crate::utils::read_data;

pub fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);

    let (part1, part2) = data
        .split_once("\n\n")
        .map(|(x, y)| (x.to_string(), y.to_string()))
        .unwrap();
    let workflows = format_workflows(&part1);

    let mut accepted: Vec<HashMap<char, u32>> = Vec::new();

    for l in part2.lines() {
        let part = format_part(&l);
        if is_accepted(&part, &workflows) {
            accepted.push(part);
        }
    }

    println!("accepted {:?}", accepted);

    accepted.iter().map(|x|  {
        x.values().sum::<u32>()
    }).sum()
}

fn format_part(part: &str) -> HashMap<char, u32> {
    part[1..part.len() - 1]
        .split(',')
        .map(|x| {
            let (key, value) = x.split_once('=').unwrap();
            (key.chars().next().unwrap(), value.parse::<u32>().unwrap())
        })
        .collect()
}

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

fn is_accepted(part: &HashMap<char, u32>, workflows: &HashMap<String, Vec<String>>) -> bool {
    apply_rule(part, workflows, String::from("in"))
}

fn apply_rule(
    part: &HashMap<char, u32>,
    workflows: &HashMap<String, Vec<String>>,
    name: String,
) -> bool {
    if name == "R" {
        return false;
    }
    if name == "A" {
        return true;
    }
    let rule = workflows.get(&name).unwrap().clone();
    for condition in rule.iter() {
        if condition == "R"  {
            return false;
        }
        if condition == "A"  {
            return true;
        }
        if condition.contains('>')  {
            let (key, value) = condition
                .split_once('>')
                .map(|(x, y)| {
                    (
                        x.chars().next().unwrap(),
                        y.split(':').next().unwrap().parse::<u32>().unwrap(),
                    )
                })
                .unwrap();
            if part.get(&key).unwrap() > &value {
                return apply_rule(
                    part,
                    workflows,
                    condition.split_once(':').unwrap().1.to_string(),
                );
            } else {
                continue;
            }
        }
        if condition.contains('<') {
            let (key, value) = condition
                .split_once('<')
                .map(|(x, y)| {
                    (
                        x.chars().next().unwrap(),
                        y.split(':').next().unwrap().parse::<u32>().unwrap(),
                    )
                })
                .unwrap();
            if part.get(&key).unwrap() < &value {
                return apply_rule(
                    part,
                    workflows,
                    condition.split_once(':').unwrap().1.to_string(),
                );
            } else {
                continue;
            }
        }
        println!("condition {:?}", condition);

        return apply_rule(part, workflows, condition.to_string());
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(19114, solve_puzzle("test_data"));
    }

    #[test]
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
