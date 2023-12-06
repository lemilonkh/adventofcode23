extern crate nom;
use rayon::prelude::*;
use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Default, Debug, PartialEq, Eq)]
struct Map<'a> {
    source: &'a str,
    destination: &'a str,
    ranges: Vec<MapRange>,
}

#[derive(Default, Debug, PartialEq, Eq)]
struct MapRange {
    source_start: u32,
    destination_start: u32,
    range_length: u32,
}

fn map_parser(i: &str) -> IResult<&str, Map> {
    let (i, (source, destination)) =
        terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:\n"))(i)?;
    let (i, range_lists) = separated_list1(char('\n'), number_list_parser)(i)?;
    let ranges = range_lists
        .iter()
        .map(|list| {
            assert_eq!(list.len(), 3);
            MapRange {
                destination_start: list[0],
                source_start: list[1],
                range_length: list[2],
            }
        })
        .collect();

    Ok((
        i,
        Map {
            source,
            destination,
            ranges,
        },
    ))
}

fn seeds_parser(i: &str) -> IResult<&str, Vec<u32>> {
    preceded(tag("seeds: "), number_list_parser)(i)
}

fn number_list_parser(i: &str) -> IResult<&str, Vec<u32>> {
    let (i, list) = separated_list1(space1, digit1)(i)?;
    let numbers = list
        .iter()
        .map(|str| str.parse().expect("valid number"))
        .collect();
    Ok((i, numbers))
}

fn part1(input: &str) -> u32 {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let (_, seed_input) = seeds_parser(parts[0]).expect("valid list of seeds");
    assert_eq!(seed_input.len() % 2, 0, "even number of seed range numbers");
    let seed_ranges: Vec<Range<u32>> = seed_input
        .chunks_exact(2)
        .inspect(|w| println!("a: {}, b: {}", w[0], w[1]))
        .map(|w| w[0]..w[0] + w[1])
        .collect();

    println!("Seed range count: {}", seed_ranges.len());

    let maps: Vec<Map> = parts[1..]
        .iter()
        .map(|part| map_parser(*part).expect("valid alamanac map").1)
        .collect();

    *seed_ranges
        .par_iter()
        .map(|range| {
            range
                .to_owned()
                .map(|number| {
                    let mut result = number;
                    for map in maps.iter() {
                        for range in map.ranges.iter() {
                            if result >= range.source_start
                                && result < range.source_start + range.range_length
                            {
                                result = result - range.source_start + range.destination_start;
                                break;
                            }
                        }
                    }
                    result
                })
                .min()
                .expect("should have a minimum height in thread")
        })
        .collect::<Vec<u32>>() // this makes it 30 seconds faster - why?
        .iter()
        .min()
        .expect("should have a minimum height")
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
    fn parse_map() {
        let result = map_parser(
            "seed-to-soil map:
50 98 2
52 50 48
",
        )
        .unwrap()
        .1;
        assert_eq!(
            result,
            Map {
                source: "seed",
                destination: "soil",
                ranges: vec!(
                    MapRange {
                        destination_start: 50,
                        source_start: 98,
                        range_length: 2
                    },
                    MapRange {
                        destination_start: 52,
                        source_start: 50,
                        range_length: 48
                    },
                )
            }
        );
    }

    #[test]
    fn it_works() {
        let result = part1(include_str!("input1_test.txt"));
        assert_eq!(result, 46);
    }
}
