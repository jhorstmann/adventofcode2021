use adventofcode2021::prelude::*;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn solve(lines: &[(Point, Point)], width: usize, height: usize, only_straight: bool, print_map: bool) -> usize {
    let mut map = vec![vec![0_usize; width as usize]; height as usize];

    for l in lines.iter() {
        let dx = (l.1.x - l.0.x).signum();
        let dy = (l.1.y - l.0.y).signum();

        if !only_straight || (dx == 0 || dy == 0) {
            let mut x = l.0.x;
            let mut y = l.0.y;
            loop {
                // dbg!(dx, dy, x, y);
                map[x as usize][y as usize] += 1;
                if x == l.1.x && y == l.1.y {
                    break;
                }
                x += dx;
                y += dy;
                if x < 0 || y < 0 || x as usize == width || y as usize == height {
                    break;
                }
            }
        }
    }

    if print_map {
        for row in map.iter() {
            for col in row.iter() {
                if *col == 0 {
                    print!(".")
                } else {
                    print!("{}", *col)
                }
            }
            println!();
        }
    }

    map.iter().flat_map(|row| row.iter()).filter(|c| **c >= 2).count()
}

pub fn main() -> Result<()> {
    let data = include_str!("../../data/a5_input.txt");
    let lines = data
        .lines()
        .map(|l| {
            let mut split = l.split_ascii_whitespace();
            let start = split.next().ok_or(Error::EmptyIterator)?;
            let start = {
                let (x, y) = start.split_once(",").ok_or(Error::General("Invalid point".into()))?;
                Point {
                    x: x.parse()?,
                    y: y.parse()?,
                }
            };
            let _ = split.next().ok_or(Error::EmptyIterator);
            let end = split.next().ok_or(Error::EmptyIterator)?;
            let end = {
                let (x, y) = end.split_once(",").ok_or(Error::General("Invalid point".into()))?;
                Point {
                    x: x.parse()?,
                    y: y.parse()?,
                }
            };
            if end.x < start.x {
                Ok((end, start))
            } else {
                Ok((start, end))
            }
        })
        .collect::<Result<Vec<_>>>()?;

    let width = lines
        .iter()
        .flat_map(|l| [l.0.x, l.1.x])
        .max()
        .ok_or(Error::General("Max width".into()))?
        + 1;
    let height = lines
        .iter()
        .flat_map(|l| [l.0.y, l.1.y])
        .max()
        .ok_or(Error::General("Max height".into()))?
        + 1;

    dbg!(width, height);

    println!(
        "Part1: {}",
        solve(&lines, width as usize, height as usize, true, false)
    );
    println!(
        "Part2: {}",
        solve(&lines, width as usize, height as usize, false, false)
    );

    Ok(())
}
