mod packet;

use self::packet::Packet;

#[aoc_generator(day13, part1)]
fn generate_pairs(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n")
        .map(|pair| {
            let mut iter = pair.split("\n").map(|packet| Packet::parse(packet));
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect()
}

#[aoc(day13, part1)]
fn right_order(pairs: &Vec<(Packet, Packet)>) -> usize {
    pairs
        .iter()
        .enumerate()
        .fold(0, |sum, (i, (a, b))| sum + if a <= b { i + 1 } else { 0 })
}

#[aoc(day13, part2)]
fn decoder_key(input: &str) -> usize {
    let mut packets: Vec<Packet> = input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|packet| Packet::parse(packet))
        .collect();

    packets.push(Packet::parse("[[2]]"));
    packets.push(Packet::parse("[[6]]"));
    packets.sort();

    let start = Packet::parse("[[2]]");
    let end = Packet::parse("[[6]]");

    let start = packets
        .iter()
        .enumerate()
        .find_map(|(i, packet)| if *packet == start { Some(i + 1) } else { None })
        .unwrap();

    let end = packets
        .iter()
        .enumerate()
        .find_map(|(i, packet)| if *packet == end { Some(i + 1) } else { None })
        .unwrap();

    start * end
}
