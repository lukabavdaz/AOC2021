fn get_input() -> (Vec<char>, Vec<(Vec<char>, char)>) {
    let file = std::fs::read_to_string("input/input.txt").unwrap();
    let (start, b) = file.trim().split_once("\n\n").unwrap();
    let rules = b
        .lines()
        .map(|s| s.split_once(" -> ").unwrap())
        .map(|(ss1, ss2)| (ss1.chars().collect(), ss2.chars().next().unwrap()))
        .collect();
    (start.chars().collect(), rules)
}

fn get_common_diff((polymer, rules): &(Vec<char>, Vec<(Vec<char>, char)>), steps: u32) -> u64 {
    let mut rules_map = std::collections::HashMap::new();
    for (a, b) in rules {
        rules_map.insert([a[0], a[1]], b);
    }
    let mut rules_count = std::collections::HashMap::new();
    for p in polymer.windows(2) {
        *rules_count.entry([p[0], p[1]]).or_insert(0) += 1u64;
    }

    for _ in 0..steps {
        let mut new_rules = std::collections::HashMap::new();
        for (&k, &v) in rules_count.iter() {
            let new_char = rules_map[&k];
            *new_rules.entry([k[0], *new_char]).or_insert(0) += v;
            *new_rules.entry([*new_char, k[1]]).or_insert(0) += v;
        }
        rules_count = new_rules;
    }

    let mut letter_counts = std::collections::HashMap::new();
    for (&k, &v) in rules_count.iter() {
        *letter_counts.entry(k[1]).or_insert(0) += v;
    }
    *letter_counts.get_mut(&polymer.first().unwrap()).unwrap() += 1;

    let most = letter_counts.values().max().unwrap();
    let least = letter_counts.values().min().unwrap();

    most - least
}

fn main() {
    let input = get_input();
    println!("part1: {}", get_common_diff(&input, 10));
    println!("part2: {}", get_common_diff(&input, 40));
}
