use std::collections::HashSet;

fn get_input() -> Vec<(Vec<String>, Vec<String>)> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|s| s.split_once('|').unwrap())
        .map(|(s1, s2)| {
            (
                s1.trim().split(' ').map(|s| s.into()).collect(),
                s2.trim().split(' ').map(|s| s.into()).collect(),
            )
        })
        .collect()
}

fn get_set(s1: &[String], l: usize) -> HashSet<char> {
    s1.iter().find(|s| s.len() == l).unwrap().chars().collect()
}

fn map_and_count(s1: &[String], s2: &[String]) -> usize {
    let (set2, set4) = (get_set(&s1, 2), get_set(&s1, 4));
    s2.iter()
        .map(|s| s.chars().collect::<HashSet<_>>())
        .map(|set| (set.len(), (&set & &set2).len(), (&set & &set4).len()))
        .map(|mask| match mask {
            (6, 2, 3) => 0,
            (2, _, _) => 1,
            (5, 1, 2) => 2,
            (5, 2, 3) => 3,
            (4, _, _) => 4,
            (5, 1, 3) => 5,
            (6, 1, 3) => 6,
            (3, _, _) => 7,
            (7, _, _) => 8,
            (6, 2, 4) => 9,
            _ => panic!(),
        })
        .fold(0, |acc, x| acc * 10 + x)
}

fn part1(input: &[(Vec<String>, Vec<String>)]) -> usize {
    input
        .iter()
        .map(|(_, s2)| {
            s2.iter()
                .filter(|s| [2, 3, 4, 7].contains(&s.len()))
                .count()
        })
        .sum()
}

fn part2(input: &[(Vec<String>, Vec<String>)]) -> usize {
    input.iter().map(|(s1, s2)| map_and_count(s1, s2)).sum()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
