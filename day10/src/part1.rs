use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq, Clone)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}
use Direction::*;

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            NORTH => SOUTH,
            EAST => WEST,
            SOUTH => NORTH,
            WEST => EAST,
        }
    }
}

fn get_direction_delta(direction: &Direction) -> (i32, i32) {
    match direction {
        NORTH => (0, -1),
        EAST => (1, 0),
        SOUTH => (0, 1),
        WEST => (-1, 0),
    }
}

/*fn get_delta_direction(delta: (i32, i32)) -> Option<Direction> {
    match delta {
        (0, -1) => Some(NORTH),
        (1, 0) => Some(EAST),
        (0, 1) => Some(SOUTH),
        (-1, 0) => Some(WEST),
        _ => None,
    }
}*/

fn get_pipe_directions(char: char) -> Option<(Direction, Direction)> {
    match char {
        '|' => Some((NORTH, SOUTH)),
        '-' => Some((EAST, WEST)),
        'L' => Some((NORTH, EAST)),
        'J' => Some((NORTH, WEST)),
        '7' => Some((SOUTH, WEST)),
        'F' => Some((SOUTH, EAST)),
        _ => None,
    }
}

fn div_round_up(dividend: u32, divisor: u32) -> u32 {
    (dividend + divisor - 1) / divisor
}

fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let mut start_coords: Option<(usize, usize)> = None;
    'outer: for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                start_coords = Some((x, y));
                break 'outer;
            }
        }
    }

    let mut position = start_coords.expect("found starting position");
    let mut prev_direction: Option<Direction> = None;
    let mut next_direction: Option<Direction> = None;
    let mut loop_length: u32 = 0;
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    'outer: loop {
        let mut found_next_pipe = false;
        println!("At {:?} with length {}", position, loop_length);
        for direction in Direction::iter() {
            if next_direction.is_some() && next_direction.as_ref() != Some(&direction) {
                println!(
                    "Direction {:?} is not next direction {:?}, skipping",
                    direction, next_direction
                );
                continue;
            }
            if prev_direction == Some(direction.opposite()) {
                println!(
                    "Prev direction {:?} is opposite of current direction {:?}, skipping",
                    prev_direction, direction
                );
                continue;
            }

            // get neighboring character and do bounds check
            let (x, y) = get_direction_delta(&direction);
            let target = (position.0 as i32 + x, position.1 as i32 + y);
            if target.0 < 0 || target.0 >= width || target.1 < 0 || target.1 >= height {
                continue;
            }
            let target_char = grid[target.1 as usize][target.0 as usize];
            println!("Found char {} in direction {:?}", target_char, direction);
            if target_char == 'S' {
                break 'outer;
            }

            // check if adjacent char is a pipe and it allows connecting with the previous pipe
            let directions = get_pipe_directions(target_char);
            if directions.is_none() {
                println!("No directions for char {}", target_char);
                continue;
            }
            let directions = dbg!(directions.unwrap());

            if direction.opposite() == directions.0 || direction.opposite() == directions.1 {
                found_next_pipe = true;
                println!(
                    "Moving to char {} in direction {:?}",
                    target_char, direction
                );
                position = (target.0 as usize, target.1 as usize);

                next_direction = if direction.opposite() == directions.0 {
                    Some(directions.1)
                } else {
                    Some(directions.0)
                };
                dbg!(&next_direction);

                prev_direction = Some(direction);
                break;
            }
        }
        assert_eq!(found_next_pipe, true);
        loop_length += 1;
    }

    div_round_up(loop_length, 2)
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
        assert_eq!(result, 4);
        let result = part1(include_str!("input2_test.txt"));
        assert_eq!(result, 8);
    }
}
