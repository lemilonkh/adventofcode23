extern crate nom;

use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::{map_res, opt, recognize},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

fn input_parser(i: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(newline, number_list_parser)(i)
}

fn number_list_parser(i: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, int_parser)(i)
}

fn int_parser(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(preceded(opt(tag("-")), digit1)),
        FromStr::from_str,
    )(i)
}

fn part1(input: &str) -> i64 {
    let histories = input_parser(input).expect("valid input").1;
    histories
        .into_iter()
        .map(|history| {
            let mut last_numbers: Vec<i64> = vec![*history.last().expect("non-empty line")];
            let mut prev_sequence = history;
            loop {
                let sequence: Vec<i64> = prev_sequence.windows(2).map(|w| w[1] - w[0]).collect();
                let all_zeroes = sequence.iter().all(|n| *n == 0);

                if all_zeroes {
                    break;
                } else {
                    last_numbers.push(*sequence.last().expect("non-empty sequence"));
                    prev_sequence = sequence;
                }
            }

            let next_value = last_numbers
                .iter()
                .rev()
                .fold(0, |acc, last_number| last_number + acc);
            dbg!(next_value)
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
    fn parse_input() {
        let result = input_parser(
            "-1 0 3 6
-16 0 4 16",
        )
        .unwrap()
        .1;
        assert_eq!(result, vec!(vec!(-1, 0, 3, 6), vec!(-16, 0, 4, 16)));
    }

    #[test]
    fn it_works() {
        let result = part1(include_str!("input1_test.txt"));
        assert_eq!(result, 114);
    }
}
