#![feature(array_windows)]

use std::collections::HashMap;
use std::str::{from_utf8, from_utf8_unchecked};
use adventofcode2021::prelude::*;

#[inline]
const fn bitpack(a: u8, b: u8) -> usize {
    (a-b'A') as usize | (((b-b'A') as usize) * 32)
}

#[inline]
fn bitunpack(x: usize) -> (u8, u8) {
    let a = x % 32;
    assert!(a < 32, "{}, {}", x, a);
    let b = (x / 32) % 32;
    (a as u8 + b'A', b as u8 + b'A')
}

const BITPACK_MAX: usize = bitpack(b'Z', b'Z');

pub fn main() -> Result<()> {
    let mut lines = include_str!("../../data/a14_example.txt").lines();
    let start = lines.next().ok_or(Error::EmptyIterator)?.as_bytes().to_vec();
    let _empty = lines.next().ok_or(Error::EmptyIterator)?;
    let rules = lines.map(|line| {
        let mut split = line.split(" -> ");
        let from: [u8;2] = split.next().ok_or(Error::EmptyIterator)?.as_bytes().try_into()?;
        let to = split.next().ok_or(Error::EmptyIterator)?.bytes().next().ok_or(Error::EmptyIterator)?;
        Ok((from, to))
    }).collect::<Result<HashMap<_, _>>>()?;

    let mut current = start.clone();
    for i in 1..=10 {
        let mut result = Vec::with_capacity(current.len()*3/2);
        if let Some(first) = current.first() {
            result.push(*first);
        }
        current.array_windows().for_each(|window| {
            // result.push(window[0]);
            if let Some(to_insert) = rules.get(window) {
                result.push(*to_insert)
            }
            result.push(window[1]);
        });

        current = result;
    }

    let mut histogram = [0_usize; 256];
    current.iter().for_each(|b| {
        histogram[*b as usize] += 1;
    });
    let part1 = histogram.iter().max().unwrap_or(&0) - histogram.iter().filter(|c| **c > 0).min().unwrap_or(&0);

    eprintln!("B: {}", histogram[b'B' as usize]);
    eprintln!("C: {}", histogram[b'C' as usize]);
    eprintln!("H: {}", histogram[b'H' as usize]);
    eprintln!("N: {}", histogram[b'N' as usize]);
    println!("Part1: {}", part1);


    let mut pair_histogram = vec![0_usize; BITPACK_MAX+1];

    start.array_windows().for_each(|[a, b]| {
        pair_histogram[bitpack(*a, *b)] += 1;
    });
    let mut next_pair_histogram = pair_histogram.clone();

    for i in 0..10 {
        // next_histogram.clear();
        for (j, count) in pair_histogram.iter().enumerate() {
            if *count > 0 {
                let (a, b) = bitunpack(j);
                if let Some(to_insert) = rules.get(&[a,b]) {
                    dbg!(a as char,b as char,*to_insert as char);
                    next_pair_histogram[bitpack(a, *to_insert)] += *count;
                    next_pair_histogram[bitpack(*to_insert, b)] += *count;
                    next_pair_histogram[bitpack(a, b)] -= *count;
                }
            }
        }

        std::mem::swap(&mut pair_histogram, &mut next_pair_histogram);
    }

    histogram.fill(0);

    for (j, count) in pair_histogram.iter().enumerate() {
        if *count > 0 {
            let (a, b) = bitunpack(j);
            histogram[a as usize] += 1;
            histogram[b as usize] += 1;
        }
    }

    let part2 = histogram.iter().max().unwrap_or(&0) - histogram.iter().filter(|c| **c > 0).min().unwrap_or(&0);

    eprintln!("B: {}", histogram[b'B' as usize]);
    eprintln!("C: {}", histogram[b'C' as usize]);
    eprintln!("H: {}", histogram[b'H' as usize]);
    eprintln!("N: {}", histogram[b'N' as usize]);
    println!("Part2: {}", part2);


    Ok(())
}