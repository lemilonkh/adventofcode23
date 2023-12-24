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
}
impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn is_in_bounds(position: (i32, i32), width: usize, height: usize) -> bool {
    position.0 >= 0 && position.1 >= 0 && position.0 < width as i32 && position.1 < height as i32
}

fn get_successors(grid: &Array2D<u8>, pos: &Pos, path: &Vec<Pos>) -> Vec<Pos> {
    Direction::iter()
        .filter_map(|direction| {
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

            if path.iter().any(|p| p.x == new_pos.0 && p.y == new_pos.1) {
                return None;
            }

            Some(Pos {
                x: new_pos.0,
                y: new_pos.1,
            })
        })
        .collect()
}

fn part1(input: &str) -> Result<u32, Error> {
    let lines: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

    let grid = Array2D::from_rows(&lines)?;
    let width = grid.num_columns() as i32;
    let height = grid.num_rows() as i32;

    let initial_position = Pos { x: 1, y: 0 };

    let mut queue = VecDeque::new();
    queue.push_back((initial_position, 0, vec![initial_position]));

    let mut path_lengths = vec![];

    while !queue.is_empty() {
        let (mut pos, mut current_length, mut path) = queue.pop_front().unwrap();
        loop {
            let successors = get_successors(&grid, &pos, &path);
            if successors.len() == 0 {
                break;
            }
            if successors.len() > 1 {
                for &successor in successors[1..].iter() {
                    let mut split_path = path.clone();
                    split_path.push(successor);
                    queue.push_back((successor, current_length + 1, split_path));
                }
            }
            pos = successors[0];
            current_length += 1;
            path.push(pos);
        }

        // println!("Path ends at {:?} with len {}", pos, current_length);
        if pos.x == width as i32 - 2 && pos.y == height as i32 - 1 {
            path_lengths.push((current_length, path));
        }
    }

    let (longest_length, longest_path) = path_lengths
        .iter()
        .max_by_key(|(len, _)| len)
        .expect("found max");

    // /* enable for debug output of grid and chosen path
    for y in 0..grid.num_rows() as i32 {
        for x in 0..grid.num_columns() as i32 {
            let pos = longest_path.iter().find(|p| p.x == x && p.y == y);
            if pos.is_some() {
                print!("O");
            } else {
                print!("{}", *grid.get(y as usize, x as usize).unwrap() as char);
            }
        }
        println!();
    }
    // */

    Ok(*longest_length)
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
        assert_eq!(result, 154);
    }
}
