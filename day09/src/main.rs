use std::convert::TryFrom;

fn get_input() -> Vec<Vec<u32>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn get_adj(input: &Vec<Vec<u32>>, x: usize, y: usize) -> impl Iterator<Item = (usize, usize, u32)> + '_  {
    [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)]
        .into_iter()
        .filter_map(move |(i, j)| {
            Some((
                usize::try_from(x as i32 + i).ok()?,
                usize::try_from(y as i32 + j).ok()?,
            ))
        })
        .filter_map(move |(xi, yj)| Some((xi, yj,*input.get(xi)?.get(yj)?)))
}

fn get_low_points(input: &Vec<Vec<u32>>) -> Vec<(usize, usize, u32)> {
    let (size_x, size_y) = (input.len(), input[0].len());
    let mut low_points = vec![];

    for x in 0..size_x {
        for y in 0..size_y {
            let value = input[x][y];
            if get_adj(input, x, y).all(|(_,_,adj)| adj > value) {
                low_points.push((x,y,value));
            }
        }
    }
    low_points
}

fn part1(input: &Vec<Vec<u32>>) -> u32 {
    get_low_points(input).iter().fold(0, |acc, (_,_,value)| acc + value + 1)
}

fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let mut basin_sizes = vec![];

    for low_point in get_low_points(input) {
        let mut size = 0;
        let mut visited = std::collections::HashSet::new();
        let mut to_visit = std::collections::VecDeque::from([low_point]);

        while let Some((low_x, low_y, value)) = to_visit.pop_front() {
            if !visited.contains(&(low_x, low_y, value)) {
                size += 1;
                to_visit.extend(get_adj(input, low_x, low_y).filter(|(_,_,v_adj)| *v_adj >= value && *v_adj < 9 ));
                visited.insert((low_x, low_y, value));
            }
        }
        basin_sizes.push(size);
    }
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
