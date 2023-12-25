use itertools::Itertools;
use std::{
    ops::{Add, Mul},
    str::FromStr,
};

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
    x: f64,
    y: f64,
    z: f64,
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, f: f64) -> Vec3 {
        Vec3 {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
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

fn int_parser(i: &str) -> IResult<&str, f64> {
    map_res(
        recognize(preceded(opt(tag("-")), digit1)),
        FromStr::from_str,
    )(i)
}

fn part1(input: &str, axis_min: f64, axis_max: f64) -> usize {
    let stones: Vec<(Vec3, Vec3)> = input
        .lines()
        .map(|line| line_parser(line).expect("valid input").1)
        .collect();

    stones
        .iter()
        .tuple_combinations()
        .filter(|(&(pa1, va), &(pb1, vb))| {
            // println!("Hailstone A: {:?} @ {:?}", pa1, va);
            // println!("Hailstone B: {:?} @ {:?}", pb1, vb);

            let pa2 = pa1 + va;
            let pb2 = pb1 + vb;

            // calculate line-line intersection with 2 points on each line
            let ta = ((pa1.x - pb1.x) * (pb1.y - pb2.y) - (pa1.y - pb1.y) * (pb1.x - pb2.x))
                / ((pa1.x - pa2.x) * (pb1.y - pb2.y) - (pa1.y - pa2.y) * (pb1.x - pb2.x));
            let tb = ((pa1.x - pb1.x) * (pa1.y - pa2.y) - (pa1.y - pb1.y) * (pa1.x - pa2.x))
                / ((pa1.x - pa2.x) * (pb1.y - pb2.y) - (pa1.y - pa2.y) * (pb1.x - pb2.x));

            if ta < 0.0 {
                // println!("Hailstones' paths crossed in the past for hailstone A.");
                return false;
            }
            if tb < 0.0 {
                // println!("Hailstones' paths crossed in the past for hailstone B.");
                return false;
            }

            let cross = pa1 + va * ta;
            // println!("Crossing at {:?}", cross);
            cross.x >= axis_min && cross.x <= axis_max && cross.y >= axis_min && cross.y <= axis_max
        })
        .count()
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input, 200_000_000_000_000.0, 400_000_000_000_000.0);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(include_str!("input1_test.txt"), 7.0, 27.0);
        assert_eq!(result, 2);
    }

    #[test]
    fn parse_stone() {
        let (i, (a, b)) = line_parser("20, 19, 15 @  1, -5, -3").expect("correctly parsed");
        assert_eq!(i, "");
        assert_eq!(a.x, 20.0);
        assert_eq!(a.y, 19.0);
        assert_eq!(a.z, 15.0);

        assert_eq!(b.x, 1.0);
        assert_eq!(b.y, -5.0);
        assert_eq!(b.z, -3.0);
    }
}
