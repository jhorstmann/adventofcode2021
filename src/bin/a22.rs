#![feature(int_abs_diff)]

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::Range;
use regex::Captures;
use adventofcode2021::prelude::*;

#[derive(Debug, Clone)]
struct Cube {
    on: bool,
    x: Range<i32>,
    y: Range<i32>,
    z: Range<i32>,
}

fn parse_range(captures: &Captures, index: usize) -> Result<Range<i32>> {
    let start: i32 = captures.get(index).unwrap().as_str().parse()?;
    let end: i32 = captures.get(index + 1).unwrap().as_str().parse()?;

    Ok(Range {
        start: start.min(end),
        end: start.max(end) + 1, // exclusive
    })
}

fn solve_part1(cubes: &[Cube]) -> usize {
    let mut space = vec![vec![[false; 101]; 101]; 101];

    for cube in cubes {
        if [cube.x.start as i32, cube.x.end as i32, cube.y.start, cube.y.end, cube.z.start as i32, cube.z.end as i32].iter()
            .all(|r| *r >= -50 && *r <= 51 ) {
            for x in cube.x.clone() {
                for y in cube.y.clone() {
                    for z in cube.z.clone() {
                        space[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = cube.on;
                    }
                }
            }
        }
    }

    space.iter().flatten().flatten().filter(|p| **p).count()
}

fn solve_part2(cubes: &[Cube]) -> usize {
    let mut xs = cubes.iter().flat_map(|c| [c.x.start, c.x.end]).collect::<Vec<_>>();
    let mut ys = cubes.iter().flat_map(|c| [c.y.start, c.y.end]).collect::<Vec<_>>();
    let mut zs = cubes.iter().flat_map(|c| [c.z.start, c.z.end]).collect::<Vec<_>>();

    xs.sort();
    ys.sort();
    zs.sort();
    xs.dedup();
    ys.dedup();
    zs.dedup();

    dbg!(xs.len(), ys.len(), zs.len());
    dbg!(xs.first(), xs.last(), ys.first(), ys.last(), zs.first(), zs.last());

    let mut cubes = cubes.to_vec();
    let mut split = Vec::with_capacity(cubes.len());

    for mut cube in cubes.iter().cloned() {
        for x in xs.iter().copied() {
            if cube.x.start < x && cube.x.end > x {
                let mut a = cube.clone();
                a.x.end = x;
                cube.x.start = x;
                split.push(a)
            }
        }
        if !cube.x.is_empty() {
            split.push(cube)
        }
    }
    dbg!(split.len());
    cubes = std::mem::take(&mut split);

    for mut cube in cubes.iter().cloned() {
        for y in ys.iter().copied() {
            if cube.y.start < y && cube.y.end > y {
                let mut a = cube.clone();
                a.y.end = y;
                cube.y.start = y;
                split.push(a)
            }
        }
        if !cube.y.is_empty() {
            split.push(cube)
        }
    }
    dbg!(split.len());
    cubes = std::mem::take(&mut split);

    for mut cube in cubes.iter().cloned() {
        for z in zs.iter().copied() {
            if cube.z.start < z && cube.z.end > z {
                let mut a = cube.clone();
                a.z.end = z;
                cube.z.start = z;
                split.push(a)
            }
        }
        if !cube.z.is_empty() {
            split.push(cube)
        }
    }
    dbg!(split.len());
    cubes = std::mem::take(&mut split);

    let mut unique: HashMap<[i32; 6], bool> = HashMap::with_capacity(4096*1024);

    for cube in cubes.iter() {
        let mut key = [cube.x.start, cube.x.end, cube.y.start, cube.y.end, cube.z.start, cube.z.end];
        match unique.entry(key) {
            Entry::Occupied(occ) => {
                if cube.on {
                    *occ.into_mut() = true;
                } else {
                    occ.remove();
                }
            }
            Entry::Vacant(vac) => {
                if cube.on {
                    vac.insert(true);
                }
            }
        }
    }
    dbg!(unique.len());

    unique.into_iter().filter(|(_key, on)| *on).map(|([x1, x2, y1, y2, z1, z2], _)| {
        let x = x1.abs_diff(x2) as usize;
        let y = y1.abs_diff(y2) as usize;
        let z = z1.abs_diff(z2) as usize;
        x * y * z
    }).sum()
}

pub fn main() -> Result<()> {
    let data = include_str!("../../data/a22_example.txt");

    let pattern = regex!(r"^(on|off) x=(-?[0-9]+)\.\.(-?[0-9]+),y=(-?[0-9]+)\.\.(-?[0-9]+),z=(-?[0-9]+)\.\.(-?[0-9]+)$");

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

    println!("Part 1: {}", solve_part1(&cubes));
    dbg!(std::mem::size_of::<Cube>());
    println!("Part 2: {}", solve_part2(&cubes));

    Ok(())
}
