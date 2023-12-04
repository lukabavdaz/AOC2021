use itertools::Itertools;

fn get_input() -> (Vec<bool>, Vec<Vec<bool>>) {
    let file = std::fs::read_to_string("input/input.txt").unwrap();
    let (rule, init) = file.split_once("\n\n").unwrap();
    (
        rule.chars().map(|c| c == '#').collect(),
        init.lines()
            .map(|s| s.chars().map(|c| c == '#').collect())
            .collect(),
    )
}

fn simulate(gens: i32, (rule, init): &(Vec<bool>, Vec<Vec<bool>>)) -> usize {
    let mut bright = std::collections::HashSet::new();
    let (init_x, init_y) = (init[0].len(), init.len());
    for y in 0..init_y {
        for x in 0..init_x {
            if init[y][x] {
                bright.insert((x as i32, y as i32));
            }
        }
    }
    for g in 0..gens {
        let prev_gen = bright;
        bright = std::collections::HashSet::new();
        for (x, y) in ((-g-1)..=(init_x as i32 + g + 1)).cartesian_product((-g-1)..=(init_y as i32 + g + 1)) {
            let i = ((y-1)..=(y+1)).cartesian_product((x-1)..=(x+1)).fold(0, |acc, (y,x)| acc * 2 + (prev_gen.contains(&(x,y)) == (g % 2 == 0 || !rule[0] )) as usize );
            if rule[i] == (g % 2 != 0 || !rule[0] ) {
                bright.insert((x,y));
            }
        }
    }
    bright.iter().count()
}

fn main() {
    let input = get_input();
    println!("part1: {}", simulate(2, &input));
    println!("part1: {}", simulate(50, &input));
}
