use std::{collections::HashMap, str::FromStr};

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
    supports: u32,
}

impl Brick {
    fn contains(&self, pos: Vec3) -> bool {
        (self.start.x..=self.end.x).contains(&pos.x)
            && (self.start.y..=self.end.y).contains(&pos.y)
            && (self.start.z..=self.end.z).contains(&pos.z)
    }

    fn direction(&self) -> (u8, u32) {
        let delta = (
            self.end.x - self.start.x,
            self.end.y - self.start.y,
            self.end.z - self.start.z,
        );
        match delta {
            (x, 0, 0) => (0, x),
            (0, y, 0) => (1, y),
            (0, 0, _) => (0, 0), // we only care about horizontal size
            _ => panic!("Non-line brick {:?}~{:?}", self.start, self.end),
        }
    }

    fn iter(&self) -> BrickIter {
        let dir = self.direction();
        BrickIter {
            brick: self,
            direction: dir.0,
            length: dir.1,
            index: 0,
        }
    }
}

struct BrickIter<'a> {
    brick: &'a Brick,
    direction: u8,
    index: u32,
    length: u32,
}
impl Iterator for BrickIter<'_> {
    type Item = Vec3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.length {
            return None;
        }

        let mut pos = self.brick.start.clone();
        match self.direction {
            0 => pos.x += self.index,
            1 => pos.y += self.index,
            2 => pos.z += self.index,
            _ => panic!("Invalid direction {}", self.direction),
        }
        self.index += 1;
        Some(pos)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Vec3 {
    x: u32,
    y: u32,
    z: u32,
}

fn line_parser(i: &str) -> IResult<&str, (u32, Brick)> {
    let (i, (start, end)) = separated_pair(vec_parser, char('~'), vec_parser)(i)?;
    Ok((
        i,
        (
            end.z,
            Brick {
                start,
                end,
                supports: 0,
            },
        ),
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

fn part1(input: &str) -> u64 {
    // top Z coordinate to all bricks that end in that layer
    let mut bricks: HashMap<u32, Vec<Brick>> = HashMap::new();
    let mut max_z = 0;

    for line in input.lines() {
        let (top_z, brick) = line_parser(line).expect("valid input").1;
        bricks.entry(top_z).or_insert(vec![]).push(brick);
        max_z = max_z.max(top_z);
    }

    for z in 2..=max_z {
        if !bricks.contains_key(&z) {
            continue;
        }
        let current_bricks = bricks.get(&z).unwrap().clone();
        for mut brick in current_bricks {
            for z_below in (1..=brick.start.z - 1).rev() {
                if !bricks.contains_key(&z_below) {
                    continue;
                }

                let bricks_below = bricks.get_mut(&z_below).unwrap();
                let mut stopped = false;
                let mut supporting_bricks = vec!();

                for brick_below in bricks_below {
                    for mut pos in brick.iter() {
                        pos.z = z_below;
                        if brick_below.contains(pos) {
                            supporting_bricks.push(brick_below);
                            stopped = true;
                            break;
                        }
                    }
                }

                // if resting on exactly one brick, mark it as non-disintegratable
                if supporting_bricks.len() == 1 {
                    supporting_bricks[0].supports += 1;
                }

                if stopped || z_below == 1 {
                    // move brick down to where it landed
                    let bricks_mut = bricks.get_mut(&z).unwrap();
                    let index = bricks_mut
                        .iter()
                        .position(|b| b == &brick)
                        .expect("found brick");
                    bricks_mut.swap_remove(index);

                    let brick_height = brick.end.z - brick.start.z;
                    brick.start.z = z_below + 1;
                    brick.end.z = brick.start.z + brick_height;

                    bricks.entry(&z_below + 1).or_insert(vec![]).push(brick);
                    break;
                }
            }
        }
    }

    bricks
        .values()
        .flatten()
        .filter(|brick| brick.supports == 0)
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
            (
                1,
                Brick {
                    start: Vec3 { x: 1, y: 0, z: 1 },
                    end: Vec3 { x: 1, y: 2, z: 1 },
                    supports: 0,
                }
            )
        );
    }
}
