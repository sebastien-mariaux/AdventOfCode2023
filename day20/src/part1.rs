use crate::utils::read_data;
use std::collections::VecDeque;
use std::{collections::HashMap, path::Display};
use std::fmt;

struct Module {
    mod_type: char,
    name: String,
    on: bool,
    received_pulses: HashMap<String, char>,
    targets: Vec::<String>,
    low_pulse_count: usize,
    high_pulse_count: usize,
}

impl Module {
    fn new(mod_type: char, name: String, targets: Vec<String>) -> Self {
        Module {
            mod_type,
            name,
            on: false,
            received_pulses: HashMap::new(),
            targets,
            low_pulse_count: 0,
            high_pulse_count: 0,
        }
    }

    fn toggle(&mut self) {
        self.on = !self.on;
    }

    fn is_on(&self) -> bool {
        self.on
    }

    fn is_off(&self) -> bool {
        !self.on
    }

    fn receive_pulse(&mut self, pulse: char, sender: String) {
        self.received_pulses.insert(sender, pulse);
    }

    fn increase_count(&mut self, pulse: char) {
        if pulse == 'L' {
            self.low_pulse_count += 1;
        } else {
            self.high_pulse_count += 1;
        }
    }

    // fn send_pulse(&mut self, pulse: char, target: String, modules: &mut HashMap<String, Module>) {
    //     let target_module = modules.get_mut(&target).unwrap();
    //     target_module.receive_pulse(pulse, self.name.to_string());
    // }

    fn process_pulse(&mut self, stack: &mut VecDeque::<(String, char)>, received_pulse: char) {
        if self.name == "button"  {
            stack.push_back(("broadcaster".to_string(), 'L'));
            self.increase_count('L');
        } else  if self.name == "broadcaster"  {
            for target in self.targets.clone() {
                stack.push_back((target.to_string(), received_pulse));
                // self.send_pulse(received_pulse, target.to_string(), modules);
                self.increase_count(received_pulse);
            }
        } else  if self.mod_type == '%' {
            if received_pulse == 'L' {
                for target in self.targets.clone() {
                    let next_pulse = if self.is_off() { 'H' } else { 'L' };
                    stack.push_back((target.to_string(), next_pulse));
                    // self.send_pulse(next_pulse, target.to_string(), modules);
                    self.increase_count(next_pulse);
                }
                self.toggle();
            }
        } else if self.mod_type == '&' {
            if self.received_pulses.values().all(|x |x == &'H') {
                for target in self.targets.clone() {
                    stack.push_back((target.to_string(), 'L'));
                    // self.send_pulse('L', target.to_string(), modules);
                    self.increase_count('L');
                }
            } else {
                for target in self.targets.clone() {
                    stack.push_back((target.to_string(), 'H'));
                    // self.send_pulse('H', target.to_string(), modules);
                    self.increase_count('H');
                }
            }
        }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} -> {}", self.mod_type, self.name, self.on)
    }
}

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);

    let mut modules = HashMap::<String, Module>::new();
    for  line in data.lines() {
        let ( a, b) = line.split_once(" -> ").unwrap();
        let targets = b.split(", ").map(|x| x.to_string()).collect::<Vec<String>>();
        if a == "broadcaster" {
            modules.insert(a.to_string(), Module::new('B', a.to_string(), targets));
        } else {
            let (mod_type, name) = a.split_at(1);
            modules.insert(name.to_string(), Module::new(mod_type.chars().next().unwrap(), name.to_string(), targets));
        }
    }
    modules.insert("button".to_string(), Module::new('S', "button".to_string(), vec!["broadcaster".to_string()]));

    let mut stack = VecDeque::<(String, char)>::new();
    for _  in 0..1000 {
        stack.push_back(("button".to_string(), 'L'));
    }

    while !stack.is_empty() {
        let (name, pulse) = stack.pop_front().unwrap();
        if let Some(module) = &mut modules.get_mut(&name) {
            module.process_pulse(&mut stack, pulse);
        }
    }

    let high_pulses_count: usize = modules.values().map(|x| x.high_pulse_count).sum();
    let low_pulses_count: usize = modules.values().map(|x| x.low_pulse_count).sum();


    high_pulses_count * low_pulses_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(32000000, solve_puzzle("test_data_1"));
    }


    #[test]
    // #[ignore]
    fn test_example_data_2() {
        assert_eq!(11687500, solve_puzzle("test_data_2"));
    }

    #[test]
    #[ignore]
    fn test_solution() {
        assert_eq!(0, solve_puzzle("input"));
    }
}

// % are off -> low pulse -> toggle on off. toggle to on -> send high pulse. toggle to off -> send low pulse
// & remember last pulse (initial low) Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.