extern crate nom;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1},
    multi::{separated_list0, separated_list1},
    sequence::separated_pair,
    IResult,
};

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

static MAX_BAG: &'static Bag = &Bag {
    red: 12,
    green: 13,
    blue: 14,
};

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
            id: parsed.0 .1.parse().unwrap(),
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
        let num: u32 = color.0.parse().unwrap();
        match color.1 {
            "red" => bag.red = num,
            "green" => bag.green = num,
            "blue" => bag.blue = num,
            _ => eprintln!("Invalid line!"),
        }
    }
    Ok((i, bag))
}

fn part1(input: &str) -> u32 {
    input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(line_parser)
        .filter(|result| {
            result
                .as_ref()
                .map(|(_, game)| {
                    game.bags.iter().all(|bag| {
                        bag.red <= MAX_BAG.red
                            && bag.green <= MAX_BAG.green
                            && bag.blue <= MAX_BAG.blue
                    })
                })
                .unwrap()
        })
        .fold(0, |sum, result| sum + result.unwrap().1.id)
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
        let result = part1(include_str!("input1_test.txt"));
        assert_eq!(result, 8);
    }
}
