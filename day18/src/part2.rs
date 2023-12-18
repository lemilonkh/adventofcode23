use polygonical::{point::Point, polygon::Polygon};
use std::str::FromStr;

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

fn polygon_perimeter(points: &[Point]) -> f64 {
    let mut peri = 0.0;
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        peri += (points[i].x - points[j].x).abs();
        peri += (points[i].y - points[j].y).abs();
    }
    peri
}

fn part1(input: &str) -> i64 {
    let lines: Vec<(char, u32, &str)> = input
        .lines()
        .map(|line| line_parser(line).expect("valid input").1)
        .collect();

    let mut points: Vec<Point> = vec![];
    let mut position = (0, 0);

    points.push(Point::new(position.0, position.1));

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
        let steps = i32::from_str_radix(&steps_hex, 16).expect("valid hex number");

        let delta = direction.delta();
        position = (position.0 + delta.0 * steps, position.1 + delta.1 * steps);
        points.push(Point::new(position.0, position.1));
        println!("At position {:?}", position);
    }

    let perimeter = polygon_perimeter(&points);
    let half_perimeter = (perimeter / 2.0).floor() + 1.0;
    println!("Perimeter: {}, half: {}", perimeter, half_perimeter);

    let area = Polygon::new(points).area();
    println!("Area: {}", area);

    area.round().abs() as i64 + half_perimeter.floor().abs() as i64
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
        assert_eq!(result, 952408144115);
    }
}
