fn get_input() -> Vec<usize> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn simulate(input: &[usize], days: usize) -> usize {
    let mut fish_count = std::collections::VecDeque::from([0; 9]);
    for &i in input {
        fish_count[i] += 1;
    }
    for _ in 0..days {
        fish_count.rotate_left(1);
        fish_count[6] += fish_count[8];
    }
    fish_count.iter().sum()
}

fn main() {
    let input = get_input();
    println!("part1: {}", simulate(&input, 80));
    println!("part2: {}", simulate(&input, 256));
}
