fn get_input() -> Vec<Vec<(i32, i32)>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| parse_snailfish(l))
        .collect()
}

fn parse_snailfish(input: &str) -> Vec<(i32, i32)> {
    let numbers = input
        .split(|c| !char::is_numeric(c))
        .filter_map(|s| s.parse().ok());
    let nesting = input
        .split(char::is_numeric)
        .filter(|s| !s.is_empty())
        .scan(0, |count, s| {
            *count += s.matches('[').count() as i32 - s.matches(']').count() as i32;
            Some(*count)
        });
    numbers.zip(nesting).collect()
}

fn reduce(mut n: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    if let Some((i, _)) = n.iter().enumerate().find(|(i, &(_, nest))| nest > 4) {
        let ((left, _), (right, _)) = (n.remove(i), n.remove(i));
        if i > 0 {
            n[i - 1].0 += left;
        }
        if i < n.len() {
            n[i].0 += right;
        }
        n.insert(i, (0, 4));
        return reduce(n);
    }
    if let Some((i, &(value, nest))) = n.iter().enumerate().find(|(i, &(value, _))| value > 9) {
        n[i] = ((value + 1) / 2, nest + 1);
        n.insert(i, (value / 2, nest + 1));
        return reduce(n);
    }
    n
}

fn add(mut n1: Vec<(i32, i32)>, mut n2: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    n1.append(&mut n2);
    n1.iter_mut().for_each(|(_, nest)| *nest += 1);
    n1
}

fn magnitude(mut n: Vec<(i32, i32)>) -> i32 {
    for d in (1..=4).rev() {
        let mut i = 0;
        while i < n.len() {
            if n[i].1 == d {
                let value = n.remove(i).0 * 3 + n.remove(i).0 * 2;
                n.insert(i, (value, d - 1));
            }
            i += 1;
        }
    }
    n[0].0
}

fn part1(input: &[Vec<(i32, i32)>]) -> i32 {
    magnitude(
        input
            .iter()
            .skip(1)
            .fold(input[0].clone(), |acc, n| reduce(add(acc, n.clone()))),
    )
}

fn part2(input: &[Vec<(i32, i32)>]) -> i32 {
    let mut best = 0;
    for (i1, n1) in input.iter().enumerate() {
        for (i2, n2) in input.iter().enumerate() {
            if i1 != i2 {
                best = best.max(magnitude(reduce(add(n1.clone(), n2.clone()))));
            }
        }
    }
    best
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part1: {}", part2(&input));
}

// fn get_input() -> Vec<Content> {
//     std::fs::read_to_string("input/input.txt").unwrap().lines().map(|l| get_snailfish(l)).collect()
// }
//
// #[derive(Debug)]
// enum Content {
//     Number(u32),
//     Fish(Box<Content>, Box<Content>)
// }
//
// fn get_snailfish(input: &str) -> Content {
//     let center_split = input.char_indices().skip(1).scan(0,|count,(i,c)| {
//         *count += (c == '[') as i32 - (c == ']') as i32;
//         Some((*count,i,c))
//     } ).find(|&(count, _, c)| count == 0 && c == ',');
//     if let Some((_,loc,_)) = center_split {
//         let (a,b) = input.split_at(loc);
//         Content::Fish(Box::new(get_snailfish(&a[1..])),Box::new(get_snailfish(&b[1..b.len()-1])))
//     } else {
//         Content::Number(input.parse().unwrap())
//     }
// }
