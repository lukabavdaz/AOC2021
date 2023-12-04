fn get_input() -> Vec<(bool, Vec<i64>)> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|s| s.split_once(" ").unwrap())
        .map(|(s1, s2)| {
            (
                s1 == "on",
                s2.split(|c| !char::is_numeric(c) && c != '-')
                    .filter_map(|s| s.parse().ok())
                    .collect(),
            )
        })
        .collect()
}

fn remove(on: &Vec<i64>, off: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut out_vec = vec![];
    for volume in 0..6 {
        let mut new_volume = vec![];
        for i in 0..3 {
            match (volume / 2, volume % 2) {
                (x, _) if x < i => new_volume.extend([on[i * 2], on[i * 2 + 1]]),
                (x, _) if x > i => new_volume
                    .extend([off[i * 2].max(on[i * 2]), off[i * 2 + 1].min(on[i * 2 + 1])]),
                (_, 1) => new_volume.extend([on[i * 2], on[i * 2 + 1].min(off[i * 2] - 1)]),
                _ => new_volume.extend([on[i * 2].max(off[i * 2 + 1] + 1), on[i * 2 + 1]]),
            }
        }
        if new_volume.chunks(2).all(|c| c[0] <= c[1]) {
            out_vec.push(new_volume);
        }
    }
    out_vec
}

fn get_on<'a>(input: impl IntoIterator<Item = &'a (bool, Vec<i64>)>) -> i64 {
    let mut on_cuboids = vec![];
    for (on, new_cuboid) in input {
        // println!("len: {}", on_cuboids.len());
        let old_cuboids = on_cuboids;
        on_cuboids = vec![];
        for old_cuboid in old_cuboids.iter() {
            on_cuboids.extend(remove(old_cuboid, new_cuboid));
        }
        if *on {
            on_cuboids.push(new_cuboid.clone());
        }
    }
    on_cuboids
        .iter()
        .map(|c| c.chunks(2).map(|a| a[1] - a[0] + 1).product::<i64>())
        .sum::<i64>()
}

fn part1(input: &[(bool, Vec<i64>)]) -> i64 {
    get_on(
        input
            .iter()
            .filter(|(_, v)| *v.iter().max().unwrap() <= 50 && *v.iter().min().unwrap() >= -50),
    )
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", get_on(&input));
}

// fn get_overlap(a: &[i64], b: &[i64]) -> Option<Vec<i64>> {
//     let mut out = vec![];
//     for (a, b) in a.chunks(2).zip(b.chunks(2)) {
//         if a[0] == a[1] && b[1] >= a[0] && b[0] <= a[0] {
//             out.extend_from_slice(&[a[0], a[1]]);
//             continue;
//         }
//         if b[0] == b[1] && a[1] >= b[0] && a[0] <= b[0] {
//             out.extend_from_slice(&[b[0], b[1]]);
//             continue;
//         }
//         let start = a[0].max(b[0]);
//         let end = a[1].min(b[1]);
//         if end - start < 0 {
//             return None;
//         }
//         out.extend_from_slice(&[start, end]);
//     }
//     Some(out)
// }

// fn get_overlap(a: &[i32], b: &[i32]) -> Option<Vec<i32>> {
//     let mut out = vec![];
//     for (a, b) in a.chunks(2).zip(b.chunks(2)) {
//         let start = a[0].max(b[0]);
//         let end = a[1].min(b[1]);
//         if end - start <= 0 {
//             return None;
//         }
//         out.extend_from_slice(&[start, end]);
//     }
//     Some(out)
// }

// let mut overlap_lists = vec![vec![];2];
// for (on, loc) in input {
// println!("checking new input:");
// if *on {
// overlap_lists[0].push(loc.clone());
// }
// let mut nesting = *on as usize;
// loop {
// let overlaps = overlap_lists[nesting].iter().filter_map(|v| get_overlap(v,loc)).collect::<Vec<_>>();
// if overlaps.len() == 0 {
// break;
// }
// if overlap_lists.len() <= nesting + 2 {
// overlap_lists.extend([vec![],vec![]]);
// }
// overlap_lists[nesting+1].extend(overlaps);
// nesting += 2;
// }
// }
