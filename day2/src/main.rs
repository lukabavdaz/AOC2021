fn get_input() -> Vec<(String, i64)> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(a, b)| (a.into(), b.parse().unwrap()))
        .collect()
}

fn part1(input: &[(String, i64)]) -> i64 {
    let (mut x, mut y) = (0, 0);
    for (direction, amount) in input {
        match direction.as_str() {
            "forward" => x += amount,
            "down" => y += amount,
            "up" => y -= amount,
            _ => (),
        }
    }
    x * y
}

fn part2(input: &[(String, i64)]) -> i64 {
    let (mut x, mut y, mut aim) = (0, 0, 0);
    for (direction, amount) in input {
        match direction.as_str() {
            "forward" => {
                x += amount;
                y += aim * amount;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => (),
        }
    }
    x * y
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
