extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

fn line_parser(i: &str) -> IResult<&str, Vec<u32>> {
    dbg!(i);
    preceded(alt((tag("Time:"), tag("Distance:"))), number_list_parser)(i)
}

fn number_list_parser(i: &str) -> IResult<&str, Vec<u32>> {
    let (i, list) = preceded(space1, separated_list1(space1, digit1))(i)?;
    let numbers = list
        .iter()
        .map(|str| str.parse().expect("valid number"))
        .collect();
    Ok((i, numbers))
}

fn part1(input: &str) -> u32 {
    let parts: Vec<Vec<u32>> = input
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(line_parser)
        .map(|l| l.expect("valid input line").1)
        .collect();
    assert_eq!(parts.len(), 2);

    parts[0]
        .iter()
        .zip(parts[1].iter())
        .map(|(time, best_distance)| {
            let mut ways_to_beat_record: u32 = 0;
            for r in 1..*time {
                let distance = r * (time - r);
                if distance > *best_distance {
                    ways_to_beat_record += 1;
                }
            }
            dbg!(ways_to_beat_record)
        })
        .product()
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
        assert_eq!(result, vec!(46, 80, 78, 66),);
    }

    #[test]
    fn it_works() {
        let result = part1(include_str!("input1_test.txt"));
        assert_eq!(result, 288);
    }
}
