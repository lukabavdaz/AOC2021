use itertools::Itertools;

fn get_input() -> Vec<(i32, i32, i32, i32)> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|s| {
            s.split(|c| !char::is_numeric(c))
                .filter_map(|x| x.parse().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn overlaps<'a>(input: impl Iterator<Item = &'a (i32, i32, i32, i32)>) -> usize {
    let mut field = HashMap::new();
    for &(x1, y1, x2, y2) in input {
        let (dx, dy) = (x2 - x1, y2 - y1);
        let (dx_s, dy_s) = (dx.signum(), dy.signum());
        for i in 0..=dx.abs().max(dy.abs()) {
            *field.entry((x1 + dx_s * i, y1 + dy_s * i)).or_insert(0) += 1;
        }
    }
    field.values().filter(|&&v| v > 1).count()
}

fn part1(input: &[(i32, i32, i32, i32)]) -> usize {
    overlaps(input.iter().filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2))
}

fn part2(input: &[(i32, i32, i32, i32)]) -> usize {
    overlaps(input.iter().filter(|(x1, y1, x2, y2)| {
        x1 == x2 || y1 == y2 || (x1 - x2) == (y1 - y2) || (x1 - x2) == -(y1 - y2)
    }))
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
