use std::collections::{BTreeMap, HashMap, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, one_of},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use num::integer::lcm;

#[derive(Debug, PartialEq, Eq)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Nand,
}

#[derive(Debug, PartialEq, Eq)]
struct Module<'a> {
    module_type: ModuleType,
    name: &'a str,
    connections: Vec<&'a str>,
}

fn line_parser(i: &str) -> IResult<&str, (&str, Module)> {
    let (i, (type_char, name, _, connections)) = tuple((
        one_of("b%&"),
        alpha1,
        tag(" -> "),
        separated_list1(tag(", "), alpha1),
    ))(i)?;
    Ok((
        i,
        (
            name,
            Module {
                name,
                connections,
                module_type: match type_char {
                    'b' => ModuleType::Broadcaster,
                    '%' => ModuleType::FlipFlop,
                    '&' => ModuleType::Nand,
                    _ => panic!("Invalid module type {}", type_char),
                },
            },
        ),
    ))
}

fn part1(input: &str) -> u64 {
    let modules: HashMap<&str, Module> = input
        .lines()
        .map(|line| line_parser(line).expect("valid input").1)
        .collect();

    let mut flip_flop_state: BTreeMap<&str, bool> = BTreeMap::new();
    let mut nand_state: HashMap<&str, HashMap<&str, bool>> = HashMap::new();

    // init flip flop states
    for module in modules.values() {
        if module.module_type == ModuleType::FlipFlop {
            flip_flop_state.insert(module.name, false);
        }
    }

    // init nand states (find all input connections)
    let nand_names = modules
        .iter()
        .filter_map(|(name, m)| (m.module_type == ModuleType::Nand).then_some(name))
        .collect::<Vec<_>>();

    for module in modules.values() {
        for nand_name in nand_names.iter() {
            if module.connections.contains(nand_name) {
                nand_state
                    .entry(nand_name)
                    .or_insert(HashMap::new())
                    .insert(module.name, false);
            }
        }
    }

    let mut flip_flop_cycles: BTreeMap<&str, u64> = BTreeMap::new();
    let mut i = 0;

    while flip_flop_cycles.len() < flip_flop_state.len() {
        let mut queue = VecDeque::new();
        queue.push_back((false, "roadcaster", "")); // button press
        println!(
            "Iteration {}, FlipFlops {}, {}",
            i,
            flip_flop_state
                .values()
                .map(|s| if *s { '1' } else { '0' })
                .collect::<String>(),
            flip_flop_state.len()
        );

        while !queue.is_empty() {
            let (is_high, name, prev_name) = queue.pop_front().unwrap();

            if name == "rx" && !is_high {
                return i;
            }

            let module = modules.get(name);
            if module.is_none() {
                continue;
            }
            let module = module.unwrap();
            let mut pulse_type = is_high;

            match module.module_type {
                ModuleType::Broadcaster => {}
                ModuleType::FlipFlop => {
                    if !is_high {
                        let prev_state = *flip_flop_state.entry(name).or_insert(false);
                        flip_flop_state.insert(name, !prev_state);
                        flip_flop_cycles.entry(name).or_insert(i + 1);
                        pulse_type = !prev_state;
                    } else {
                        continue; // don't send output pulse for high input
                    }
                }
                ModuleType::Nand => {
                    let state = nand_state.get_mut(name).unwrap();
                    state.insert(prev_name, is_high);
                    let result = state.values().fold(true, |acc, s| acc & *s); // AND all inputs
                    pulse_type = !result; // NOT
                }
            }

            for connection_name in module.connections.iter() {
                queue.push_back((pulse_type, connection_name, name));
            }
        }

        i += 1;
    }

    println!("Cycles {:?}", flip_flop_cycles);

    let mut cycle_pairs = Vec::from_iter(flip_flop_cycles.into_iter());
    cycle_pairs.sort_by_key(|(_name, cycle)| *cycle);
    cycle_pairs.dedup_by_key(|(_name, cycle)| *cycle);

    // determine correct combination to send low pulse to rx
    let mut module_inputs: HashMap<&str, Vec<&str>> = HashMap::new();
    for module in modules.values() {
        for connection_name in module.connections.iter() {
            module_inputs
                .entry(connection_name)
                .or_insert(vec![])
                .push(module.name);
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back((Some(false), "rx"));
    let mut flip_flop_values: BTreeMap<&str, Option<bool>> = BTreeMap::new();

    while !queue.is_empty() {
        let (is_high, name) = queue.pop_front().unwrap();
        if module_inputs.contains_key(name) {
            for input_name in module_inputs.get(name).unwrap() {
                let module = modules.get(input_name).expect("found module");

                match module.module_type {
                    ModuleType::Broadcaster => continue,
                    ModuleType::FlipFlop => {
                        flip_flop_values.insert(input_name, is_high);
                        // TODO only one of the inputs needs to have emitted a pulse => try out combinations?
                        queue.push_back((Some(false), input_name));
                    }
                    ModuleType::Nand => {
                        if is_high.is_some() {
                            if is_high.unwrap() {
                                queue.push_back((Some(true), input_name));
                            } else {
                                queue.push_back((None, input_name));
                            }
                        } else {
                            queue.push_back((None, input_name));
                        }
                    }
                }
            }
        }
    }

    println!("Values {:?}", flip_flop_values);

    todo!()
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(include_str!("input1_test.txt"));
        assert_eq!(result, 32000000);
        let result = part1(include_str!("input2_test.txt"));
        assert_eq!(result, 11687500);
    }

    #[test]
    fn parse_module() {
        let (i, (name, module)) = line_parser("%sf -> pz, gj").unwrap();
        assert_eq!(i, "");
        assert_eq!(name, "sf");
        assert_eq!(
            module,
            Module {
                name: "sf",
                module_type: ModuleType::FlipFlop,
                connections: vec!("pz", "gj"),
            }
        )
    }
}
