use adventofcode2021::prelude::*;


pub fn main() -> Result<()> {
    let map = include_str!("../../data/a15_input.txt")
        .lines()
        .map(|line| line.bytes().map(|b| b- b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut cumulative_risk = map.iter().map(|row| vec![0_usize; row.len()]).collect::<Vec<Vec<usize>>>();

    for (y, row) in map.iter().enumerate() {
        for (x, value) in row.iter().copied().enumerate() {
            let value = value as usize;
            // let current = get_nested_relative(&cumulative_risk, y, x, 0, 0).unwrap_or(&0_usize);
            let left = get_nested_relative(&cumulative_risk, y, x, 0, -1);
            let up = get_nested_relative(&cumulative_risk, y, x, -1, 0);
            let sum = match (left, up) {
                (Some(left), Some(up)) => value + left.min(up),
                (Some(left), None) => value + left,
                (None, Some(up)) => value + up,
                (None, None) => value,
            };

            cumulative_risk[y][x] = sum
        }
    }

    for (y, row) in cumulative_risk.iter().enumerate() {
        for (x, value) in row.iter().copied().enumerate() {
            eprint!("{:4}", value)
        }
        eprintln!();
    }
    eprintln!();

    println!("Part 1: {}", cumulative_risk.last().map(|row| row.last()).flatten().unwrap() - cumulative_risk[0][0]);


    Ok(())
}