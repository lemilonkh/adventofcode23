use itertools::Itertools;
use std::str::FromStr;

use nom::{
    character::complete::{char, digit1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Brick {
    start: Vec3,
    end: Vec3,
    supports: Vec<usize>,
    supported_by: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Vec3 {
    x: u32,
    y: u32,
    z: u32,
}

fn line_parser(i: &str) -> IResult<&str, Brick> {
    let (i, (start, end)) = separated_pair(vec_parser, char('~'), vec_parser)(i)?;
    Ok((
        i,
        Brick {
            start,
            end,
            supports: vec![],
            supported_by: vec![],
        },
    ))
}

fn vec_parser(i: &str) -> IResult<&str, Vec3> {
    let (i, vec) = separated_list1(char(','), int_parser)(i)?;
    assert_eq!(vec.len(), 3);
    Ok((
        i,
        Vec3 {
            x: vec[0],
            y: vec[1],
            z: vec[2],
        },
    ))
}

fn int_parser(i: &str) -> IResult<&str, u32> {
    map_res(digit1, FromStr::from_str)(i)
}

fn check_range(val: u32, start: u32, end: u32) -> bool {
    val >= start && val <= end
}

fn check_collision(a: &Brick, b: &Brick) -> bool {
    (check_range(a.start.z, b.start.z, b.end.z)
        || check_range(a.end.z, b.start.z, b.end.z)
        || check_range(b.start.z, a.start.z, a.end.z)
        || check_range(b.end.z, a.start.z, a.end.z))
        && (check_range(a.start.x, b.start.x, b.end.x)
            || check_range(a.end.x, b.start.x, b.end.x)
            || check_range(b.start.x, a.start.x, a.end.x)
            || check_range(b.end.x, a.start.x, a.end.x))
        && (check_range(a.start.y, b.start.y, b.end.y)
            || check_range(a.end.y, b.start.y, b.end.y)
            || check_range(b.start.y, a.start.y, a.end.y)
            || check_range(b.end.y, a.start.y, a.end.y))
}

fn get_settled_bricks(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| line_parser(line).expect("valid input").1)
        .sorted_by_key(|brick| brick.start.z)
        .fold(
            (Vec::<Brick>::new(), 1),
            |(mut result, max_z), mut brick| {
                let height = brick.end.z - brick.start.z;
                brick.start.z = max_z + 1;
                brick.end.z = brick.start.z + height;

                loop {
                    let mut stopped = false;
                    brick.start.z -= 1;
                    brick.end.z -= 1;

                    for i in (0..result.len()).rev() {
                        if result[i].end.z < brick.start.z {
                            continue;
                        }

                        let collision = check_collision(&brick, &result[i]);
                        if collision {
                            stopped = true;
                            let len = result.len();
                            result[i].supports.push(len);
                            brick.supported_by.push(i);
                        }
                    }

                    if stopped {
                        brick.start.z += 1;
                        brick.end.z += 1;
                        break;
                    }

                    if brick.start.z == 1 {
                        break;
                    }
                }

                let new_max_z = max_z.max(brick.end.z);
                result.push(brick);
                (result, new_max_z)
            },
        )
        .0
}

fn part1(input: &str) -> u64 {
    let bricks: Vec<Brick> = get_settled_bricks(input);

    bricks
        .iter()
        .filter(|brick| {
            brick
                .supports
                .iter()
                .all(|&i| bricks[i].supported_by.len() > 1)
        })
        .count() as u64
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
    fn it_works() {
        let result = part1(include_str!("input1_test.txt"));
        assert_eq!(result, 5);
    }

    #[test]
    fn parse_line() {
        let (i, brick) = line_parser("1,0,1~1,2,1").unwrap();
        assert_eq!(i, "");
        assert_eq!(
            brick,
            Brick {
                start: Vec3 { x: 1, y: 0, z: 1 },
                end: Vec3 { x: 1, y: 2, z: 1 },
                supports: vec![],
                supported_by: vec![],
            }
        );
    }
}
