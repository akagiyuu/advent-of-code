use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pulse {
    Low,
    High,
}
#[derive(Default, Debug, Clone)]
struct FlipFlop {
    is_on: bool,
    destinations: Vec<String>,
}
impl FlipFlop {
    fn respond_to_pulse(&mut self, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => {
                self.is_on = !self.is_on;
                Some(if self.is_on { Pulse::High } else { Pulse::Low })
            }
        }
    }
}
#[derive(Default, Debug, Clone)]
struct Conjunction {
    inputs_count: usize,
    high_pulse_inputs: HashSet<String>,
    destinations: Vec<String>,
}
impl Conjunction {
    fn respond_to_pulse(&mut self, sender: String, pulse: Pulse) -> Pulse {
        if pulse == Pulse::High {
            self.high_pulse_inputs.insert(sender);
        } else {
            self.high_pulse_inputs.remove(&sender);
        }
        if self.high_pulse_inputs.len() == self.inputs_count {
            Pulse::Low
        } else {
            Pulse::High
        }
    }
}
#[derive(Default, Debug, Clone)]
struct BroadCaster {
    destinations: Vec<String>,
}
#[derive(Debug, Clone)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    BroadCaster(BroadCaster),
}
fn parse_modules(modules_raw: &str) -> HashMap<String, Module> {
    let mut modules = modules_raw
        .lines()
        .map(|value| {
            let (identifier, destinations) = value.split_once(" -> ").unwrap();
            let destinations = destinations
                .split(", ")
                .map(|destination| destination.to_string())
                .collect::<Vec<_>>();
            match identifier.as_bytes()[0] {
                b'b' => (
                    identifier.to_string(),
                    Module::BroadCaster(BroadCaster { destinations }),
                ),
                b'%' => (
                    identifier[1..].to_string(),
                    Module::FlipFlop(FlipFlop {
                        destinations,
                        ..Default::default()
                    }),
                ),
                b'&' => (
                    identifier[1..].to_string(),
                    Module::Conjunction(Conjunction {
                        destinations,
                        ..Default::default()
                    }),
                ),
                _ => unreachable!(),
            }
        })
        .collect::<HashMap<_, _>>();
    for (_, module) in modules.clone() {
        let destinations = match module {
            Module::FlipFlop(flip_flop) => flip_flop.destinations,
            Module::Conjunction(conjunction) => conjunction.destinations,
            Module::BroadCaster(broadcaster) => broadcaster.destinations,
        };
        for destination in destinations {
            if let Some(Module::Conjunction(conjunction)) = modules.get_mut(&destination) {
                conjunction.inputs_count += 1;
            }
        }
    }
    modules
}

fn is_default_modules(modules: &HashMap<String, Module>) -> bool {
    modules.iter().all(|(_, value)| match value {
        Module::BroadCaster(_) => true,
        Module::FlipFlop(flip_flop) => !flip_flop.is_on,
        Module::Conjunction(conjunction) => conjunction.high_pulse_inputs.is_empty(),
    })
}

fn calculate_metric(modules: HashMap<String, Module>, repeat_time: usize) -> usize {
    let mut modules = modules;
    let mut cycle_length = 1;
    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;
    let mut cached_metric = vec![1];
    let mut queue = VecDeque::<(String, String, Pulse)>::new();
    while cycle_length <= repeat_time {
        queue.clear();
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
        low_pulse_count += 1;
        while let Some((sender, receiver, pulse)) = queue.pop_front() {
            let Some(module) = modules.get_mut(&receiver) else {
                continue;
            };
            let (new_pulse, destinations) = match module {
                Module::BroadCaster(broadcaster) => (pulse, &broadcaster.destinations),
                Module::FlipFlop(flip_flop) => {
                    let pulse = flip_flop.respond_to_pulse(pulse);
                    let Some(pulse) = pulse else {
                        continue;
                    };
                    (pulse, &flip_flop.destinations)
                }
                Module::Conjunction(conjunction) => {
                    let pulse = conjunction.respond_to_pulse(sender, pulse);
                    (pulse, &conjunction.destinations)
                }
            };

            if new_pulse == Pulse::High {
                high_pulse_count += destinations.len();
            } else {
                low_pulse_count += destinations.len();
            }
            for destination in destinations {
                queue.push_back((receiver.clone(), destination.clone(), new_pulse));
            }
        }
        cached_metric.push(low_pulse_count * high_pulse_count);
        if is_default_modules(&modules) {
            break;
        }
        cycle_length += 1;
    }
    if cycle_length >= repeat_time {
        return low_pulse_count * high_pulse_count;
    }
    cached_metric[repeat_time % cycle_length]
        * low_pulse_count
        * high_pulse_count
        * (repeat_time / cycle_length).pow(2)
}

/// Unable to calculate fewest cycle in acceptable time
fn calculate_fewest_cycle(modules: HashMap<String, Module>) -> usize {
    let mut modules = modules;
    let mut cycle_length = 1;
    let mut queue = VecDeque::<(String, String, Pulse)>::new();
    loop {
        queue.clear();
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
        while let Some((sender, receiver, pulse)) = queue.pop_front() {
            if receiver == "rx" && pulse == Pulse::Low {
                return cycle_length;
            }
            let Some(module) = modules.get_mut(&receiver) else {
                continue;
            };
            let (new_pulse, destinations) = match module {
                Module::BroadCaster(broadcaster) => (pulse, &broadcaster.destinations),
                Module::FlipFlop(flip_flop) => {
                    let pulse = flip_flop.respond_to_pulse(pulse);
                    let Some(pulse) = pulse else {
                        continue;
                    };
                    (pulse, &flip_flop.destinations)
                }
                Module::Conjunction(conjunction) => {
                    let pulse = conjunction.respond_to_pulse(sender, pulse);
                    (pulse, &conjunction.destinations)
                }
            };

            for destination in destinations {
                queue.push_back((receiver.clone(), destination.clone(), new_pulse));
            }
        }
        cycle_length += 1;
    }
}

/// Gather data to calculate part 2 by hand
fn gather_data(modules: HashMap<String, Module>) -> ! {
    let mut modules = modules;
    let mut cycle_length = 1;
    let mut queue = VecDeque::<(String, String, Pulse)>::new();
    let target_modules = ["lk", "fn", "fh", "hh"];
    loop {
        queue.clear();
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
        while let Some((sender, receiver, pulse)) = queue.pop_front() {
            if target_modules.contains(&receiver.as_str()) && pulse == Pulse::Low {
                println!("{}, {}", receiver, cycle_length);
            }
            let Some(module) = modules.get_mut(&receiver) else {
                continue;
            };
            let (new_pulse, destinations) = match module {
                Module::BroadCaster(broadcaster) => (pulse, &broadcaster.destinations),
                Module::FlipFlop(flip_flop) => {
                    let pulse = flip_flop.respond_to_pulse(pulse);
                    let Some(pulse) = pulse else {
                        continue;
                    };
                    (pulse, &flip_flop.destinations)
                }
                Module::Conjunction(conjunction) => {
                    let pulse = conjunction.respond_to_pulse(sender, pulse);
                    (pulse, &conjunction.destinations)
                }
            };

            for destination in destinations {
                queue.push_back((receiver.clone(), destination.clone(), new_pulse));
            }
        }
        cycle_length += 1;
    }
}

fn main() {
    let modules = parse_modules(INPUT);
    let metric = calculate_metric(modules.clone(), 1000);
    println!("{}", metric);
    gather_data(modules.clone());
}
