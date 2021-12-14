#![feature(array_windows)]

use std::collections::HashMap;
use std::str::{from_utf8, from_utf8_unchecked};
use adventofcode2021::prelude::*;

pub fn main() -> Result<()> {
    let mut lines = include_str!("../../data/a14_example.txt").lines();
    let mut start = lines.next().ok_or(Error::EmptyIterator)?.as_bytes().to_vec();
    let _empty = lines.next().ok_or(Error::EmptyIterator)?;
    let rules = lines.map(|line| {
        let mut split = line.split(" -> ");
        let from: [u8;2] = split.next().ok_or(Error::EmptyIterator)?.as_bytes().try_into()?;
        let to = split.next().ok_or(Error::EmptyIterator)?.bytes().next().ok_or(Error::EmptyIterator)?;
        Ok((from, to))
    }).collect::<Result<HashMap<_, _>>>()?;

    for i in 1..=10 {
        let mut result = Vec::with_capacity(start.len()*3/2);
        if let Some(first) = start.first() {
            result.push(*first);
        }
        start.array_windows().for_each(|window| {
            // result.push(window[0]);
            if let Some(to_insert) = rules.get(window) {
                result.push(*to_insert)
            }
            result.push(window[1]);
        });

        start = result;
    }

    let mut histogram = [0_usize; 256];
    start.into_iter().for_each(|b| {
        histogram[b as usize] += 1;
    });
    let part1 = histogram.iter().max().unwrap_or(&0) - histogram.iter().filter(|c| **c > 0).min().unwrap_or(&0);

    eprintln!("B: {}", histogram[b'B' as usize]);
    eprintln!("C: {}", histogram[b'C' as usize]);
    eprintln!("H: {}", histogram[b'H' as usize]);
    eprintln!("N: {}", histogram[b'N' as usize]);
    println!("Part1: {}", part1);

    Ok(())
}