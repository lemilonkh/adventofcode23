extern crate nom;

use cached::proc_macro::cached;
use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, one_of, space1},
    combinator::{map_res, recognize},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

fn line_parser(i: &str) -> IResult<&str, (&str, Vec<usize>)> {
    separated_pair(recognize(many1(one_of(".#?"))), space1, number_list_parser)(i)
}

fn number_list_parser(i: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(","), int_parser)(i)
}

fn int_parser(i: &str) -> IResult<&str, usize> {
    map_res(digit1, FromStr::from_str)(i)
}

fn part1(input: &str) -> usize {
    let unfolded_input = input.lines().fold(String::new(), |mut acc, line| {
        let (c, g) = line.split_once(" ").expect("space-separated line");
        acc.push_str(&format!("{c}?{c}?{c}?{c}?{c} {g},{g},{g},{g},{g}\n"));
        acc
    });

    unfolded_input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line_parser(line).expect("valid input").1)
        .map(|(conditions, groups)| permutations(conditions.as_bytes(), None, &groups))
        .sum()
}

#[cached(
    key = "String",
    convert = r#"{format!("{:?}{:?}{:?}", input, size, groups)}"#
)]
fn permutations(input: &[u8], size: Option<usize>, groups: &[usize]) -> usize {
    if input.is_empty() {
        return match size {
            // final group has the right size
            Some(n) if groups == &[n] => 1,
            // all groups matched
            None if groups.is_empty() => 1,
            // unmatched groups remaining or wrong size, no possible permutations
            _ => 0,
        };
    }

    match (input[0], size, groups) {
        // throw away consecutive dots and question marks after all groups are resolved
        (b'.', None, _) | (b'?', None, []) => permutations(&input[1..], None, groups),
        // if the current group amount has been matched, move on to the next group
        (b'.' | b'?', Some(n), [e, ..]) if n == *e => permutations(&input[1..], None, &groups[1..]),
        // increase size of current group if it hasn't been matched yet
        (b'#' | b'?', Some(n), [e, ..]) if n < *e => permutations(&input[1..], Some(n + 1), groups),
        // start a new group
        (b'#', None, [_, ..]) => permutations(&input[1..], Some(1), groups),
        // calculate both branches for a ? at the start of a group - as . or as #
        (b'?', None, _) => {
            permutations(&input[1..], None, groups) + permutations(&input[1..], Some(1), groups)
        }
        // group not matched, no possible permutations
        _ => 0,
    }
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
    fn parse_input() {
        let result = line_parser("???.### 1,1,3").unwrap().1;
        assert_eq!(result, ("???.###", vec!(1, 1, 3)));
    }

    #[test]
    fn it_works() {
        let result = part1(include_str!("input1_test.txt"));
        assert_eq!(result, 525152);
    }
}
