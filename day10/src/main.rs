fn get_input() -> Vec<String> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|s| s.into())
        .collect()
}

fn analyze_line(line: &String) -> Result<Vec<char>, u64> {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' if stack.pop() != Some('(') => return Err(3),
            ']' if stack.pop() != Some('[') => return Err(57),
            '}' if stack.pop() != Some('{') => return Err(1197),
            '>' if stack.pop() != Some('<') => return Err(25137),
            _ => (),
        }
    }
    Ok(stack)
}

fn part1(input: &[String]) -> u64 {
    input.iter().filter_map(|l| analyze_line(l).err()).sum()
}

fn part2(input: &[String]) -> u64 {
    let mut scores = vec![];
    for l in input {
        if let Ok(s) = analyze_line(l) {
            scores.push(s.iter().rev().fold(0, |acc, &c| match c {
                '(' => acc * 5 + 1,
                '[' => acc * 5 + 2,
                '{' => acc * 5 + 3,
                '<' => acc * 5 + 4,
                _ => panic!(),
            }));
        }
    }
    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
