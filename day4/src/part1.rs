extern crate nom;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Default, Debug, PartialEq, Eq)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

fn line_parser(i: &str) -> IResult<&str, Card> {
    let (i, card) = separated_pair(tag("Card"), space1, digit1)(i)?;
    let (i, _) = preceded(char(':'), space1)(i)?;
    let (i, winning_numbers) = number_list_parser(i)?;
    let (i, _) = preceded(tag(" |"), space1)(i)?;
    let (i, card_numbers) = number_list_parser(i)?;
    Ok((
        i,
        Card {
            id: card.1.parse().expect("valid game ID"),
            winning_numbers,
            card_numbers,
        },
    ))
}

fn number_list_parser(i: &str) -> IResult<&str, Vec<u32>> {
    let (i, list) = separated_list1(space1, digit1)(i)?;
    let numbers = list
        .iter()
        .map(|str| str.parse().expect("valid number"))
        .collect();
    Ok((i, numbers))
}

fn part1(input: &str) -> i32 {
    input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(line_parser)
        .map(|result| {
            result
                .as_ref()
                .map(|(_, card): &(&str, Card)| -> i32 {
                    let matches = card
                        .card_numbers
                        .iter()
                        .map(|number| {
                            if card.winning_numbers.contains(number) {
                                1
                            } else {
                                0
                            }
                        })
                        .sum::<i32>();
                    if matches < 1 {
                        return 0;
                    }
                    let points = 1 << (matches - 1);
                    dbg!(points)
                })
                .expect("valid line")
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
    fn parse_line() {
        let result = line_parser("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
            .unwrap()
            .1;
        assert_eq!(
            result,
            Card {
                id: 1,
                winning_numbers: vec!(41, 48, 83, 86, 17),
                card_numbers: vec!(83, 86, 6, 31, 17, 9, 48, 53),
            }
        );
    }

    #[test]
    fn it_works() {
        let result = part1(include_str!("input1_test.txt"));
        assert_eq!(result, 13);
    }
}
