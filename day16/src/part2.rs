use std::cmp::max;
use Direction::*;

#[derive(Debug, PartialEq, Eq, Clone)]
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

    fn bit(&self) -> u32 {
        match self {
            NORTH => 1 << 0,
            EAST => 1 << 1,
            SOUTH => 1 << 2,
            WEST => 1 << 3,
        }
    }
}

fn is_in_bounds(position: (i32, i32), width: usize, height: usize) -> bool {
    position.0 >= 0 && position.1 >= 0 && position.0 < width as i32 && position.1 < height as i32
}

fn shoot_laser(
    grid: &Vec<Vec<char>>,
    status_grid: &mut Vec<Vec<u32>>,
    initial_direction: Direction,
    initial_position: (i32, i32),
) {
    let mut position = (initial_position.0, initial_position.1);
    let mut direction = initial_direction;
    let width = grid[0].len();
    let height = grid.len();

    while is_in_bounds(position, width, height) {
        if (status_grid[position.1 as usize][position.0 as usize] & direction.bit()) > 0 {
            // println!("Been there before, stopping!");
            break;
        }
        status_grid[position.1 as usize][position.0 as usize] |= direction.bit();
        let tile = grid[position.1 as usize][position.0 as usize];
        // println!("At {:?} going {:?} found {}", position, direction, tile);

        match tile {
            '.' => {}
            '/' => {
                direction = match direction {
                    NORTH => EAST,
                    EAST => NORTH,
                    SOUTH => WEST,
                    WEST => SOUTH,
                };
            }
            '\\' => {
                direction = match direction {
                    NORTH => WEST,
                    EAST => SOUTH,
                    SOUTH => EAST,
                    WEST => NORTH,
                };
            }
            '|' => {
                if direction == EAST || direction == WEST {
                    // println!("Splitting vertically!");
                    direction = NORTH;
                    shoot_laser(grid, status_grid, SOUTH, (position.0, position.1 + 1));
                }
            }
            '-' => {
                if direction == NORTH || direction == SOUTH {
                    // println!("Splitting horizontally!");
                    direction = WEST;
                    shoot_laser(grid, status_grid, EAST, (position.0 + 1, position.1));
                }
            }
            _ => eprintln!("Invalid tile {}", tile),
        }
        let delta = direction.delta();
        position.0 += delta.0;
        position.1 += delta.1;
    }
}

fn clear_grid(status_grid: &mut Vec<Vec<u32>>) {
    for row in status_grid {
        row.fill(0);
    }
}

fn count_energized_tiles(status_grid: &Vec<Vec<u32>>) -> u32 {
    // println!();
    status_grid
        .iter()
        /*.inspect(|row| {
            println!(
                "{}",
                row.iter()
                    .map(|s| if *s > 0 { '#' } else { '.' })
                    .collect::<String>()
            )
        })*/
        .map(|row| row.iter().map(|s| (*s > 0) as u32).sum::<u32>())
        .sum()
}

fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let width = grid[0].len();
    let height = grid.len();
    let mut status_grid: Vec<Vec<u32>> = vec![vec![0; width]; height];

    let horizontal_max_tiles = (0..width as i32)
        .map(|x| {
            clear_grid(&mut status_grid);
            shoot_laser(&grid, &mut status_grid, SOUTH, (x, 0));
            let tiles1 = count_energized_tiles(&status_grid);

            clear_grid(&mut status_grid);
            shoot_laser(&grid, &mut status_grid, NORTH, (x, height as i32 - 1));
            let tiles2 = count_energized_tiles(&status_grid);

            max(tiles1, tiles2)
        })
        .max()
        .expect("found max");

    let vertical_max_tiles = (0..height as i32)
        .map(|y| {
            clear_grid(&mut status_grid);
            shoot_laser(&grid, &mut status_grid, EAST, (0, y));
            let tiles1 = count_energized_tiles(&status_grid);

            clear_grid(&mut status_grid);
            shoot_laser(&grid, &mut status_grid, WEST, (width as i32 - 1, y));
            let tiles2 = count_energized_tiles(&status_grid);

            max(tiles1, tiles2)
        })
        .max()
        .expect("found max");

    max(horizontal_max_tiles, vertical_max_tiles)
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
        assert_eq!(result, 51);
    }
}
