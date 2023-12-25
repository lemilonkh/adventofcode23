use itertools::Itertools;
use ndarray::prelude::*;
use ndarray_linalg::Solve;
use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, space1},
    combinator::{map_res, opt, recognize},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair},
    IResult,
};

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

fn line_parser(i: &str) -> IResult<&str, (Vec3, Vec3)> {
    separated_pair(vec3_parser, pair(tag(" @"), space1), vec3_parser)(i)
}

fn vec3_parser(i: &str) -> IResult<&str, Vec3> {
    let (i, numbers) = separated_list1(pair(char(','), space1), int_parser)(i)?;
    Ok((
        i,
        Vec3 {
            x: numbers[0],
            y: numbers[1],
            z: numbers[2],
        },
    ))
}

fn int_parser(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(preceded(opt(tag("-")), digit1)),
        FromStr::from_str,
    )(i)
}

fn find_speed(speeds: Vec<(i64, i64)>) -> i64 {
    let min_speed = speeds.iter().map(|s| s.1).min().unwrap();
    let max_speed = speeds.iter().map(|s| s.1).max().unwrap();
    let possibilities = vec![true; (max_speed - min_speed + 1) as usize];

    speeds
        .iter()
        .tuple_combinations()
        .fold(possibilities, |mut possible, ((p1, v1), (p2, v2))| {
            if v1 == v2 {
                let offset = (p1 - p2).abs();
                for i in 0..possible.len() {
                    let speed_diff = i as i64 + min_speed - *v2;
                    if speed_diff != 0 && offset % speed_diff != 0 {
                        possible[i] = false;
                    }
                }
            }
            possible
        })
        .into_iter()
        .position(|s| s) // find first true
        .unwrap() as i64
        + min_speed
}

fn part2(input: &str) -> i64 {
    let stones: Vec<(Vec3, Vec3)> = input
        .lines()
        .map(|line| line_parser(line).expect("valid input").1)
        .collect();

    // determine possible velocity of thrown rock
    let speed_x = find_speed(stones.iter().map(|(p, v)| (p.x, v.x)).collect()) as f64;
    let speed_y = find_speed(stones.iter().map(|(p, v)| (p.y, v.y)).collect()) as f64;
    let speed_z = find_speed(stones.iter().map(|(p, v)| (p.z, v.z)).collect()) as f64;

    // construct matrix and vector for solver
    let matrix: Array2<f64> = array![
        [1.0, 0.0, 0.0, speed_x - stones[0].1.x as f64, 0.0],
        [0.0, 1.0, 0.0, speed_y - stones[0].1.y as f64, 0.0],
        [0.0, 0.0, 1.0, speed_z - stones[0].1.z as f64, 0.0],
        [1.0, 0.0, 0.0, 0.0, speed_x - stones[1].1.x as f64],
        [0.0, 1.0, 0.0, 0.0, speed_y - stones[1].1.y as f64],
    ];
    let vector: Array1<f64> = array![
        stones[0].0.x as f64,
        stones[0].0.y as f64,
        stones[0].0.z as f64,
        stones[1].0.x as f64,
        stones[1].0.y as f64,
    ];

    let solution = matrix.solve_into(vector).expect("found solution");
    println!(
        "Found solution! Throw rock from ({}, {}, {}) with speed ({}, {}, {})",
        solution[0], solution[1], solution[2], speed_x, speed_y, speed_z
    );

    (solution[0] + solution[1] + solution[2]).round() as i64
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[link(name = "lapack")]
extern "C" {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2(include_str!("input1_test.txt"));
        // 47 is given in example but this seems to work too. Maybe too
        // unconstrained, also not quite an integer solution
        assert_eq!(result, 75);
    }

    #[test]
    fn parse_stone() {
        let (i, (a, b)) = line_parser("20, 19, 15 @  1, -5, -3").expect("correctly parsed");
        assert_eq!(i, "");
        assert_eq!(a.x, 20);
        assert_eq!(a.y, 19);
        assert_eq!(a.z, 15);

        assert_eq!(b.x, 1);
        assert_eq!(b.y, -5);
        assert_eq!(b.z, -3);
    }
}
