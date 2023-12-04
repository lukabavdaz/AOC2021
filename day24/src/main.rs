fn get_input() -> Vec<Vec<String>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.split(" ").map(|s| s.into()).collect())
        .collect()
}

fn get_binding<T>(s: &str, w: T, x: T, y: T, z: T) -> T {
    match s {
        "w" => w,
        "x" => x,
        "y" => y,
        "z" => z,
        _ => panic!(),
    }
}

fn fn_chunk(input: &[Vec<String>], mut w: i64, mut z: i64) -> i64 {
    let (mut x, mut y) = (0, 0);
    for command in input.iter().skip(1) {

        let param1 = command.get(1).unwrap().as_str();
        let param2 = command.get(2).unwrap().as_str();
        let b = param2.parse().unwrap_or_else(|_| get_binding(param2, w, x, y, z));
        let a = get_binding(param1, &mut w, &mut x, &mut y, &mut z);

        // let b = match command.get(2).unwrap().as_str() {
        //     "w" => w,
        //     "x" => x,
        //     "y" => y,
        //     "z" => z,
        //     b => b.parse().unwrap(),
        // };

        // let a = match command.get(1).unwrap().as_str() {
        //     "w" => &mut w,
        //     "x" => &mut x,
        //     "y" => &mut y,
        //     "z" => &mut z,
        //     _ => panic!(),
        // };

        match command[0].as_str() {
            "add" => *a += b,
            "mul" => *a *= b,
            "div" => *a /= b,
            "mod" => *a %= b,
            "eql" => *a = (*a == b) as i64,
            _ => panic!(),
        }
    }

    z
}

fn part1(input: &Vec<Vec<String>>) -> i64 {
    let c = input.iter().skip(5).step_by(18).map(|v| v[2].parse::<i64>().unwrap()).collect::<Vec<_>>();
    let mut z_stack = vec![0;15];
    let mut digits = vec![0;14];
    let mut digit_options = vec![vec![];14];
    let parts = input.chunks(18).collect::<Vec<_>>();

    let mut i = 0;
    while i < 14 {
        if digits[i] == 0 {
            if c[i] > 0 {
                digit_options[i] = (1..=9).collect();
            } else {
                digit_options[i] = (1..=9).filter(|&w| w == (z_stack[i]%26 + c[i])).collect();
            }
        }
        if let Some(digit) = digit_options[i].pop() {
            z_stack[i+1] = fn_chunk(parts[i], digit, z_stack[i]);
            digits[i] = digit;
            i += 1;
        } else {
            digits[i] = 0;
            i -= 1;
        }
    }

    println!("digits: {:?}", digits);
    digits.iter().fold(0,|acc,i| acc*10+i )
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
}
