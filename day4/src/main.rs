fn get_input() -> (Vec<u32>, Vec<Vec<u32>>) {
    let file = std::fs::read_to_string("input/input.txt").unwrap();
    let mut split = file.split_whitespace();
    let draws = split
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let boards = split
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>()
        .chunks(25)
        .map(|c| c.into())
        .collect();
    (draws, boards)
}

fn board_win(draws: &[u32], board: &[u32]) -> bool {
    (0..5).any(|i|
    board.iter().skip(i*5).take(5).all(|n| draws.contains(n)) ||
        board.iter().skip(i).step_by(5).all(|n| draws.contains(n)))
}

fn board_unmarked_sum(draws: &[u32], board: &[u32]) -> u32 {
    board.iter().filter(|n| !draws.contains(n)).sum()
}

fn part1(draws: &[u32], boards: &[Vec<u32>]) -> u32 {
    for i in 5..draws.len() {
        for board in boards {
            if board_win(&draws[0..i], board) {
                return board_unmarked_sum(&draws[0..i], board) * draws[i-1];
            }
        }
    }
    panic!()
}

fn part2(draws: &[u32], boards: &[Vec<u32>]) -> u32 {
    for i in (5..draws.len()).rev() {
        for board in boards {
            if !board_win(&draws[0..i], board) {
                return board_unmarked_sum(&draws[0..i+1], board) * draws[i];
            }
        }
    }
    panic!()
}

fn main() {
    let (numbers, boards) = get_input();
    println!("part1: {}", part1(&numbers, &boards));
    println!("part2: {}", part2(&numbers, &boards));
}
