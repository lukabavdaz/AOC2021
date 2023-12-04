use std::iter::FromIterator;

fn get_input() -> (Vec<(u32, u32)>, Vec<(char, u32)>) {
    let file = std::fs::read_to_string("input/input.txt").unwrap();
    let dots = file
        .trim()
        .lines()
        .take_while(|s| !s.is_empty())
        .map(|l| l.split_once(',').unwrap())
        .map(|(s1,s2)| (s1.parse().unwrap(), s2.parse().unwrap()))
        .collect();
    let folds = file
        .trim()
        .lines()
        .skip_while(|s| !s.starts_with('f'))
        .map(|s| s.split_once('=').unwrap())
        .map(|(s1,s2)| (s1.chars().last().unwrap(), s2.parse().unwrap()))
        .collect();
    (dots, folds)
}

fn part1((dots, folds): &(Vec<(u32,u32)>, Vec<(char, u32)>)) -> usize {
    let fold = folds.first().unwrap();
    let mut dots_copy = dots.clone();
    // print(&dots_copy);
    // println!();
    if fold.0 == 'x' {
        for dot in dots_copy.iter_mut() {
            (*dot).0 = (*dot).0.min(2*fold.1-(*dot).0);
        }
    } else {
        for dot in dots_copy.iter_mut() {
            (*dot).1 = (*dot).1.min(2*fold.1-(*dot).1);
        }
    }
    // print(&dots_copy);

    let hash = std::collections::HashSet::<(u32,u32)>::from_iter(dots_copy);
    hash.len() as usize
}

fn print(dots: &Vec<(u32,u32)>) {
    let (max_x, max_y) = dots.iter().fold((0,0),|acc, d| (acc.0.max(d.0),acc.1.max(d.1)));
    let hash = std::collections::HashSet::<&(u32,u32)>::from_iter(dots);
    for y in 0..=max_y {
        for x in 0..=max_x {
            if hash.contains(&(x,y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn part2((dots, folds): &(Vec<(u32,u32)>, Vec<(char, u32)>)) {
    let mut dots_copy = dots.clone();
    // print(&dots_copy);
    // println!();
    for fold in folds {
        if fold.0 == 'x' {
            for dot in dots_copy.iter_mut() {
                (*dot).0 = (*dot).0.min(2*fold.1-(*dot).0);
            }
        } else {
            for dot in dots_copy.iter_mut() {
                (*dot).1 = (*dot).1.min(2*fold.1-(*dot).1);
            }
        }
    }

    print(&dots_copy);
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    part2(&input);
}