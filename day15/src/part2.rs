#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u32,
}

fn hash(step: &str) -> u32 {
    let mut result: u32 = 0;
    for c in step.as_bytes() {
        result += *c as u32;
        result *= 17;
        result %= 256;
    }
    result
}

fn part1(input: &str) -> usize {
    let mut boxes: Vec<Vec<Lens>> = vec![vec!(); 256];
    let input = input.replace("\n", "");
    let steps = input.split(",").collect::<Vec<&str>>();

    for step in steps {
        if step.ends_with('-') {
            let label = step.strip_suffix('-').expect("- terminated string");
            let box_id = hash(label);
            let lenses = boxes.get_mut(box_id as usize).expect("box found");
            let index = lenses.iter().position(|l| l.label == label);
            if index.is_some() {
                lenses.remove(index.unwrap());
            }
        } else if step.contains('=') {
            let (label, num) = step.split_once('=').expect("= separated string");
            let box_id = hash(label);
            let lenses = boxes.get_mut(box_id as usize).expect("box found");
            let focal_length = num.parse().expect("valid focal length number");
            let index = lenses.iter().position(|l| l.label == label);
            let lens = Lens {
                label: label.to_owned(),
                focal_length,
            };
            if index.is_some() {
                lenses[index.unwrap()] = lens;
            } else {
                lenses.push(lens);
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(box_id, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(slot, lens)| (box_id + 1) * (slot + 1) * lens.focal_length as usize)
                .sum::<usize>()
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
    fn it_works() {
        let result = part1(include_str!("input1_test.txt"));
        assert_eq!(result, 145);
    }
}
