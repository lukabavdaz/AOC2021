fn get_input() -> Vec<Vec<u32>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn run_cycle(input: &mut Vec<Vec<u32>>) -> u32 {
    for i in input.iter_mut().flatten() {
        *i += 1;
    }

    let mut to_flash = input
        .iter()
        .flatten()
        .enumerate()
        .filter(|(_, x)| **x > 9)
        .map(|(i, _)| (i % input[0].len(), i / input[0].len()))
        .collect::<std::collections::VecDeque<(usize, usize)>>();

    let mut flashes = 0;
    while let Some((x, y)) = to_flash.pop_front() {
        input[y][x] = 0;
        flashes += 1;
        for i in (x as i32 - 1)..=(x as i32 + 1) {
            for j in (y as i32 - 1)..=(y as i32 + 1) {
                if i >= 0 && j >= 0 {
                    let (i, j) = (i as usize, j as usize);
                    if i < input[0].len() && j < input.len() && input[j][i] != 0 {
                        input[j][i] += 1;
                        if input[j][i] == 10 {
                            to_flash.push_back((i, j));
                        }
                    }
                }
            }
        }
    }
    flashes
}

fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let mut input_copy = input.clone();
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += run_cycle(&mut input_copy);
    }
    flashes
}

fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let mut input_copy = input.clone();
    let mut counter = 1;
    while run_cycle(&mut input_copy) != 100 {
        counter += 1;
    }
    counter
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
