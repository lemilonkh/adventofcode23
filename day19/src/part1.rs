use std::{collections::HashMap, str::FromStr};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, one_of},
    combinator::{map_res, opt},
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Rule<'a> {
    stat: char,
    is_gt: bool,
    value: u32,
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

fn part_parser(i: &str) -> IResult<&str, Part> {
    let (i, stats) = delimited(
        tag("{"),
        separated_list1(char(','), separated_pair(alpha1, char('='), int_parser)),
        tag("}"),
    )(i)?;

    Ok((
        i,
        Part {
            x: stats[0].1,
            m: stats[1].1,
            a: stats[2].1,
            s: stats[3].1,
        },
    ))
}

fn int_parser(i: &str) -> IResult<&str, u32> {
    map_res(digit1, FromStr::from_str)(i)
}

fn part1(input: &str) -> u32 {
    let (workflows_input, parts_input) = input.split_once("\n\n").expect("valid input");
    let workflows: HashMap<&str, Vec<Rule>> = workflows_input
        .lines()
        .map(|line| workflow_parser(line).expect("valid input").1)
        .collect();

    parts_input
        .lines()
        .map(|line| part_parser(line).expect("valid input").1)
        .filter_map(|part| {
            let mut current_workflow = "in";
            loop {
                if current_workflow == "A" {
                    return Some(part.x + part.m + part.a + part.s);
                } else if current_workflow == "R" {
                    return None;
                }

                let rules = workflows.get(current_workflow).expect("workflow found");
                for rule in rules {
                    if rule.is_fallback {
                        current_workflow = rule.target;
                        break;
                    }

                    let value = match rule.stat {
                        'x' => part.x,
                        'm' => part.m,
                        'a' => part.a,
                        's' => part.s,
                        _ => {
                            eprintln!("Invalid stat {}", rule.stat);
                            0
                        }
                    };

                    if (value > rule.value && rule.is_gt) || (value < rule.value && !rule.is_gt) {
                        current_workflow = rule.target;
                        break;
                    }
                }
            }
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
        assert_eq!(result, 19114);
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

    #[test]
    fn parse_part() {
        let (i, part) = part_parser("{x=1679,m=44,a=2067,s=496}").unwrap();
        assert_eq!(i, "");
        assert_eq!(part, Part { x: 1679, m: 44, a: 2067, s: 496 });
    }
}
