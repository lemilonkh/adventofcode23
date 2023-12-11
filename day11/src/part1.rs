use itertools::Itertools;

fn part1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line.chars().collect())
        .collect();

    let empty_rows: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter_map(|(y, row)| row.iter().all(|c| *c == '.').then_some(y))
        .collect();
    let empty_cols: Vec<usize> = (0..grid[0].len())
        .filter_map(|x| (0..grid.len()).all(|y| grid[y][x] == '.').then_some(x))
        .collect();

    let mut galaxies: Vec<(i32, i32)> = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '#' {
                let x_offset = empty_cols.iter().filter(|c| **c < x).count() as i32;
                let y_offset = empty_rows.iter().filter(|r| **r < y).count() as i32;

                galaxies.push((x as i32 + x_offset, y as i32 + y_offset))
            }
        }
    }

    // compute manhattan distance of all pairs of galaxies
    galaxies
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|(a, b)| (a.0 - b.0).abs() + (a.1 - b.1).abs())
        .sum()
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
        assert_eq!(result, 374);
    }
}
