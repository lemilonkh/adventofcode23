use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

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

    // all flip flops reachable from the network root module
    let mut groups: BTreeMap<&str, HashSet<&str>> = BTreeMap::new();
    let mut queue = VecDeque::new();
    let mut visited: Vec<&str> = vec![];
    queue.push_back(("roadcaster", None));

    while !queue.is_empty() {
        let (name, group) = queue.pop_front().unwrap();
        visited.push(name);
        if !modules.contains_key(name) {
            continue;
        }
        let module = modules.get(name).unwrap();
        if module.module_type == ModuleType::FlipFlop {
            groups
                .entry(group.unwrap()) // safe because there are no flip flops at top level
                .or_insert(HashSet::new())
                .insert(name);
        }
        for connection_name in module.connections.iter() {
            let group = group.unwrap_or(connection_name);

            if !visited.contains(connection_name) {
                queue.push_back((connection_name, Some(group)));
            }
        }
    }

    println!("Groups {:?}", groups);

    // network root module name to history of hashes
    let mut group_histories: BTreeMap<&str, Vec<u64>> = BTreeMap::new();
    // network root module name to length of cycle
    let mut group_cycles: BTreeMap<&str, u64> = BTreeMap::new();
    let mut i = 0;

    for &group_name in groups.keys() {
        group_histories.insert(group_name, vec![]);
    }

    while group_cycles.len() < groups.len() {
        let mut queue = VecDeque::new();
        queue.push_back((false, "roadcaster", "")); // button press

        /*println!(
            "Iteration {}, FlipFlops {}, {}",
            i,
            flip_flop_state
                .values()
                .map(|s| if *s { '1' } else { '0' })
                .collect::<String>(),
            flip_flop_state.len()
        );*/

        for (&group_name, group_modules) in groups.iter() {
            if group_cycles.contains_key(group_name) {
                continue;
            }

            // concat binary number from flip flop states
            let history_entry = group_modules
                .iter()
                .map(|&module_name| *flip_flop_state.get(module_name).unwrap_or(&false))
                .fold(0, |acc, s| (acc << 1) | s as u64);

            let history = group_histories.get_mut(group_name).unwrap();
            if history.contains(&history_entry) {
                group_cycles.insert(group_name, i);
            }

            history.push(history_entry);
        }

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
                        pulse_type = !prev_state;
                    } else {
                        continue; // don't send output pulse for high input
                    }
                }
                ModuleType::Nand => {
                    let state = nand_state.get_mut(name).unwrap();
                    state.insert(prev_name, is_high);
                    let result = state.values().fold(true, |acc, s| acc && *s); // AND all inputs
                    pulse_type = !result; // NOT
                }
            }

            for connection_name in module.connections.iter() {
                queue.push_back((pulse_type, connection_name, name));
            }
        }

        i += 1;
    }

    // rx is activated when all groups repeat at the same time (so they all have sent out a high pulse last)
    println!("Group cycles {:?}", group_cycles);
    group_cycles.values().product()
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
        assert_eq!(result, 1);
        let result = part1(include_str!("input2_test.txt"));
        assert_eq!(result, 4);
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
