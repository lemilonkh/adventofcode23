use std::collections::{HashMap, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, one_of},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

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

    let mut flip_flop_state: HashMap<&str, bool> = HashMap::new();
    let mut nand_state: HashMap<&str, HashMap<&str, bool>> = HashMap::new();

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

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _i in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back((false, "roadcaster", "")); // button press

        while !queue.is_empty() {
            let (is_high, name, prev_name) = queue.pop_front().unwrap();
            if is_high {
                high_pulses += 1;
            } else {
                low_pulses += 1;
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
    }

    low_pulses * high_pulses
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
