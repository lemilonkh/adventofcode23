extern crate nom;

use num::integer::lcm;
use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::alphanumeric1,
    sequence::{delimited, separated_pair},
    IResult,
};

fn line_parser(i: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        ),
    )(i)
}

fn part1(input: &str) -> u64 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(parts.len(), 2);
    let steps: Vec<char> = parts[0].chars().collect();

    let nodes: BTreeMap<&str, (&str, &str)> = parts[1]
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(line_parser)
        .map(|l| l.expect("valid input line").1)
        .collect();

    let start_nodes: Vec<&str> = nodes.keys().filter(|k| k.ends_with("A")).cloned().collect();
    let step_counts = start_nodes.iter().map(|start_node| {
        let mut current_node: &str = start_node;
        let mut step_counter: u32 = 0;

        while !current_node.ends_with("Z") {
            let direction = steps[step_counter as usize % steps.len()];

            let next_nodes = nodes.get(current_node).expect("node found");
            current_node = match direction {
                'L' => next_nodes.0,
                'R' => next_nodes.1,
                _ => {
                    eprintln!("Invalid direction {}", direction);
                    "INVALID"
                }
            };

            step_counter += 1;
        }
        step_counter
    }).inspect(|c| print!("{}, ", c));

    // calculate lowest common multiple of all step counts
    step_counts.fold(1, |acc, count| lcm(acc, count as u64))
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
    fn parse_line() {
        let result = line_parser("AAA = (BBB, CCC)").unwrap().1;
        assert_eq!(result, ("AAA", ("BBB", "CCC")));
    }

    #[test]
    fn it_works() {
        let result = part1(include_str!("input3_test.txt"));
        assert_eq!(result, 6);
    }
}
