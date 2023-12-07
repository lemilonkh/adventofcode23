extern crate nom;

use std::{cmp::Ordering, collections::HashMap};

use nom::{
    character::complete::{anychar, space1, u32},
    multi::count,
    sequence::separated_pair,
    IResult,
};

static CARDS: &'static [char] = &[
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

fn line_parser(i: &str) -> IResult<&str, (Vec<char>, u32)> {
    dbg!(i);
    separated_pair(hand_parser, space1, u32)(i)
}

fn hand_parser(i: &str) -> IResult<&str, Vec<char>> {
    count(anychar, 5)(i)
}

fn get_hand_type(cards: &Vec<char>) -> u32 {
    let mut char_amounts: HashMap<char, u32> = HashMap::new();
    for card in cards {
        char_amounts
            .entry(*card)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    let amounts: Vec<u32> = char_amounts.values().cloned().collect();

    if amounts.len() == 1 {
        return 6; // five of a kind
    } else if amounts.len() == 2 {
        if amounts[0] == 4 || amounts[0] == 1 {
            return 5; // four of a kind
        } else {
            return 4; // full house
        }
    } else if amounts.iter().any(|a| *a == 3) {
        return 3; // three of a kind
    }

    let pairs = amounts.iter().filter(|a| **a == 2).count();
    if pairs == 2 {
        return 2;
    } else if pairs == 1 {
        return 1;
    }

    return 0;
}

fn part1(input: &str) -> u32 {
    let mut hands: Vec<(Vec<char>, u32, u32)> = input
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(line_parser)
        .map(|l| l.expect("valid input line").1)
        .map(|hand| {
            let hand_type = get_hand_type(&hand.0);
            (hand.0, hand.1, hand_type)
        })
        .collect();

    hands.sort_by(|a, b| {
        if a.2 != b.2 {
            return a.2.cmp(&b.2);
        }

        for i in 0..b.0.len() {
            let value_a = CARDS.iter().position(|c| *c == a.0[i]).expect("valid card");
            let value_b = CARDS.iter().position(|c| *c == b.0[i]).expect("valid card");
            if value_a != value_b {
                return value_a.cmp(&value_b);
            }
        }

        eprintln!(
            "Hands are equal?! {} {}",
            a.0.iter().collect::<String>(),
            b.0.iter().collect::<String>()
        );
        return Ordering::Equal;
    });

    hands
        .iter()
        .enumerate()
        .inspect(|(i, (cards, bet, hand_type))| {
            println!(
                "Hand {}: Cards {} Bet {} Hand Type {}",
                i,
                cards.iter().collect::<String>(),
                bet,
                hand_type
            )
        })
        .map(|(i, (_cards, bet, _hand_type))| (i as u32 + 1) * bet)
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
        let result = line_parser("32T3K 765").unwrap().1;
        assert_eq!(result, (vec!('3', '2', 'T', '3', 'K'), 765));
    }

    #[test]
    fn it_works() {
        let result = part1(include_str!("input1_test.txt"));
        assert_eq!(result, 6440);
    }
}
