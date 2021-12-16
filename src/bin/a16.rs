use std::str::from_utf8;
use adventofcode2021::prelude::*;

#[derive(Debug, Clone, PartialEq)]
enum Packet {
    Number{
        version: u64,
        value: u64,
    },
    Operator{
        version: u64,
        op_type: u64,
        data: Vec<Packet>,
    },
}

fn hex_value(c: u8) -> Result<u8> {
    let hex_value = match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a',
        b'A'..=b'F' => c - b'A',
        _ => {
            return Err(Error::General(format!("Could not parse hex digit: {}", c as char)));
        }
    };
    Ok(hex_value)
}

fn parse_hex(input: &str) -> Vec<bool> {
    input.bytes().map(|b| format!("{:04b}", u8::from_str_radix(from_utf8(&[b]).unwrap(), 16).unwrap()).into_bytes()).flatten().map(|b| b == b'1').collect()
}

fn read_bits(bits: &mut dyn Iterator<Item=bool>, num_bits: usize) -> u64 {
    bits.take(num_bits).fold(0, |a, b| {
        a << 1 | b as u64
    })
}

fn read_num(bits: &mut dyn Iterator<Item=bool>) -> u64 {
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



fn parse_packet(mut bits: &mut dyn Iterator<Item=bool>) -> Option<Packet> {
    let version = read_bits(bits, 3);
    let packet_type = read_bits(bits, 3);

    let packet = match packet_type {
        4 => {
            let value= read_num(bits);
            Packet::Number {
                version,
                value
            }
        }
        _ => {
            let length_type = bits.next();
            dbg!(&length_type);
            let packets = match length_type {
                None => return None,
                Some(false) => {
                    let num_bits = read_bits(&mut bits, 15) as usize;

                    let mut bits = bits.take(num_bits);
                    parse_packets(&mut bits, None)
                },
                Some(true) => {
                    let num_packets = read_bits(&mut bits, 11) as usize;
                    parse_packets(&mut bits, Some(num_packets))
                }
            };

            Packet::Operator {
                version,
                op_type: packet_type,
                data: packets
            }

        }
    };

    Some(packet)
}

fn parse_packets(mut bits: &mut dyn Iterator<Item=bool>, limit: Option<usize>) -> Vec<Packet> {
    let mut packets = vec![];
    loop {
        if let Some(limit) = limit {
            if packets.len() >= limit {
                break;
            }
        }
        if let Some(packet) = parse_packet(bits) {
            packets.push(packet);
        }  else {
            break;
        }
    }
    packets
}

fn version_sum(packets: &[Packet]) -> u64 {
    packets.iter().fold(0_u64, |a, p| {
        a + match p {
            Packet::Number { version, .. } => { *version}
            Packet::Operator { version, data, .. } => {
                *version + version_sum(data)
            }
        }
    })
}

pub fn main() -> Result<()> {
    let input = include_str!("../../data/a16_input.txt");
    let mut bits = parse_hex(input).into_iter();
    let packets = parse_packets(&mut bits, None);

    // dbg!(&packets);
    println!("Part1: {}", version_sum(&packets));

    Ok(())
}
