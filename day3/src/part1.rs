use std::cmp::max;

fn is_symbol(c: char) -> bool {
    c != '.' && !is_number(c)
}

fn is_number(c: char) -> bool {
    c >= '0' && c <= '9'
}

static OFFSETS: &'static [(i32, i32)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];

fn has_adjacent_symbol(grid: &Vec<Vec<char>>, cur_x: usize, cur_y: usize, digits: usize) -> bool {
    let mut found_symbol = false;
    let x_start = max(cur_x - digits, 0);
    'outer: for x in x_start..cur_x {
        for offset in OFFSETS {
            let ny = cur_y as i32 + offset.1;
            let nx = x as i32 + offset.0;
            if nx < 0 || nx >= grid[0].len() as i32 || ny < 0 || ny >= grid.len() as i32 {
                continue;
            }

            let c = grid[ny as usize][nx as usize];
            if is_symbol(c) {
                found_symbol = true;
                println!("Found symbol {}", c);
                break 'outer;
            }
        }
    }
    found_symbol
}

fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line.chars().collect())
        .collect();
    let mut valid_numbers: Vec<u32> = vec![];

    let width = grid[0].len();
    for y in 0..grid.len() {
        let mut current_number = 0;
        let mut digits = 0;

        for x in 0..width {
            let c = grid[y][x];
            if is_number(c) {
                current_number *= 10;
                current_number += c.to_digit(10).expect("valid digit");
                digits += 1;
            } else {
                if current_number > 0 {
                    dbg!(current_number);
                    if has_adjacent_symbol(&grid, x, y, digits) {
                        valid_numbers.push(current_number);
                    }
                }
                digits = 0;
                current_number = 0;
            }
        }

        if current_number > 0 {
            if has_adjacent_symbol(&grid, width - 1, y, digits) {
                valid_numbers.push(current_number);
            }
        }
    }

    valid_numbers.iter().sum()
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
        assert_eq!(result, 4361);
    }
}
