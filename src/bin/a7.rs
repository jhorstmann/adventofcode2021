#![feature(int_abs_diff)]

use adventofcode2021::prelude::*;

pub fn main() -> Result<()> {
    let positions = include_str!("../../data/a7_input.txt")
        .split(",")
        .map(|n| Ok(i32::from_str(n)?))
        .collect::<Result<Vec<_>>>()?;

    let min = *positions.iter().min().ok_or(Error::EmptyIterator)?;
    let max = *positions.iter().max().ok_or(Error::EmptyIterator)?;

    let min_fuel_part1 = (min..max + 1)
        .map(|pos| positions.iter().map(|n| n.abs_diff(pos) as u64).sum::<u64>())
        .min()
        .ok_or(Error::EmptyIterator)?;

    println!("{}", min_fuel_part1);

    let min_fuel_part2 = (min..max + 1)
        .map(|pos| {
            positions
                .iter()
                .map(|n| {
                    let diff = n.abs_diff(pos) as u64;

                    diff * (diff + 1) / 2
                })
                .sum::<u64>()
        })
        .min()
        .ok_or(Error::EmptyIterator)?;

    println!("{}", min_fuel_part2);

    Ok(())
}
