use std::{
    cmp::{max, min},
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, one_of},
    combinator::{map_res, opt},
    multi::separated_list1,
    sequence::{delimited, pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Part {
    min: [u64; 4],
    max: [u64; 4],
}

impl Default for Part {
    fn default() -> Self {
        Self {
            min: [1, 1, 1, 1],
            max: [4000, 4000, 4000, 4000],
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rule<'a> {
    stat: char,
    is_gt: bool,
    value: u64,
    target: &'a str,
    is_fallback: bool,
}

fn workflow_parser(i: &str) -> IResult<&str, (&str, Vec<Rule>)> {
    let (i, (name, rules)) = tuple((
        alpha1,
        delimited(tag("{"), separated_list1(char(','), rule_parser), tag("}")),
    ))(i)?;
    Ok((i, (name, rules)))
}

fn rule_parser(i: &str) -> IResult<&str, Rule> {
    let (i, (parts, target)) = pair(
        opt(tuple((alpha1, one_of("<>"), int_parser, tag(":")))),
        alpha1,
    )(i)?;

    let rule = if let Some((stat, operator, value, _)) = parts {
        Rule {
            stat: stat.chars().nth(0).expect("found stat"),
            is_gt: operator == '>',
            value,
            target,
            is_fallback: false,
        }
    } else {
        Rule {
            stat: '?',
            is_gt: false,
            value: 0,
            target,
            is_fallback: true,
        }
    };

    Ok((i, rule))
}

fn int_parser(i: &str) -> IResult<&str, u64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn part1(input: &str) -> u64 {
    let (workflows_input, _) = input.split_once("\n\n").expect("valid input");
    let workflows: HashMap<&str, Vec<Rule>> = workflows_input
        .lines()
        .map(|line| workflow_parser(line).expect("valid input").1)
        .collect();

    let mut queue = VecDeque::new();
    queue.push_back(("in", Part::default()));
    let mut accepted_parts = vec![];

    while !queue.is_empty() {
        let (current_workflow, part) = queue.pop_front().unwrap();
        println!("Workflow: {}, Part range: {:?}", current_workflow, part);

        if current_workflow == "A" {
            accepted_parts.push(part);
            continue;
        } else if current_workflow == "R" {
            continue;
        }

        let rules = workflows.get(current_workflow).expect("workflow found");
        let mut current_part = part.clone();
        let mut next_part = part.clone();
        for rule in rules {
            if rule.is_fallback {
                queue.push_back((rule.target, current_part));
                break;
            }

            let stat = "xmas".find(rule.stat).expect("valid stat");

            if rule.is_gt {
                current_part.min[stat] = max(current_part.min[stat], rule.value + 1);
                next_part.max[stat] = min(next_part.max[stat], rule.value);
            } else {
                current_part.max[stat] = min(current_part.max[stat], rule.value - 1);
                next_part.min[stat] = max(next_part.min[stat], rule.value);
            }

            queue.push_back((rule.target, current_part));
            current_part = next_part.clone();
        }
    }

    accepted_parts
        .iter()
        .map(|part| {
            (0..4)
                .map(|i| part.max[i] - part.min[i] + 1)
                .product::<u64>()
        })
        .sum()
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
        assert_eq!(result, 167409079868000);
    }

    #[test]
    fn parse_rule() {
        let (i, rule) = rule_parser("m>1548:R").unwrap();
        assert_eq!(i, "");
        assert_eq!(
            rule,
            Rule {
                stat: 'm',
                is_gt: true,
                value: 1548,
                target: "R",
                is_fallback: false
            }
        );

        let (i, rule) = rule_parser("A").unwrap();
        assert_eq!(i, "");
        assert_eq!(
            rule,
            Rule {
                stat: '?',
                is_gt: false,
                value: 0,
                target: "A",
                is_fallback: true
            }
        );
    }

    #[test]
    fn parse_workflow() {
        let (i, (name, rules)) = workflow_parser("px{a<2006:qkq,m>2090:A,rfg}").unwrap();
        assert_eq!(i, "");
        assert_eq!(name, "px");
        assert_eq!(
            rules,
            vec!(
                Rule {
                    stat: 'a',
                    is_gt: false,
                    value: 2006,
                    target: "qkq",
                    is_fallback: false
                },
                Rule {
                    stat: 'm',
                    is_gt: true,
                    value: 2090,
                    target: "A",
                    is_fallback: false
                },
                Rule {
                    stat: '?',
                    is_gt: false,
                    value: 0,
                    target: "rfg",
                    is_fallback: true
                },
            )
        );
    }
}
