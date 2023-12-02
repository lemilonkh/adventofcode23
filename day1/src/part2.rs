fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

static NUMBERS: &'static [&'static str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "1", "2", "3", "4", "5", "6", "7", "8", "9",
];

fn find_digit(line: &str, reverse: bool) -> Option<u32> {
    let indices = NUMBERS
        .iter()
        .enumerate()
        .map(|(i, num)| {
            if reverse {
                (line.rfind(num), i)
            } else {
                (line.find(num), i)
            }
        })
        .filter(|x| x.0.is_some());

    let outermost = if reverse {
        indices.max_by_key(|x| x.0)
    } else {
        indices.min_by_key(|x| x.0)
    };

    outermost.map(|(_index, number)| {
        if number > 8 {
            number as u32 - 8
        } else {
            number as u32 + 1
        }
    })
}

fn part2(input: &str) -> u32 {
    input
        .split("\n")
        .map(|line| {
            let first_number = find_digit(line, false);
            let second_number = find_digit(line, true);
            if first_number.is_none() || second_number.is_none() {
                eprintln!("Invalid input line {}", line);
                return 0;
            }
            dbg!(first_number.unwrap() * 10 + second_number.unwrap())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2(include_str!("input2_test.txt"));
        assert_eq!(result, 281);
    }
}
