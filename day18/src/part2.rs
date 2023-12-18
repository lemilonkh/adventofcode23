use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

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
    start_x: i32,
    start_y: i32,
    board: &mut HashMap<(i32, i32), bool>,
    map_size: i32,
) {
    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y));

    while !queue.is_empty() {
        let pos = queue.pop_front().expect("found pos");
        if board.contains_key(&pos) {
            continue;
        }

        board.insert(pos, true);
        let (x, y) = pos;

        if y > -map_size {
            queue.push_back((x, y - 1));
        }
        if x > -map_size {
            queue.push_back((x - 1, y));
        }
        if y < map_size {
            queue.push_back((x, y + 1));
        }
        if x < map_size {
            queue.push_back((x + 1, y));
        }
    }
}

fn part1(input: &str) -> usize {
    let lines: Vec<(char, u32, &str)> = input
        .lines()
        .map(|line| line_parser(line).expect("valid input").1)
        .collect();

    let mut board: HashMap<(i32, i32), bool> = HashMap::new();
    let mut position = (0, 0);
    let map_size = 1000000;

    board.insert(position, true);

    for (_dir, _steps, color) in lines {
        let mut steps_hex = color.to_owned();
        let direction_char = steps_hex.pop().expect("found direction char");
        let direction = match direction_char {
            '0' => EAST,
            '2' => WEST,
            '3' => NORTH,
            '1' => SOUTH,
            _ => {
                eprintln!("Invalid input! {}", direction_char);
                NORTH
            }
        };
        let steps = i64::from_str_radix(&steps_hex, 16).expect("valid hex number");

        let delta = direction.delta();
        for _step in 0..steps {
            position = (position.0 + delta.0, position.1 + delta.1);
            println!("Step {} at {:?}", _step, position);
            board.entry(position).or_insert(true);
        }
    }

    flood_fill(
        position.1 + 1,
        position.0 + 1,
        &mut board,
        map_size,
    );

    // for y in 0..height as i32 {
    //     for x in 0..width as i32 {
    //         print!("{}", grid.get(y as usize, x as usize).unwrap());
    //     }
    //     println!();
    // }

    board.len()
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
        assert_eq!(result, 62);
    }
}
