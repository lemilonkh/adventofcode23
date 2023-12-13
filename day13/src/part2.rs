fn search_reflection(grid: &Vec<Vec<char>>) -> Option<usize> {
    let width = grid[0].len();
    for x in 0..width - 1 {
        let mut smudges: u32 = 0;

        for d in 1..=width / 2 {
            let left_index = x as i32 - (d as i32 - 1);
            if left_index < 0 || x + d >= width {
                break;
            }

            smudges += (0..grid.len())
                .map(|y| (grid[y][left_index as usize] != grid[y][x + d]) as u32)
                .sum::<u32>();
            if smudges > 1 {
                break;
            }
        }

        if smudges == 1 {
            return Some(x + 1);
        }
    }

    return None;
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let patterns = input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| line.chars().collect())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    patterns
        .iter()
        .map(|pattern| {
            let horizontal = search_reflection(pattern).unwrap_or(0);
            let transposed = &transpose(pattern.clone());
            let vertical = search_reflection(transposed).unwrap_or(0) * 100;
            println!("H {} V {}", horizontal, vertical);
            horizontal + vertical
        })
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
    fn test_reflection() {
        let result = search_reflection(&vec!["#.###.##.".chars().collect()]);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn it_works() {
        let result = part1(include_str!("input1_test.txt"));
        assert_eq!(result, 400);
    }
}
