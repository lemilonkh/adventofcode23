use itertools::Itertools;

fn part1(input: &str, scale_factor: i64) -> i64 {
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

    let mut galaxies: Vec<(i64, i64)> = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '#' {
                let x_offset = empty_cols.iter().filter(|c| **c < x).count() as i64;
                let y_offset = empty_rows.iter().filter(|r| **r < y).count() as i64;

                // println!("At {}/{}, {} empty cols and {} empty rows", x, y, x_offset, y_offset);

                galaxies.push((
                    x as i64 + x_offset * (scale_factor - 1),
                    y as i64 + y_offset * (scale_factor - 1),
                ))
            }
        }
    }

    // compute manhattan distance of all pairs of galaxies
    galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a.0 - b.0).abs() + (a.1 - b.1).abs())
        .sum()
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input, 1_000_000_i64);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(include_str!("input1_test.txt"), 2);
        assert_eq!(result, 374);

        let result = part1(include_str!("input1_test.txt"), 10);
        assert_eq!(result, 1030);

        let result = part1(include_str!("input1_test.txt"), 100);
        assert_eq!(result, 8410);
    }
}
