fn get_input() -> Vec<String> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|s| s.into())
        .collect()
}

fn most_common_bit(input: &[String], location: usize) -> u32 {
    let mut count = 0;
    for s in input {
        count += s.chars().nth(location).unwrap().to_digit(2).unwrap();
    }
    (2 * count >= input.len() as u32) as u32
}

fn part1(input: &[String]) -> u32 {
    let length_bits = input[0].chars().count();
    let mut most_bits = 0;
    for i in 0..length_bits {
        most_bits += most_common_bit(&input, length_bits - i - 1) * 2u32.pow(i as u32);
    }
    most_bits * (most_bits ^ (2u32.pow(length_bits as u32)  - 1))
}

fn filter_by_bits(mut input: Vec<String>, most_same: bool) -> u32 {
    for i in 0..input[0].chars().count() {
        let most_bit = most_common_bit(&input, i);
        input.retain(|s| most_same == (s.chars().nth(i).unwrap().to_digit(2).unwrap() == most_bit));
        if input.len() == 1 {
            return u32::from_str_radix(&input[0], 2).unwrap();
        }
    }
    panic!();
}

fn part2(input: &[String]) -> u32 {
    filter_by_bits(input.to_vec(), true) * filter_by_bits(input.to_vec(), false)
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
