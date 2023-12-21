use array2d::{Array2D, Error};
use pathfinding::directed::astar::astar_bag;
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
    distance: u32,
}
impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn is_in_bounds(position: (i32, i32), width: usize, height: usize) -> bool {
    position.0 >= 0 && position.1 >= 0 && position.0 < width as i32 && position.1 < height as i32
}

fn get_successors(grid: &Array2D<u8>, pos: Pos, max_distance: u32) -> Vec<(Pos, u32)> {
    if pos.distance >= max_distance {
        return vec![];
    }

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
            Some((
                Pos {
                    x: new_pos.0,
                    y: new_pos.1,
                    distance: pos.distance + 1,
                },
                1,
            ))
        })
        .collect()
}

fn part1(input: &str, steps: u32) -> Result<u32, Error> {
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

    let (paths, _) = astar_bag(
        &position,
        |p| get_successors(&grid, *p, steps),
        // manhattan distance heuristic
        |p| steps - p.distance as u32,
        |p| p.distance == steps,
    )
    .expect("found paths");

    let mut final_steps: Vec<Pos> = paths.map(|path| *path.last().unwrap()).collect();
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

    Ok(final_steps.len() as u32)
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input, 64).expect("found result");
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(include_str!("input1_test.txt"), 6).expect("run without errors");
        assert_eq!(result, 16);
    }
}
