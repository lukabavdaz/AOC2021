fn get_input() -> Vec<Vec<u32>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn get_neighbors(x: usize, y: usize, size_x: usize, size_y: usize) -> impl Iterator<Item = (usize, usize)> {
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .map(move |(i, j)| (i + x as i32, j + y as i32))
        .filter(move |&(i, j)| i >= 0 && j >= 0 && i < size_x as i32 && j < size_y as i32)
        .map(|(i, j)| (i as usize, j as usize))
}

fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let (size_x, size_y) = (input.len(), input[0].len());
    let mut visited = vec![vec![None; size_y]; size_x];
    let mut to_visit = std::collections::BinaryHeap::new();
    to_visit.push(std::cmp::Reverse((0, 0, 0)));
    while visited[size_x - 1][size_y - 1] == None {
        let (value, x, y) = to_visit.pop().unwrap().0;
        if visited[x][y] == None {
            visited[x][y] = Some(value);
            to_visit.extend(
                get_neighbor_pos(x, y, size_x, size_y)
                    .filter(|&(i, j)| visited[i][j] == None)
                    .map(|(i, j)| std::cmp::Reverse((input[i][j] + value, i, j))),
            );
        }
    }
    visited[size_x - 1][size_y - 1].unwrap()
}

fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let (size_x, size_y) = (input.len(), input[0].len());
    let mut new_input = vec![vec![0; size_y * 5]; size_x * 5];
    for x in 0..size_x * 5 {
        for y in 0..size_y * 5 {
            new_input[x][y] =
                (input[x % size_x][y % size_y] + (x / size_x) as u32 + (y / size_y) as u32 - 1) % 9
                    + 1;
        }
    }
    part1(&new_input)
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
