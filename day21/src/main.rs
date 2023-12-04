use itertools::Itertools;

fn get_input() -> (i64, i64) {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|s| s.split(": ").last().unwrap().parse().unwrap())
        .collect_tuple()
        .unwrap()
}

fn part1(&(mut p1, mut p2): &(i64,i64)) -> i64 {
    let (mut p1s, mut p2s) = (0,0);
    let mut throws = 0;
    while p1s < 1000 && p2s < 1000 {
        //improve with "current player"
        if throws%2 == 0 {
            p1 = (p1 + 6 + 3*throws - 1)%10 + 1;
            p1s += p1;
        } else {
            p2 = (p2 + 6 + 3*throws - 1)%10 + 1;
            p2s += p2;
        }
        throws += 3;
    }
    throws * p1s.min(p2s)
}

fn throws() -> impl Iterator<Item = (i64,i64)> {
    [(3,1),(4,3),(5,6),(6,7),(7,6),(8,3),(9,1)].into_iter()
}

fn part2(&(p1,p2): &(i64,i64)) -> i64 {
    let mut map = std::collections::HashMap::from([((p1,p2,0,0),1i64)]);
    let mut turn = 0;
    while map.iter().filter(|((_,_,p1s,p2s),_)| *p1s < 21 && *p2s < 21).map(|(_,v)| *v).sum::<i64>() > 0 {
        let old = map.clone();
        // println!("{:?}", old);
        for (&(p1,p2,p1s,p2s),v) in old.iter().filter(|((_,_,p1s,p2s),_)| *p1s < 21 && *p2s < 21) {
            if turn % 2 == 0 {
                // println!("old: {:?}", (p1,p1s));
                for (p1_new, p1s_new, a) in throws().map(|(t,a)| ((p1+t-1)%10+1, a)).map(|(i,a)| (i, p1s + i, a)) {
                    // println!("new: {:?}", (p1_new,p1s_new));
                    *map.entry((p1_new,p2,p1s_new,p2s)).or_insert(0) += v * a;
                }
            } else {
                for (p2_new, p2s_new, a) in throws().map(|(t,a)| ((p2+t-1)%10+1, a)).map(|(i,a)| (i, p2s + i, a)) {
                    *map.entry((p1,p2_new,p1s,p2s_new)).or_insert(0) += v * a;
                }
            }
        }
        for (&p,v) in old.iter().filter(|((_,_,p1s,p2s),_)| *p1s < 21 && *p2s < 21) {
            *map.entry(p).or_insert(0) -= v;
        }
        map.retain(|_, v| *v != 0);
        turn += 1;
    }
    let p1_wins = map.iter().filter(|((_,_,p1s,_),_)| *p1s >= 21).map(|(_,v)| *v ).sum::<i64>();
    let p2_wins = map.iter().filter(|((_,_,_,p2s),_)| *p2s >= 21).map(|(_,v)| *v ).sum::<i64>();

    p1_wins.max(p2_wins)
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
