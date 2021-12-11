fn get_input() -> Vec<i32> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_fuel(input: &[i32], fuel: impl Fn(i32) -> i32) -> i32 {
    (*input.iter().min().unwrap()..=*input.iter().max().unwrap())
        .map(|t| input.iter().map(|&n| fuel((n - t).abs())).sum())
        .min()
        .unwrap()
}

fn part1(input: &[i32]) -> i32 {
    get_fuel(input, |dist| dist)
}

fn part2(input: &[i32]) -> i32 {
    get_fuel(&input, |dist| dist * (dist + 1) / 2)
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
