#[derive(Clone)]
struct State {
    side_rooms: Vec<Vec<u8>>,
    hallway: Vec<Option<u8>>,
    cost: u32,
}

fn solution(state: &State) -> bool {
    if state.hallway.iter().any(|h|h.is_some()) {
        return false;
    }
    for (i, v) in state.side_rooms.iter().enumerate() {
        if v.iter().any(|&a| a != i as u8) {
            return false;
        }
    }
    true
}

fn get_steps(side_room: &Vec<u8>, side_room_i: usize, hallway_i: usize) -> u32 {
    let side_room_steps = 4 - side_room.len(); //note this constant is hardcoded! Should be based on desired # in cave
    let hallway_steps = if hallway_i >= side_room_i+2 {
        2 * (hallway_i - side_room_i - 2) + 1
    } else {
        2 * (side_room_i + 2 - hallway_i) - 1
    };
    let hallway_fix = (hallway_i == 0 || hallway_i == 6) as u32;
    let steps = (side_room_steps + hallway_steps) as u32 - hallway_fix;
    // println!("sr: {:?} sr_i: {}, h_i: {}, steps: {}", side_room,side_room_i,hallway_i, steps);
    steps
}

fn hallway_ends(i: usize, hallway: &Vec<Option<u8>>) -> (Option<usize>,Option<usize>) {
    let left = hallway.iter().take(i+2).rposition(|v| v.is_some());
    let right = hallway.iter().skip(i+2).position(|v| v.is_some());
    (left, right.map(|p| p+i+2))
}

fn get_insert_state(old_state: State, i: usize, hallway_i: usize) -> State {
    let mut new_state = old_state.clone();
    new_state.cost += 10u32.pow(i as u32)*get_steps(&old_state.side_rooms[i], i, hallway_i);
    new_state.hallway[hallway_i] = None;
    new_state.side_rooms[i].push(i as u8);
    new_state
}

fn get_options(old_state: State) -> Vec<State> {
    let mut options = vec![];
    for (i,cave) in old_state.side_rooms.iter().enumerate() {
        let (left, right) = hallway_ends(i, &old_state.hallway);
        if cave.iter().all(|&v| i == v as usize ) {
            if let Some(l) = left {
                if Some(i as u8) == old_state.hallway[l] {
                    return vec![get_insert_state(old_state, i, l)];
                }
            }
            if let Some(r) = right {
                if Some(i as u8) == old_state.hallway[r] {
                    return vec![get_insert_state(old_state, i, r)];
                }
            }
        } else {
            if let Some(&last) = cave.last() {
                for x in left.map_or(0,|l| l+1)..=right.map_or(6,|r| r-1) {
                    let mut new_state = old_state.clone();
                    new_state.side_rooms[i].pop();
                    new_state.cost += 10u32.pow(last as u32)*get_steps(&new_state.side_rooms[i], i, x);
                    new_state.hallway[x] = Some(last);
                    options.push(new_state);
                }
            }
        }

    }
    options
}

fn solve(old_state: State, mut results: &mut Vec<u32>) {
    // println!("state: {:?}", old_state.side_rooms);
    for new_state in get_options(old_state) {
        if solution(&new_state) {
            // println!("cost: {}", new_state.cost);
            results.push(new_state.cost);
        } else {
            solve(new_state, &mut results);
        }
    }
}

fn part1() -> u32 {
    let side_rooms = vec![vec![2,1],vec![3,2],vec![3,0],vec![0,1]];
    let hallway = vec![None;7];
    let mut results = vec![];
    solve(State{side_rooms, hallway, cost: 0}, &mut results);
    *results.iter().min().unwrap()
}

fn part2() -> u32 {
    let side_rooms = vec![vec![2,3,3,1],vec![3,1,2,2],vec![3,0,1,0],vec![0,2,0,1]];
    let hallway = vec![None;7];
    let mut results = vec![];
    solve(State{side_rooms, hallway, cost: 0}, &mut results);
    *results.iter().min().unwrap()
}

fn main() {
    // println!("part1: {}", part1());
    println!("part2: {}", part2());
}
