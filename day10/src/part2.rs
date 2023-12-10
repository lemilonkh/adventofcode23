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

fn flood_fill(x: usize, y: usize, target: char, replacement: char, grid: &mut Vec<Vec<char>>) {
    if grid[y][x] == target {
        grid[y][x] = replacement;

        if y > 0 {
            flood_fill(x, y - 1, target, replacement, grid);
        }
        if x > 0 {
            flood_fill(x - 1, y, target, replacement, grid);
        }
        if y < grid.len() - 1 {
            flood_fill(x, y + 1, target, replacement, grid);
        }
        if x < grid[0].len() - 1 {
            flood_fill(x + 1, y, target, replacement, grid);
        }
    }
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
    let mut next_direction: Option<Direction> = None;
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    // let mut status_grid: Vec<Vec<char>> = (0..height).map(|_i| vec!['.'; width as usize]).collect();
    let mut status_grid: Vec<Vec<char>> = (0..height * 2)
        .map(|_i| vec!['.'; width as usize * 2])
        .collect();

    'outer: loop {
        let mut found_next_pipe = false;

        for direction in Direction::iter() {
            if next_direction.is_some() && next_direction.as_ref() != Some(&direction) {
                continue;
            }

            // get neighboring character and do bounds check
            let (x, y) = get_direction_delta(&direction);
            let target = (position.0 as i32 + x, position.1 as i32 + y);
            if target.0 < 0 || target.0 >= width || target.1 < 0 || target.1 >= height {
                continue;
            }
            let target_char = grid[target.1 as usize][target.0 as usize];
            if target_char == 'S' {
                status_grid[position.1 * 2][position.0 * 2] = 'L';
                status_grid[(position.1 as i32 * 2 + y) as usize]
                    [(position.0 as i32 * 2 + x) as usize] = 'L';
                break 'outer;
            }

            // check if adjacent char is a pipe and it allows connecting with the previous pipe
            let directions = get_pipe_directions(target_char);
            if directions.is_none() {
                continue;
            }
            let directions = directions.unwrap();

            if direction.opposite() == directions.0 || direction.opposite() == directions.1 {
                found_next_pipe = true;
                status_grid[position.1 * 2][position.0 * 2] = 'L';
                status_grid[(position.1 as i32 * 2 + y) as usize]
                    [(position.0 as i32 * 2 + x) as usize] = 'L';
                position = (target.0 as usize, target.1 as usize);

                next_direction = if direction.opposite() == directions.0 {
                    Some(directions.1)
                } else {
                    Some(directions.0)
                };

                break;
            }
        }
        assert_eq!(found_next_pipe, true);
    }

    // flood fill from every outer border tile of the grid
    for y in 0..grid.len() * 2 {
        flood_fill(0, y, '.', 'O', &mut status_grid);
        flood_fill((width as usize - 1) * 2 + 1, y, '.', 'O', &mut status_grid);
    }
    for x in 1..grid[0].len() * 2 {
        flood_fill(x, 0, '.', 'O', &mut status_grid);
        flood_fill(x, (height as usize - 1) * 2 + 1, '.', 'O', &mut status_grid);
    }

    status_grid
        .iter()
        .for_each(|row| println!("{}", row.iter().collect::<String>()));

    status_grid.iter().step_by(2).fold(0, |acc, row| {
        acc + row
            .iter()
            .step_by(2)
            .fold(0, |acc, c| if *c == '.' { acc + 1 } else { acc })
    })
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
        let result = part1(include_str!("input3_test.txt"));
        assert_eq!(result, 4);
        let result = part1(include_str!("input4_test.txt"));
        assert_eq!(result, 8);
    }
}
