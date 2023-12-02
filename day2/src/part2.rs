extern crate nom;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1},
    multi::{separated_list0, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::cmp;

#[derive(Default, Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    bags: Vec<Bag>,
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

fn line_parser(i: &str) -> IResult<&str, Game> {
    dbg!(i);
    let (i, parsed) = separated_pair(
        separated_pair(tag("Game"), char(' '), digit1),
        tag(": "),
        game_parser,
    )(i)?;
    Ok((
        i,
        Game {
            id: parsed.0 .1.parse().expect("valid game ID"),
            bags: parsed.1,
        },
    ))
}

fn game_parser(i: &str) -> IResult<&str, Vec<Bag>> {
    separated_list1(tag("; "), bag_parser)(i)
}

fn bag_parser(i: &str) -> IResult<&str, Bag> {
    let (i, color_list) = separated_list0(tag(", "), separated_pair(digit1, char(' '), alpha1))(i)?;
    let mut bag = Bag {
        red: 0,
        green: 0,
        blue: 0,
    };
    for color in color_list {
        let num: u32 = color.0.parse().expect("valid cube amount");
        match color.1 {
            "red" => bag.red = num,
            "green" => bag.green = num,
            "blue" => bag.blue = num,
            _ => eprintln!("Invalid line!"),
        }
    }
    Ok((i, bag))
}

fn part2(input: &str) -> u32 {
    input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(line_parser)
        .map(|result| {
            result
                .as_ref()
                .map(|(_, game)| {
                    game.bags.iter().fold(
                        Bag {
                            red: 0,
                            green: 0,
                            blue: 0,
                        },
                        |max_bag, bag| Bag {
                            red: cmp::max(max_bag.red, bag.red),
                            green: cmp::max(max_bag.green, bag.green),
                            blue: cmp::max(max_bag.blue, bag.blue),
                        },
                    )
                })
                .expect("valid line")
        })
        .fold(0, |sum, bag| sum + bag.red * bag.green * bag.blue)
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line() {
        let result = line_parser("Game 1: 3 blue, 4 red; 8 green, 5 blue")
            .unwrap()
            .1;
        assert_eq!(
            result,
            Game {
                id: 1,
                bags: vec!(
                    Bag {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    Bag {
                        red: 0,
                        green: 8,
                        blue: 5
                    }
                )
            }
        );
    }

    #[test]
    fn parse_game() {
        let result = game_parser("3 blue, 4 red; 8 green, 5 blue").unwrap().1;
        assert_eq!(
            result,
            vec!(
                Bag {
                    red: 4,
                    green: 0,
                    blue: 3
                },
                Bag {
                    red: 0,
                    green: 8,
                    blue: 5
                }
            )
        );
    }

    #[test]
    fn parse_bag() {
        let result = bag_parser("3 blue, 4 red").unwrap().1;
        assert_eq!(
            result,
            Bag {
                red: 4,
                blue: 3,
                green: 0
            }
        );
    }

    #[test]
    fn it_works() {
        let result = part2(include_str!("input2_test.txt"));
        assert_eq!(result, 2286);
    }
}
