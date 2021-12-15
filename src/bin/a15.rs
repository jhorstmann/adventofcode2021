use std::cmp::Reverse;
use std::collections::BinaryHeap;
use adventofcode2021::prelude::*;

fn part1(map: &[Vec<u8>]) -> usize {
    let mut cumulative_risk = map.iter().map(|row| vec![0_usize; row.len()]).collect::<Vec<Vec<usize>>>();

    for (y, row) in map.iter().enumerate() {
        for (x, value) in row.iter().copied().enumerate() {
            let value = value as usize;
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

    let last = cumulative_risk.last().map(|row| row.last()).flatten().unwrap();
    let first = cumulative_risk[0][0];

    last - first
}

fn part2<'a>(map: &[Vec<u8>]) -> usize {
    let mut min_sum = map.iter().map(|row| vec![usize::MAX; row.len()]).collect::<Vec<_>>();

    let mut q = BinaryHeap::new();
    q.push((Reverse(0), 0, 0));
    while let Some((Reverse(sum), x, y)) = q.pop() {
        let sum = sum + map[y][x] as usize;
        if sum < min_sum[y][x] {
            min_sum[y][x] = sum;
            if y > 0  {
                q.push((Reverse(sum), x, y-1));
            }
            if x > 0  {
                q.push((Reverse(sum), x-1, y));
            }
            if y < map.len()-1  {
                q.push((Reverse(sum), x, y+1));
            }
            if x < map[y].len()-1  {
                q.push((Reverse(sum), x+1, y));
            }
        }
    }

    let last = min_sum.last().map(|row| row.last()).flatten().unwrap();
    let first = min_sum[0][0];

    last - first
}

pub fn main() -> Result<()> {
    let map = include_str!("../../data/a15_input.txt")
        .lines()
        .map(|line| line.bytes().map(|b| b- b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&map));
    println!("Part 1: {}", part2(&map));

    // let map = vec![vec![9_usize]];

    let mut bigmap: Vec<Vec<u8>> = Vec::with_capacity(map.len()*5);
    for i1 in 0..5 {
        for (_y, row) in map.iter().enumerate() {
            let mut bigrow = Vec::with_capacity(row.len()*5);
            for i2 in 0..5 {
                for (_x, value) in row.iter().copied().enumerate() {
                    let new_value = value as usize + i1 + i2;
                    let new_value = if new_value > 9 {
                        new_value - 9
                    } else {
                        new_value
                    };
                    bigrow.push(new_value as u8);
                }
            }
            bigmap.push(bigrow);
        }
    }
/*
    for y in 0..bigmap.len() {
        for x in 0..bigmap[y].len() {
            let value = bigmap[y][x];
            eprint!("{:2}", value)
        }
        eprintln!();
    }
    eprintln!();
*/
    // 2970 to high

    println!("Part 2: {}", part1(&bigmap));
    println!("Part 2: {}", part2(&bigmap));

    Ok(())
}