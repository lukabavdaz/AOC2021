fn get_input() -> String {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .collect()
}

enum Packet {
    Literary(u64,u64,u64),
    Complex(u64,u64,Vec<Packet>)
}

fn get_version_number(packet: &Packet) -> u64 {
    match packet {
        Packet::Literary(v,_,_) => *v,
        Packet::Complex(v,_,ps) => *v + ps.iter().map(get_version_number).sum::<u64>()
    }
}

fn get_value(packet: &Packet) -> u64 {
    match packet {
        Packet::Literary(_,_,v) => *v,
        Packet::Complex(_,0,ps) => ps.iter().map(get_value).sum::<u64>(),
        Packet::Complex(_,1,ps) => ps.iter().map(get_value).product::<u64>(),
        Packet::Complex(_,2,ps) => ps.iter().map(get_value).min().unwrap(),
        Packet::Complex(_,3,ps) => ps.iter().map(get_value).max().unwrap(),
        Packet::Complex(_,5,ps) => (get_value(&ps[0]) > get_value(&ps[1])) as u64,
        Packet::Complex(_,6,ps) => (get_value(&ps[0]) < get_value(&ps[1])) as u64,
        Packet::Complex(_,7,ps) => (get_value(&ps[0]) == get_value(&ps[1])) as u64,
        _ => panic!()
    }
}

fn lit_packet(version: u64, id: u64, input: &str) -> (Packet, &str) {
    let (stop,_) = input.chars().step_by(5).enumerate().find(|(_,c)| c == &'0').unwrap();
    let result = input.chars().take((stop+1)*5).enumerate().filter(|&(i, _)| i%5 != 0).fold(0,|acc, (_,i)| acc*2+i.to_digit(2).unwrap() as u64);
    (Packet::Literary(version, id, result), &input[((stop+1)*5)..])
}

fn complex_length(version: u64, id: u64, input: &str) -> (Packet, &str) {
    let length = u32::from_str_radix(&input[0..15],2).unwrap();
    let mut rest = &input[15..(15+length as usize)];
    let mut packets = vec![];
    while rest.len() > 0 {
        let (new_packet, new_rest) = analyze_packet(rest);
        packets.push(new_packet);
        rest = new_rest;
    }
    (Packet::Complex(version, id, packets), &input[(15+length as usize)..])
}

fn complex_number(version: u64, id: u64, input: &str) -> (Packet, &str) {
    let number = u32::from_str_radix(&input[0..11],2).unwrap();
    let mut rest = &input[11..];
    let mut packets = vec![];
    for _ in 0..number {
        let (new_packet, new_rest) = analyze_packet(rest);
        packets.push(new_packet);
        rest = new_rest;
    }
    (Packet::Complex(version, id, packets), rest)
}

fn analyze_packet(input: &str) -> (Packet,&str) {
    let version = u64::from_str_radix(&input[0..3],2).unwrap();
    let id = u64::from_str_radix(&input[3..6],2).unwrap();
    match id {
        4 => lit_packet(version, id, &input[6..]),
        _ if &input[6..7] == "0" => complex_length(version, id, &input[7..]),
        _ => complex_number(version, id, &input[7..]),
    }
}

fn part1(input: &str) -> u64 {
    let (packet, _) = analyze_packet(input);
    get_version_number(&packet)
}

fn part2(input: &str) -> u64 {
    let (packet, _) = analyze_packet(input);
    get_value(&packet)
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
