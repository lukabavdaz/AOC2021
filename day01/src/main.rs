fn get_input() -> Vec<i32> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1(input: &[i32]) -> usize {
    input.windows(2).filter(|w| w[1] > w[0]).count()
}

fn part2(input: &[i32]) -> usize {
    input.windows(4).filter(|w| w[3] > w[0]).count()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
