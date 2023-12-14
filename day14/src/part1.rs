fn part1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut rock_heights: Vec<usize> = vec!();
    for x in 0..grid[0].len() {
        let mut blocked_height = 0_usize;
        for y in 0..grid.len() {
            let tile = grid[y][x];
            match tile {
                '.' => continue,
                '#' => blocked_height = y + 1,
                'O' => {
                    rock_heights.push(blocked_height);
                    blocked_height += 1;
                },
                _ => eprintln!("Invalid character {}", tile),
            }
        }
    }

    let height = grid.len();
    rock_heights
        .iter()
        .map(|h| height - h)
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
        assert_eq!(result, 136);
    }
}
