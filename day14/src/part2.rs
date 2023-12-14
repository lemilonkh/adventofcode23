use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::Hasher,
};

#[derive(Debug)]
struct HistoryEntry {
    load: usize,
    cycle: u32,
}

fn part1(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let width = grid[0].len();
    let height = grid.len();
    let mut history: HashMap<u64, HistoryEntry> = HashMap::new();

    let mut cycle_hash: u64 = 0;
    let mut repeating_cycle: u32 = 0;

    for cycle in 0..1_000_000_000_u32 {
        let load = (0..width)
            .map(|x| {
                (0..height)
                    .map(|y| if grid[y][x] == 'O' { height - y } else { 0 })
                    .sum::<usize>()
            })
            .sum::<usize>();

        println!("Cycle {}, Load {}", cycle, load);
        let board: String = grid
            .iter()
            .map(|row| row.iter().collect::<String>() + "\n")
            .collect();
        println!("{}", board);

        let mut hasher = DefaultHasher::new();
        hasher.write(board.as_bytes());
        let hash = hasher.finish();
        if history.contains_key(&hash) {
            repeating_cycle = cycle;
            cycle_hash = hash;
            break;
        }

        history.entry(hash).or_insert(HistoryEntry { load, cycle });

        // North
        for x in 0..width {
            let mut blocked_pos = 0_usize;
            for y in 0..height {
                let tile = grid[y][x];
                match tile {
                    '.' => continue,
                    '#' => blocked_pos = y + 1,
                    'O' => {
                        grid[y][x] = '.';
                        grid[blocked_pos][x] = 'O';
                        blocked_pos += 1;
                    }
                    _ => eprintln!("Invalid character {}", tile),
                }
            }
        }

        // West
        for y in 0..height {
            let mut blocked_pos = 0_usize;
            for x in 0..width {
                let tile = grid[y][x];
                match tile {
                    '.' => continue,
                    '#' => blocked_pos = x + 1,
                    'O' => {
                        grid[y][x] = '.';
                        grid[y][blocked_pos] = 'O';
                        blocked_pos += 1;
                    }
                    _ => eprintln!("Invalid character {}", tile),
                }
            }
        }

        // South
        for x in 0..width {
            let mut blocked_pos = height - 1;
            for y in (0..height).rev() {
                let tile = grid[y][x];
                match tile {
                    '.' => continue,
                    '#' => blocked_pos = if y > 0 { y - 1 } else { 0 },
                    'O' => {
                        grid[y][x] = '.';
                        grid[blocked_pos][x] = 'O';
                        if blocked_pos > 0 {
                            blocked_pos -= 1;
                        }
                    }
                    _ => eprintln!("Invalid character {}", tile),
                }
            }
        }

        // East
        for y in 0..height {
            let mut blocked_pos = width - 1;
            for x in (0..width).rev() {
                let tile = grid[y][x];
                match tile {
                    '.' => continue,
                    '#' => blocked_pos = if x > 0 { x - 1 } else { 0 },
                    'O' => {
                        grid[y][x] = '.';
                        grid[y][blocked_pos] = 'O';
                        if blocked_pos > 0 {
                            blocked_pos -= 1;
                        }
                    }
                    _ => eprintln!("Invalid character {}", tile),
                }
            }
        }
    }

    let prev_entry = history.get(&cycle_hash).expect("found previous entry");
    println!(
        "Found cycle! Current cycle {}, previous cycle {}, load {}",
        repeating_cycle, prev_entry.cycle, prev_entry.load
    );
    let cycle_len = repeating_cycle - prev_entry.cycle;
    let final_offset = (1_000_000_000_u32 - repeating_cycle) % cycle_len;
    let final_cycle = prev_entry.cycle + final_offset;
    let final_entry = history
        .iter()
        .find(|(_hash, entry)| entry.cycle == final_cycle)
        .expect("found final cycle entry")
        .1;
    println!("Final entry {:?}", final_entry);
    final_entry.load
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
        assert_eq!(result, 64);
    }
}
