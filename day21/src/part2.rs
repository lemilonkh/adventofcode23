use std::collections::VecDeque;

use array2d::{Array2D, Error};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use Direction::*;

#[derive(Debug, PartialEq, EnumIter, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
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
    distance: u64,
}
impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn is_in_bounds(position: (i32, i32), width: usize, height: usize) -> bool {
    position.0 >= 0 && position.1 >= 0 && position.0 < width as i32 && position.1 < height as i32
}

fn get_successors<'a>(grid: &'a Array2D<u8>, pos: &'a Pos) -> impl Iterator<Item = Pos> + 'a {
    Direction::iter().filter_map(|direction| {
        let delta = direction.delta();
        let new_pos = (pos.x + delta.0, pos.y + delta.1);
        let in_bounds = is_in_bounds(new_pos, grid.num_columns(), grid.num_rows());
        if !in_bounds {
            return None;
        }

        let grid_value = *grid
            .get(new_pos.1 as usize, new_pos.0 as usize)
            .expect("found grid tile");
        if grid_value == b'#' {
            return None;
        }

        Some(Pos {
            x: new_pos.0,
            y: new_pos.1,
            distance: pos.distance + 1,
        })
    })
}

fn part1(input: &str, steps: u64) -> Result<u64, Error> {
    let lines: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

    let grid = Array2D::from_rows(&lines)?;

    let width = grid.num_columns();
    let height = grid.num_rows();

    let mut position = Pos {
        x: 0,
        y: 0,
        distance: 0,
    };

    'outer: for y in 0..height as i32 {
        for x in 0..width as i32 {
            let tile = *grid.get(x as usize, y as usize).unwrap();
            if tile == b'S' {
                position.x = x;
                position.y = y;
                break 'outer;
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(position);

    for step in 0..steps {
        println!("Step {}", step);

        let mut next_steps = VecDeque::new();

        // only handle positions added before this step
        while !queue.is_empty() {
            let pos = queue.pop_front().unwrap();
            for next in get_successors(&grid, &pos) {
                if !next_steps.contains(&next) {
                    next_steps.push_back(next);
                }
            }
        }

        queue = next_steps;
    }

    let mut final_steps: Vec<Pos> = Vec::from(queue);
    println!("Found {} possible paths", final_steps.len());
    final_steps.sort();
    final_steps.dedup();
    println!("Found {} unique final positions", final_steps.len());

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let pos = final_steps.iter().find(|p| p.x == x && p.y == y);
            if pos.is_some() {
                print!("O");
            } else {
                print!("{}", *grid.get(y as usize, x as usize).unwrap() as char);
            }
        }
        println!();
    }

    Ok(final_steps.len() as u64)
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input, 64).expect("found result");
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(6, 16)]
    #[case(10, 50)]
    #[case(50, 1594)]
    #[case(100, 6539)]
    #[case(500, 167004)]
    #[case(1000, 668697)]
    #[case(5000, 16733044)]
    fn it_works(#[case] steps: u64, #[case] expected: u64) {
        let result = part1(include_str!("input1_test.txt"), steps).expect("run without errors");
        assert_eq!(result, expected);
    }
}
