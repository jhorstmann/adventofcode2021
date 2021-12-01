#![feature(array_windows)]

use adventofcode2021::prelude::*;

fn count_increasing(lines: &[i64]) -> usize {
    let count_increasing = lines.array_windows().filter(|[a, b]| a < b).count();
    count_increasing
}

pub fn main() -> Result<()> {
    let lines = read_lines("data/a1_input.txt")?;
    let lines = lines
        .into_iter()
        .map(|l| Ok(i64::from_str(&l)?))
        .collect::<Result<Vec<i64>>>()?;

    println!("{}", count_increasing(&lines));

    let sums = lines
        .iter()
        .zip(lines.iter().skip(1).zip(lines.iter().skip(2)))
        .map(|(a, (b, c))| a + b + c)
        .collect::<Vec<_>>();

    println!("{}", count_increasing(&sums));

    Ok(())
}
