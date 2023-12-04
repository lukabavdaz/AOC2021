use itertools::Itertools;

fn get_input() -> (i32, i32, i32, i32) {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .trim()
        .split(|c| !char::is_numeric(c) && c != '-')
        .filter_map(|x| x.parse().ok())
        .collect_tuple()
        .unwrap()
}

fn hits_target((mut dx, mut dy): &(i32, i32), (min_x, max_x, min_y, max_y): &(i32, i32, i32, i32)) -> bool {
    let (mut x,mut y) = (0,0);
    while y >= *min_y {
        x += dx;
        y += dy;
        dx = 0.max(dx - 1);
        dy = dy - 1;
        if x >= *min_x && x <= *max_x && y >= *min_y && y <= *max_y {
            return true;
        }
    }
    false
}

fn part1(target: &(i32, i32, i32, i32)) -> i32 {
    (0..100)
        .cartesian_product(0..100)
        .filter(|p| hits_target(p, target))
        .map(|(_, dy)| (dy * (dy + 1)) / 2)
        .max()
        .unwrap()
}

fn part2(target: &(i32, i32, i32, i32)) -> usize {
    (0..=target.1+1)
        .cartesian_product((target.2 - 1)..1000)
        .filter(|p| hits_target(p, target))
        .count()
}

fn main() {
    let input = get_input();
    // println!("hits target: {}", hits_target(&(6,9),&input));
    println!("part1: {}", part1(&input));
    println!("part1: {}", part2(&input));
}
