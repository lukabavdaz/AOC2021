use itertools::Itertools;

type Point = (i32, i32, i32);

fn get_input() -> Vec<Vec<Point>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .trim()
        .split("\n\n")
        .flat_map(|s| s.split("\r\n\r\n"))
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|s| {
                    s.split(',')
                        .map(|n| n.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect()
        })
        .collect()
}

fn all_rots(p: Point) -> impl Iterator<Item = Point> {
    [p, rot_y(p), rot_z(p)]
        .into_iter()
        .flat_map(|p| [p, rot_z(rot_z(p))].into_iter())
        .flat_map(|p| [p, rot_x(p), rot_x(rot_x(p)), rot_x(rot_x(rot_x(p)))].into_iter())
}

fn rot_x(p: Point) -> Point {
    (p.0, -p.2, p.1)
}

fn rot_y(p: Point) -> Point {
    (p.2, p.1, -p.0)
}

fn rot_z(p: Point) -> Point {
    (-p.1, p.0, p.2)
}

fn diff(p1: Point, p2: Point) -> Point {
    (p1.0 - p2.0, p1.1 - p2.1, p1.2 - p2.2)
}

fn add(p1: Point, p2: Point) -> Point {
    (p1.0 + p2.0, p1.1 + p2.1, p1.2 + p2.2)
}

fn part1(input: &Vec<Vec<Point>>) -> usize {
    let mut beacons = input.clone();
    let mut to_visit = vec![0];
    let mut distances = vec![(0,0,0);input.len()];
    let mut visited = std::collections::HashSet::new();
    while let Some(i1) = to_visit.pop() {
        if visited.insert(i1) {
            for (i2, scan2) in input.iter().enumerate().filter(|(i2,_)| !visited.contains(i2)) {
                for transform in (0..24).map(|i| move |p| all_rots(p).nth(i).unwrap()) {
                    let mut map = std::collections::HashMap::new();
                    for diff in beacons[i1].iter().cartesian_product(scan2.iter()).map(|(&p1, &p2)| diff(p1, transform(p2))) {
                        *map.entry(diff).or_insert(0) += 1;
                    }
                    if let Some((&d, &v)) = map.iter().max_by_key(|&(_, v)| v) {
                        if v >= 12 {
                            beacons[i2] = scan2.iter().map(|p| add(transform(*p), d)).collect();
                            distances[i2] = d;
                            to_visit.push(i2);
                            break;
                        }
                    }
                }
            }
        }
    }

    let max_distance = distances.iter().tuple_combinations().map(|(p1,p2)| (p1.0-p2.0).abs()+(p1.1-p2.1).abs()+(p1.2-p2.2).abs() ).max().unwrap();

    println!("part2: {}", max_distance);

    let mut unique_beacons = std::collections::HashSet::new();
    for b in beacons.iter().flatten() {
        unique_beacons.insert(b);
    }
    unique_beacons.len()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    // println!("Hello, world!");
}
