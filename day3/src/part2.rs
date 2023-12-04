use std::{cmp::max, collections::BTreeMap};

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

fn find_adjacent_gear(
    grid: &Vec<Vec<char>>,
    cur_x: usize,
    cur_y: usize,
    digits: usize,
) -> Option<(usize, usize)> {
    let x_start = max(cur_x - digits, 0);
    for x in x_start..cur_x {
        for offset in OFFSETS {
            let ny = cur_y as i32 + offset.1;
            let nx = x as i32 + offset.0;
            if nx < 0 || nx >= grid[0].len() as i32 || ny < 0 || ny >= grid.len() as i32 {
                continue;
            }

            let c = grid[ny as usize][nx as usize];
            if c == '*' {
                println!("Found symbol {}", c);
                return Some((nx as usize, ny as usize));
            }
        }
    }
    None
}

fn check_and_save_number(
    grid: &Vec<Vec<char>>,
    gears: &mut BTreeMap<(usize, usize), Vec<u32>>,
    number: u32,
    x: usize,
    y: usize,
    digits: usize,
) {
    if number > 0 {
        dbg!(number);
        let gear = find_adjacent_gear(&grid, x, y, digits);
        match gear {
            Some(pos) => gears
                .entry(pos)
                .or_insert(Vec::with_capacity(2))
                .push(number),
            _ => (),
        };
    }
}

fn part2(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line.chars().collect())
        .collect();
    let mut gears: BTreeMap<(usize, usize), Vec<u32>> = BTreeMap::new();

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
                check_and_save_number(&grid, &mut gears, current_number, x, y, digits);
                digits = 0;
                current_number = 0;
            }
        }

        check_and_save_number(&grid, &mut gears, current_number, width - 1, y, digits);
    }

    gears
        .values()
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers[0] * numbers[1])
        .sum()
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(include_str!("input1_test.txt"));
        assert_eq!(result, 467835);
    }
}
