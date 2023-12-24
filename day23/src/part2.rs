use std::collections::BTreeMap;

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
    fn delta(&self) -> (isize, isize) {
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
    x: usize,
    y: usize,
}
impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn is_in_bounds(position: (isize, isize), width: usize, height: usize) -> bool {
    position.0 >= 0
        && position.1 >= 0
        && position.0 < width as isize
        && position.1 < height as isize
}

fn get_successors(grid: &Array2D<u8>, pos: &Pos) -> Vec<Pos> {
    Direction::iter()
        .filter_map(|direction| {
            let delta = direction.delta();
            let new_pos = (pos.x as isize + delta.0, pos.y as isize + delta.1);
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
                x: new_pos.0 as usize,
                y: new_pos.1 as usize,
            })
        })
        .collect()
}

fn depth_first_search(
    graph: &BTreeMap<Pos, Vec<(Pos, u32)>>,
    visited_grid: &mut Array2D<bool>,
    position: Pos,
) -> Option<u32> {
    if position.y == visited_grid.num_rows() - 1 {
        return Some(0);
    }

    let mut max_length = None;
    for &(pos, dist) in &graph[&position] {
        if !visited_grid.get(pos.y as usize, pos.x as usize).unwrap() {
            visited_grid
                .set(pos.y as usize, pos.x as usize, true)
                .expect("wrote to grid");
            if let Some(d) = depth_first_search(graph, visited_grid, pos) {
                max_length = Some(max_length.unwrap_or(0).max(d + dist));
            }
            visited_grid
                .set(pos.y as usize, pos.x as usize, false)
                .expect("wrote to grid");
        }
    }

    max_length
}

fn part1(input: &str) -> Result<u32, Error> {
    let lines: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

    let grid = Array2D::from_rows(&lines)?;
    let width = grid.num_columns();
    let height = grid.num_rows();

    // construct graph of all nodes and their neighbors as edges
    let mut graph: BTreeMap<Pos, Vec<(Pos, u32)>> = BTreeMap::new();

    for y in 0..height {
        for x in 0..width {
            let pos = Pos { x, y };
            let node = graph.entry(pos).or_default();
            for successor in get_successors(&grid, &pos) {
                node.push((successor, 1));
            }
        }
    }

    // collapse linear corridors to single edges
    let corridors = graph
        .iter()
        .filter(|(_, edges)| edges.len() == 2)
        .map(|(node, _)| *node)
        .collect::<Vec<Pos>>();

    for pos in corridors {
        let neighbors = graph.remove(&pos).unwrap();
        let (pos1, dist1) = neighbors[0];
        let (pos2, dist2) = neighbors[1];
        let neighbor1 = graph.get_mut(&pos1);
        if let Some(n1) = neighbor1 {
            if let Some(i) = n1.iter().position(|&(p, _)| p == pos) {
                n1[i] = (pos2, dist1 + dist2);
            }
        }
        let neighbor2 = graph.get_mut(&pos2);
        if let Some(n2) = neighbor2 {
            if let Some(i) = n2.iter().position(|&(p, _)| p == pos) {
                n2[i] = (pos1, dist1 + dist2);
            }
        }
    }

    let start_pos = Pos { x: 1, y: 0 };
    let mut seen = Array2D::filled_with(false, height, width);
    let max_length = depth_first_search(&graph, &mut seen, start_pos).expect("found longest path");

    Ok(max_length)
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
