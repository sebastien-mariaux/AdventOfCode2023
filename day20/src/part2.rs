use crate::utils::read_data;
use std::collections::VecDeque;
use std::fmt;
use std::collections::HashMap;
use num::integer::gcd;

struct Module {
    mod_type: char,
    name: String,
    on: bool,
    received_pulses: HashMap<String, char>,
    targets: Vec<String>,
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

    fn toggle(&mut self, flip_flop_states: &mut HashMap::<String, usize>, counter: usize) {
        self.on = !self.on;
        if !flip_flop_states.contains_key(&self.name) {
            return;
        }
        let current = *flip_flop_states.get(&self.name).unwrap();
        if self.on && current == 0 {
            flip_flop_states.insert(self.name.to_string(), counter - current);
        }
    }

    fn is_off(&self) -> bool {
        !self.on
    }

    fn initiate_input_modules(&mut self, modules: &HashMap<String, Vec<String>>) {
        if self.mod_type != '&' {
            return;
        }
        for (name, targets) in modules.iter() {
            if targets.contains(&self.name) {
                self.receive_pulse('L', name);
            }
        }
    }

    fn receive_pulse(&mut self, pulse: char, sender: &str) {
        self.received_pulses.insert(sender.to_string(), pulse);
    }

    fn increase_count(&mut self, pulse: char) {
        if pulse == 'L' {
            self.low_pulse_count += 1;
        } else {
            self.high_pulse_count += 1;
        }
    }

    fn send_pulse(
        &mut self,
        pulse: char,
        target: String,
        stack: &mut VecDeque<(String, char)>,
        sent_pulses: &mut Vec<(String, char)>,
    ) {
        stack.push_back((target.to_string(), pulse));
        sent_pulses.push((target.to_string(), pulse));
        self.increase_count(pulse);
        // println!("{} -> {} ({})", self.name, target, pulse);
    }

    fn process_pulse(
        &mut self,
        stack: &mut VecDeque<(String, char)>,
        received_pulse: char,
        flip_flop_states: &mut HashMap::<String, usize>,
        counter: usize,
    ) -> Vec<(String, char)> {
        let mut sent_pulses = Vec::<(String, char)>::new();
        if self.name == "button" {
            self.send_pulse('L', "broadcaster".to_string(), stack, &mut sent_pulses);
        } else if self.name == "broadcaster" {
            for target in self.targets.clone() {
                self.send_pulse(received_pulse, target.to_string(), stack, &mut sent_pulses);
            }
        } else if self.mod_type == '%' {
            if received_pulse == 'L' {
                for target in self.targets.clone() {
                    let next_pulse = if self.is_off() { 'H' } else { 'L' };
                    self.send_pulse(next_pulse, target.to_string(), stack, &mut sent_pulses);
                }
                self.toggle(flip_flop_states, counter);
            }
        } else if self.mod_type == '&' {
            if self.received_pulses.values().all(|x| x == &'H') {
                for target in self.targets.clone() {
                    self.send_pulse('L', target.to_string(), stack, &mut sent_pulses);
                }
            } else {
                for target in self.targets.clone() {
                    self.send_pulse('H', target.to_string(), stack, &mut sent_pulses);
                }
            }
        }
        sent_pulses
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} -> {}", self.mod_type, self.name, self.on)
    }
}

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);

    // Build modules list from input
    let mut modules = HashMap::<String, Module>::new();
    for line in data.lines() {
        let (a, b) = line.split_once(" -> ").unwrap();
        let targets = b
            .split(", ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        if a == "broadcaster" {
            modules.insert(a.to_string(), Module::new('B', a.to_string(), targets));
        } else {
            let (mod_type, name) = a.split_at(1);
            modules.insert(
                name.to_string(),
                Module::new(mod_type.chars().next().unwrap(), name.to_string(), targets),
            );
        }
    }
    // Add button in  modules
    modules.insert(
        "button".to_string(),
        Module::new('S', "button".to_string(), vec!["broadcaster".to_string()]),
    );

    // initiate received pulses for modules
    let mut modules_targets = HashMap::new();
    for module in modules.values() {
        modules_targets.insert(module.name.to_string(), module.targets.clone());
    }
    for module in modules.values_mut() {
        module.initiate_input_modules(&modules_targets);
    }

    // Identify rx % inputs (all that go to hp)
    let mut rx_flip_flops: Vec<String> = Vec::new();
    let mut candidates = Vec::<String>::new();
    candidates.push("rx".to_string());
    while !candidates.is_empty() {
        let candidate = candidates.pop().unwrap();
        for module in modules.values() {
            if module.targets.contains(&candidate) {
                if module.mod_type == '%' {
                    rx_flip_flops.push(module.name.to_string());
                } else {
                    candidates.push(module.name.to_string());
                }
            }
        }
    }
    // All rx flips flops need to be ON at the same time

    // Initiate stack
    let mut stack = VecDeque::<(String, char)>::new();

    // Process stack 1000 times
    let mut counter = 0;
    let mut rx_flip_flop_states = HashMap::<String, usize>::new();
    for rx_flip_flop in rx_flip_flops {
        rx_flip_flop_states.insert(rx_flip_flop, 0);
    }

    for _ in 0..10000 {
        println!("Flip flop states: {:?}", rx_flip_flop_states);
        // if stack.contains(&("rx".to_string(), 'L')) {
        //     return counter;
        // }
        counter += 1;
        println!("counter: {}", counter);
        stack.push_back(("button".to_string(), 'L'));
        while !stack.is_empty() {
            let (name, pulse) = stack.pop_front().unwrap();
            let module = &mut modules.get_mut(&name);
            match module {
                Some(module) => {
                    let sent_pulses = module.process_pulse(&mut stack, pulse,
                        &mut rx_flip_flop_states, counter);
                    for (target, pulse) in sent_pulses {
                        let target_module = modules.get_mut(&target);
                        if let Some(target_module) = target_module {
                            target_module.receive_pulse(pulse, &name);
                        }
                    }
                }
                None => ()
            }
        }
    }


    rx_flip_flop_states.values().fold(0, |acc, x| if acc == 0 { *x } else { gcd(acc, *x) })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // #[ignore]
    fn test_solution() {
        assert_eq!(0, solve_puzzle("input"));
    }
}



