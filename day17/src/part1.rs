use array2d::{Array2D, Error};
use cached::proc_macro::cached;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use Direction::*;

#[derive(Debug, PartialEq, EnumIter, Eq, Clone)]
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

    fn opposite(&self) -> Direction {
        match self {
            NORTH => SOUTH,
            EAST => WEST,
            SOUTH => NORTH,
            WEST => EAST,
        }
    }
}

fn is_in_bounds(position: (i32, i32), width: usize, height: usize) -> bool {
    position.0 >= 0 && position.1 >= 0 && position.0 < width as i32 && position.1 < height as i32
}

#[cached(key = "String", convert = r#"{format!("{:?}", position)}"#)]
fn find_path(
    position: (usize, usize),
    target: (usize, usize),
    grid: &Array2D<u32>,
    status_grid: &mut Array2D<u32>,
    prev_direction: Direction,
    prev_count: u32,
) -> u32 {
    println!(
        "At {:?} prev dir {:?} with count {}",
        position, prev_direction, prev_count
    );
    if position == target {
        return *grid.get(position.0, position.1).expect("found value");
    }

    let status = *status_grid
        .get(position.0, position.1)
        .expect("found status");
    if status > 0 {
        return status;
    }

    let heat_loss = Direction::iter()
        .filter_map(|direction| {
            if direction == prev_direction.opposite() {
                return None;
            }
            let direction_count = if direction == prev_direction {
                prev_count + 1
            } else {
                0
            };

            // go max 3 times into the same direction
            if direction_count > 2 {
                return None;
            }
            let delta = direction.delta();
            let new_pos = (position.0 as i32 + delta.0, position.1 as i32 + delta.1);
            if !is_in_bounds(new_pos, grid.num_columns(), grid.num_rows()) {
                return None;
            }

            let heat_loss = find_path(
                (new_pos.0 as usize, new_pos.1 as usize),
                target,
                grid,
                status_grid,
                direction,
                direction_count,
            );
            Some(heat_loss)
        })
        .min()
        .expect("found minimum heat loss");

    let current_tile = *grid.get(position.0, position.1).expect("found value");
    status_grid
        .set(position.0, position.1, heat_loss + current_tile)
        .expect("can write status");
    return heat_loss + current_tile;
}

fn part1(input: &str) -> Result<u32, Error> {
    let lines: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("valid digit"))
                .collect::<Vec<u32>>()
        })
        .collect();

    let grid = Array2D::from_rows(&lines)?;
    let mut status_grid = Array2D::filled_with(0, grid.num_rows(), grid.num_columns());

    let width = grid.num_columns();
    let height = grid.num_rows();

    let position: (usize, usize) = (0, 0);
    let target: (usize, usize) = (width - 1, height - 1);

    let heat_loss = find_path(position, target, &grid, &mut status_grid, WEST, 0);
    Ok(heat_loss)
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
        let result = part1(include_str!("input2_test.txt")).expect("run without errors");
        assert_eq!(result, 102);
    }
}
