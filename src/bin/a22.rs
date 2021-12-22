use std::ops::Range;
use regex::Captures;
use adventofcode2021::prelude::*;

#[derive(Debug)]
struct Cube {
    on: bool,
    x: Range<i32>,
    y: Range<i32>,
    z: Range<i32>,
}

fn parse_range(captures: &Captures, index: usize) -> Result<Range<i32>> {
    let start: i32 = captures.get(index).unwrap().as_str().parse()?;
    let end: i32 = captures.get(index+1).unwrap().as_str().parse()?;

    Ok(Range {
        start: start.min(end),
        end: start.max(end)+1, // exclusive
    })
}

pub fn main() -> Result<()> {
    let data = include_str!("../../data/a22_input.txt");

    let pattern = regex!(r"^(on|off) x=(-?[0-9]+)..(-?[0-9]+),y=(-?[0-9]+)..(-?[0-9]+),z=(-?[0-9]+)..(-?[0-9]+)$");

    let cubes = data
        .lines()
        .map(|l| {
            let captures = pattern.captures(l).ok_or(Error::PatternMatch)?;
            Ok(Cube {
                on: captures.get(1).unwrap().as_str().len() == 2,
                x: parse_range(&captures, 2)?,
                y: parse_range(&captures, 4)?,
                z: parse_range(&captures, 6)?,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let cubes_in_range = cubes.iter().filter(|c| {
        [&c.x, &c.y, &c.z].iter().all(|range| range.start >= -50 && range.start <= 51 && range.end >= -50 && range.end <= 51)
    }).collect::<Vec<_>>();

    dbg!(&cubes_in_range.len());

    let offset = 50;

    let mut space = vec![vec![[false; 101]; 101]; 101];

    for cube in cubes_in_range {
        for x in cube.x.clone() {
            for y in cube.y.clone() {
                for z in cube.z.clone() {
                    space[(x+offset) as usize][(y+offset) as usize][(z+offset) as usize] = cube.on;
                }
            }
        }
    }

    let count = space.iter().flatten().flatten().filter(|p| **p).count();

    println!("Part 1: {}", count);


    Ok(())
}