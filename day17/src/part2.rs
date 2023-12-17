use array2d::{Array2D, Error};
use pathfinding::directed::astar::astar;
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

    fn opposite(&self) -> Direction {
        match self {
            NORTH => SOUTH,
            EAST => WEST,
            SOUTH => NORTH,
            WEST => EAST,
        }
    }

    fn char(&self) -> char {
        match self {
            NORTH => '^',
            EAST => '>',
            SOUTH => 'v',
            WEST => '<',
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

fn is_in_bounds(position: (i32, i32), width: usize, height: usize) -> bool {
    position.0 >= 0 && position.1 >= 0 && position.0 < width as i32 && position.1 < height as i32
}

fn get_successors(grid: &Array2D<u32>, pos: Pos, target: Pos) -> Vec<(Pos, u32)> {
    Direction::iter()
        .filter_map(|direction| {
            if direction == pos.direction.opposite() {
                return None;
            }
            let direction_count = if direction == pos.direction {
                pos.count + 1
            } else {
                0
            };

            // go max 10 times in the same direction
            if direction_count > 9 {
                return None;
            }
            // go min 4 times in the same direction, but not at the ends
            // to prevent hard coding the initial/ final direction
            if !(pos.x == 0 && pos.y == 0) && pos != target {
                if pos.count < 3 && (direction != pos.direction) {
                    return None;
                }
            }
            let delta = direction.delta();
            let new_pos = (pos.x + delta.0, pos.y + delta.1);
            is_in_bounds(new_pos, grid.num_columns(), grid.num_rows()).then(|| {
                (
                    Pos {
                        x: new_pos.0,
                        y: new_pos.1,
                        direction,
                        count: direction_count,
                    },
                    *grid
                        .get(new_pos.1 as usize, new_pos.0 as usize)
                        .expect("found heat loss"),
                )
            })
        })
        .collect()
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

    let width = grid.num_columns();
    let height = grid.num_rows();

    let position = Pos {
        x: 0,
        y: 0,
        direction: EAST,
        count: 0,
    };
    let target = Pos {
        x: width as i32 - 1,
        y: height as i32 - 1,
        direction: SOUTH,
        count: 0,
    };

    let (path, heat_loss) = astar(
        &position,
        |p| get_successors(&grid, *p, target),
        // manhattan distance heuristic
        |p| ((p.x - target.x).abs() + (p.y - target.y).abs()) as u32,
        |p| *p == target,
    )
    .expect("found path");

    println!("Found path with length {}", heat_loss);

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let pos = path.iter().find(|p| p.x == x && p.y == y);
            if pos.is_some() {
                print!("{}", pos.unwrap().direction.char());
            } else {
                print!("{}", grid.get(y as usize, x as usize).unwrap());
            }
        }
        println!();
    }

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
        let result = part1(include_str!("input1_test.txt")).expect("run without errors");
        assert_eq!(result, 94);
    }
}
