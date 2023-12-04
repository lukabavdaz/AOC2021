fn get_input() -> Vec<Vec<i32>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '>' => 1,
                    'v' => 2,
                    _ => 0,
                })
                .collect()
        })
        .collect()
}

fn step(prev_state: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (x_size, y_size) = (prev_state[0].len(), prev_state.len());
    let mut new_state = prev_state.clone();

    for y in 0..y_size {
        for x in 0..x_size {
            if prev_state[y][x] == 1 {
                if x + 1 < x_size {
                    if prev_state[y][x+1] == 0 {
                        new_state[y][x] = 0;
                        new_state[y][x+1] = 1;
                    }
                } else {
                    if prev_state[y][0] == 0 {
                        new_state[y][x] = 0;
                        new_state[y][0] = 1;
                    }
                }
            }
        }
    }
    let prev_state2 = new_state.clone();
    for y in 0..y_size {
        for x in 0..x_size {
            if prev_state2[y][x] == 2 {
                if y + 1 < y_size {
                    if prev_state2[y+1][x] == 0 {
                        new_state[y][x] = 0;
                        new_state[y+1][x] = 2;
                    }
                } else {
                    if prev_state2[0][x] == 0 {
                        new_state[y][x] = 0;
                        new_state[0][x] = 2;
                    }
                }
            }
        }
    }
    new_state
}

fn print_map(input: &Vec<Vec<i32>>) {
    let (x_size, y_size) = (input[0].len(), input.len());
    for y in 0..y_size {
        for x in 0..x_size {
            match input[y][x] {
                1 => print!(">"),
                2 => print!("v"),
                _ => print!("."),
            }
        }
        println!();
    }
}

fn part1(input: &Vec<Vec<i32>>) -> i64 {
    let mut prev_state = input.clone();
    let mut i = 0;
    loop {
        i += 1;
        // println!("{}", i);
        // print_map(&prev_state);
        let new_state = step(&prev_state);
        if new_state == prev_state {
            return i;
        } else {
            prev_state = new_state;

        }
    }
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
}
