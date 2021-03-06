use adventofcode2021::prelude::*;

#[derive(Debug, Clone, PartialEq)]
enum Packet {
    Number {
        version: u64,
        value: u64,
    },
    Operator {
        version: u64,
        op_type: u64,
        data: Vec<Packet>,
    },
}

fn parse_hex(input: &str) -> Vec<bool> {
    input
        .bytes()
        .flat_map(|b| {
            let digit = (b as char).to_digit(16).expect("hex digit");
            (0..4).rev().map(move |i| (digit & (1 << i)) != 0)
        })
        .collect()
}

fn read_bits(bits: &mut dyn Iterator<Item = bool>, num_bits: usize) -> u64 {
    bits.take(num_bits).fold(0, |a, b| a << 1 | b as u64)
}

fn read_num(bits: &mut dyn Iterator<Item = bool>) -> u64 {
    let mut num = 0;
    loop {
        if let Some(cont) = bits.next() {
            num = num << 4 | read_bits(bits, 4);

            if !cont {
                break;
            }
        } else {
            break;
        }
    }

    num
}

fn parse_packet(mut bits: &mut dyn Iterator<Item = bool>) -> Option<Packet> {
    let version = read_bits(bits, 3);
    let packet_type = read_bits(bits, 3);

    let packet = match packet_type {
        4 => {
            let value = read_num(bits);
            Packet::Number { version, value }
        }
        _ => {
            let length_type = bits.next();
            let packets = match length_type {
                None => return None,
                Some(false) => {
                    let num_bits = read_bits(&mut bits, 15) as usize;

                    let mut bits = bits.take(num_bits);
                    parse_packets(&mut bits, None)
                }
                Some(true) => {
                    let num_packets = read_bits(&mut bits, 11) as usize;
                    parse_packets(&mut bits, Some(num_packets))
                }
            };

            Packet::Operator {
                version,
                op_type: packet_type,
                data: packets,
            }
        }
    };

    Some(packet)
}

fn parse_packets(bits: &mut dyn Iterator<Item = bool>, limit: Option<usize>) -> Vec<Packet> {
    let mut packets = vec![];
    loop {
        if let Some(limit) = limit {
            if packets.len() >= limit {
                break;
            }
        }
        if let Some(packet) = parse_packet(bits) {
            packets.push(packet);
        } else {
            break;
        }
    }
    packets
}

fn version_sum(packets: &[Packet]) -> u64 {
    packets.iter().fold(0_u64, |a, p| {
        a + match p {
            Packet::Number { version, .. } => *version,
            Packet::Operator { version, data, .. } => *version + version_sum(data),
        }
    })
}

fn evaluate(packet: &Packet) -> u64 {
    match packet {
        Packet::Number { value, .. } => *value,
        Packet::Operator { op_type, data, .. } => match op_type {
            0 => data.iter().map(evaluate).sum(),
            1 => data.iter().map(evaluate).product(),
            2 => data.iter().map(evaluate).min().unwrap(),
            3 => data.iter().map(evaluate).max().unwrap(),
            5 => (evaluate(&data[0]) > evaluate(&data[1])) as u64,
            6 => (evaluate(&data[0]) < evaluate(&data[1])) as u64,
            7 => (evaluate(&data[0]) == evaluate(&data[1])) as u64,
            _ => {
                panic!("Unsupported operator {}", op_type);
            }
        },
    }
}

pub fn main() -> Result<()> {
    let input = include_str!("../../data/a16_input.txt");
    let mut bits = parse_hex(input).into_iter();
    let packets = parse_packets(&mut bits, None);

    println!("Part1: {}", version_sum(&packets));
    for packet in packets.iter() {
        println!("Part2: {}", evaluate(&packet));
    }

    Ok(())
}
