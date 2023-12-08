extern crate nom;

use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    sequence::{delimited, separated_pair},
    IResult,
};

fn line_parser(i: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        alpha1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alpha1, tag(", "), alpha1),
            tag(")"),
        ),
    )(i)
}

fn part1(input: &str) -> u32 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(parts.len(), 2);
    let steps: Vec<char> = parts[0].chars().collect();

    let nodes: HashMap<&str, (&str, &str)> = parts[1]
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(line_parser)
        .map(|l| l.expect("valid input line").1)
        .collect();
    assert_ne!(nodes.get("AAA"), None);
    assert_ne!(nodes.get("ZZZ"), None);

    let mut current_node: &str = "AAA";
    let mut step_index: usize = 0;
    let mut step_counter: u32 = 0;
    while current_node != "ZZZ" {
        let direction = steps[step_index];
        step_index += 1;
        if step_index >= steps.len() {
            step_index = 0;
        }

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
        // println!("Visiting {}, went {}", current_node, direction);
    }

    step_counter
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
        let result = part1(include_str!("input1_test.txt"));
        assert_eq!(result, 2);
        let result = part1(include_str!("input2_test.txt"));
        assert_eq!(result, 6);
    }
}
