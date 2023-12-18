use std::{collections::VecDeque, str::FromStr};

use array2d::{Array2D, Error};
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, one_of, space1},
    combinator::map_res,
    sequence::{delimited, tuple},
    IResult,
};

use Direction::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            NORTH => (0, -1),
            EAST => (1, 0),
            SOUTH => (0, 1),
            WEST => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
    direction: Direction,
    count: u32,
}
impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn line_parser(i: &str) -> IResult<&str, (char, u32, &str)> {
    let (i, (direction, _, steps, _, color)) = tuple((
        one_of("RDLU"),
        space1,
        int_parser,
        space1,
        delimited(tag("(#"), alphanumeric1, tag(")")),
    ))(i)?;
    Ok((i, (direction, steps, color)))
}

fn int_parser(i: &str) -> IResult<&str, u32> {
    map_res(digit1, FromStr::from_str)(i)
}

fn flood_fill(
    start_x: usize,
    start_y: usize,
    target: char,
    replacement: char,
    grid: &mut Array2D<char>,
) -> Result<(), Error> {
    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y));

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().expect("found pos");
        if grid.get(y, x) != Some(&target) {
            continue;
        }

        grid.set(y, x, replacement)?;

        if y > 0 {
            queue.push_back((x, y - 1));
        }
        if x > 0 {
            queue.push_back((x - 1, y));
        }
        if y < grid.num_rows() - 1 {
            queue.push_back((x, y + 1));
        }
        if x < grid.num_columns() - 1 {
            queue.push_back((x + 1, y));
        }
    }

    Ok(())
}

fn part1(input: &str) -> Result<u32, Error> {
    let lines: Vec<(char, u32, &str)> = input
        .lines()
        .map(|line| line_parser(line).expect("valid input").1)
        .collect();

    let mut grid = Array2D::filled_with('.', 800, 800);
    let mut position = (400, 400);

    grid.set(position.1 as usize, position.0 as usize, '#')?;

    for (dir, steps, _color) in lines {
        let direction = match dir {
            'R' => EAST,
            'L' => WEST,
            'U' => NORTH,
            'D' => SOUTH,
            _ => {
                eprintln!("Invalid input! {}", dir);
                NORTH
            }
        };

        let delta = direction.delta();
        for _step in 0..steps {
            position = (position.0 + delta.0, position.1 + delta.1);
            // println!("Step {} at {:?}", _step, position);
            grid.set(position.1 as usize, position.0 as usize, '#')?;
        }
    }

    flood_fill(position.1 as usize + 1, position.0 as usize + 1, '.', '#', &mut grid)?;

    // for y in 0..height as i32 {
    //     for x in 0..width as i32 {
    //         print!("{}", grid.get(y as usize, x as usize).unwrap());
    //     }
    //     println!();
    // }

    let volume = grid
        .rows_iter()
        .map(|row| row.filter(|c| **c == '#').count() as u32)
        .sum();

    Ok(volume)
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input).expect("found result");
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(include_str!("input1_test.txt")).expect("run without errors");
        assert_eq!(result, 62);
    }
}
