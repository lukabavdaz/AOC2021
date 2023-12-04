use std::collections::HashMap;

fn get_input() -> Vec<(String, String)> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .map(|(s1,s2)| (s1.into(), s2.into()))
        .collect()
}

fn traverse_path(path: Vec<&String>, double_allowed: bool, map: &HashMap<&String, Vec<&String>>) -> u32 {
    let mut path_count = 0;
    if let Some(last) = path.last() {
        if last.as_str() == "end" {
            return 1;
        }
        if let Some(new_locs) = map.get(last) {
            for new_loc in new_locs {
                if !(path.contains(new_loc) && new_loc != &&new_loc.to_uppercase()) {
                    let mut path_copy = path.clone();
                    path_copy.push(new_loc);
                    path_count += traverse_path(path_copy, double_allowed, map);
                } else if double_allowed {
                    if new_loc.as_str() != "start" {
                        let mut path_copy = path.clone();
                        path_copy.push(new_loc);
                        path_count += traverse_path(path_copy, false, map);
                    }
                }
            }
        }
    }
    path_count
}

fn part1(input: &[(String,String)]) -> u32 {
    let mut map = HashMap::new();
    for (from, to) in input {
        map.entry(from).or_insert(vec![]).push(to);
        map.entry(to).or_insert(vec![]).push(from);
    }
    traverse_path(vec![&"start".to_owned()], false, &map)
}

fn part2(input: &[(String,String)]) -> u32 {
    let mut map = HashMap::new();
    for (from, to) in input {
        map.entry(from).or_insert(vec![]).push(to);
        map.entry(to).or_insert(vec![]).push(from);
    }
    traverse_path(vec![&"start".to_owned()], true, &map)
}

fn main() {
    let input = get_input();
    let t = std::time::Instant::now();
    let p2 = part2(&input);
    println!("part1: {}", part1(&input));
    println!("part2: {}", p2);
    println!("time: {}", t.elapsed().as_millis());
}