// Flip flop states: {
    // "cz": 8,
    //  "rv": 256,
    //  "fh": 32,
    //  "tb": 1024,
    //  "lr": 64,
    //  "dl": 2048,
    //  "nt": 1024,
    //  "xt": 1,
    //  "kr": 2048,
    //  "hd": 1,
    //  "kb": 512,
    //  "rq": 4,
    //  "xs": 64,
    //  "jh": 256,
    //  "kj": 1,
    //  "gr": 8,
    //  "pd": 256,
    //   "fm": 64,
    //   "ph": 2048,
    //   "rg": 1024,
    //   "rd": 4,
    //   "dk": 512,
    //   "hs": 2,
    //   "pj": 256,
    //   "gf": 16,
    //   "zn": 16,
    //   "nf": 1024,
    //   "bz": 512,
    //   "zt": 1,
    //   "qx": 32,
    //   "tp": 2,
    //   "jm": 128,
    //   "ck": 512,
    //   "cr": 4,
    //   "gd": 2048,
    //   "fc": 16}



    // GK
    // "zt": 1,
    // "cz": 8,
    //  "nt": 1024,
    //  "kb": 512,
    //  "pd": 256,
    //   "fm": 64,
    //   "rd": 4,
    //   "ph": 2048,
    // LCM 2048

    // GX
    //   "nf": 1024,
    //   "gd": 2048,
    //  "kj": 1,
    //  "lr": 64,
    //   "ck": 512,
    //  "rv": 256,
    //   "gf": 16,
    //   "hs": 2,

    // TF
    //  "xt": 1,
    //   "jm": 128,
    //  "kr": 2048,
    //   "dk": 512,
    //   "cr": 4,
    //  "tb": 1024,
    //   "fc": 16
    //   "qx": 32,
    //   "pj": 256,

    // XR
    //  "hd": 1,
    //   "rg": 1024,
    //  "fh": 32,
    //  "gr": 8,
    //  "jh": 256,
    //   "tp": 2,
    //  "rq": 4,
    //  "dl": 2048,
    //  "xs": 64,
    //   "bz": 512,
    //   "zn": 16,

