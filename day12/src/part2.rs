extern crate nom;

use std::str::FromStr;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, one_of, space1},
    combinator::{map_res, recognize},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

fn line_parser(i: &str) -> IResult<&str, (&[u8], Vec<u32>)> {
    let (i, (conditions, groups)) =
        separated_pair(recognize(many1(one_of(".#?"))), space1, number_list_parser)(i)?;

    Ok((i, (conditions.as_bytes(), groups)))
}

fn number_list_parser(i: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), int_parser)(i)
}

fn int_parser(i: &str) -> IResult<&str, u32> {
    map_res(digit1, FromStr::from_str)(i)
}

fn is_valid_permutation(conditions: &str, groups: &Vec<u32>, permutation: usize) -> bool {
    // println!("Perm: {:?}, Groups: {:?}", permutation, groups);
    let mut contiguous_damaged_springs: u32 = 0;
    let mut question_mark_count: u32 = 0;
    let mut group_iter = groups.iter();
    for mut spring in conditions.chars() {
        if spring == '?' {
            let bit = permutation & (1 << question_mark_count);
            question_mark_count += 1;
            spring = if bit > 0 { '.' } else { '#' };
        }
        match spring {
            '#' => contiguous_damaged_springs += 1,
            '.' => {
                if contiguous_damaged_springs > 0 {
                    let group_count = group_iter.next();
                    if group_count.is_none() {
                        return false;
                    }
                    if contiguous_damaged_springs != *group_count.unwrap() {
                        return false;
                    }
                    contiguous_damaged_springs = 0;
                }
            }
            _ => eprintln!("Invalid char in permutation: {}", spring),
        }
    }
    // if permutation ends on #, check last group count
    if contiguous_damaged_springs > 0 {
        let group_count = group_iter.next();
        if group_count.is_none() {
            return false;
        }
        if contiguous_damaged_springs != *group_count.unwrap() {
            return false;
        }
    }
    // if there should be more groups than there are in the input, return false
    group_iter.next().is_none()
}

fn apply_permutation(conditions: &str, unknown_indices: &Vec<usize>, permutation: usize) -> String {
    let mut condition_permutation = conditions.to_owned();
    for (i, &index) in unknown_indices.iter().enumerate() {
        let bit = permutation & (1 << i);
        condition_permutation.replace_range(index..index + 1, if bit > 0 { "." } else { "#" });
    }
    condition_permutation
}

fn part1(input: &str) -> usize {
    let rows = input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line_parser(line).expect("valid input").1)
        .collect::<Vec<_>>();

    rows.into_iter()
        .map(|(conditions, groups)| {
            let mut conditions = (conditions.to_owned() + "?").repeat(5);
            conditions.pop(); // remove last ?

            let groups = std::iter::repeat(groups.iter())
                .take(5)
                .flatten()
                .map(|g| *g)
                .collect::<Vec<_>>();

            let sequence_permutations = conditions
                .split(".")
                .map(|sequence| {
                    if !sequence.contains('?') {
                        vec![sequence.to_owned()]
                    } else {
                        let unknown_indices = conditions
                            .char_indices()
                            .filter_map(|(i, spring)| (spring == '?').then_some(i))
                            .collect::<Vec<_>>();
                        (0..(2_usize.pow(unknown_indices.len() as u32)))
                            .filter_map(|permutation| {
                                is_valid_permutation(&conditions, &groups, permutation).then_some(
                                    apply_permutation(&conditions, &unknown_indices, permutation),
                                )
                            })
                            .collect()
                    }
                })
                .collect::<Vec<_>>();

            let possibilities = 1_usize;
            for sequence in sequence_permutations {
                if sequence.len() == 1 {
                    continue;
                }
                todo!();
            }
            possibilities
        })
        .inspect(|count| println!("Valid permutations: {}", count))
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
    fn parse_input() {
        let result = line_parser("???.### 1,1,3").unwrap().1;
        assert_eq!(result, ("???.###", vec!(1, 1, 3)),);
    }

    #[test]
    fn it_works() {
        let result = part1(include_str!("input1_test.txt"));
        assert_eq!(result, 525152);
    }
}
