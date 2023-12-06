extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

fn line_parser(i: &str) -> IResult<&str, u64> {
    dbg!(i);
    preceded(
        alt((tag("Time:"), tag("Distance:"))),
        number_with_spaces_parser,
    )(i)
}

fn number_with_spaces_parser(i: &str) -> IResult<&str, u64> {
    let (i, list) = preceded(space1, separated_list1(space1, digit1))(i)?;
    let numbers = list.join("").parse().expect("valid number");
    Ok((i, numbers))
}

fn part1(input: &str) -> u64 {
    let parts: Vec<u64> = input
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(line_parser)
        .map(|l| l.expect("valid input line").1)
        .collect();
    assert_eq!(parts.len(), 2);

    let time = parts[0];
    let best_distance = parts[1];
    let mut ways_to_beat_record: u64 = 0;
    for r in 1..time {
        let distance = r * (time - r);
        if distance > best_distance {
            ways_to_beat_record += 1;
        }
    }
    dbg!(ways_to_beat_record)
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
        let result = line_parser("Time:        46     80     78     66")
            .unwrap()
            .1;
        assert_eq!(result, 46807866);
    }

    #[test]
    fn it_works() {
        let result = part1(include_str!("input1_test.txt"));
        assert_eq!(result, 71503);
    }
}
