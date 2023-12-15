fn hash(step: &str) -> u32 {
    let mut result: u32 = 0;
    for c in step.as_bytes() {
        result += *c as u32;
        result *= 17;
        result %= 256;
    }
    result
}

fn part1(input: &str) -> u32 {
    input
        .replace("\n", "")
        .split(",")
        .map(hash)
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
        assert_eq!(result, 1320);
    }
}
