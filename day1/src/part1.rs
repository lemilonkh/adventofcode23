fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn find_digit<'a, I>(line: &mut I) -> Option<u32>
where
    I: Iterator<Item = char>,
{
    line.find(|c| c >= &&'0' && c <= &&'9')
        .map(|c| c.to_digit(10))
        .flatten()
}

fn part1(input: &str) -> u32 {
    input
        .split("\n")
        .map(|line| {
            let first_number = find_digit(&mut line.chars().into_iter());
            let second_number = find_digit(&mut line.chars().rev().into_iter());
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
        let result = part1(include_str!("input1_test.txt"));
        assert_eq!(result, 142);
    }
}
