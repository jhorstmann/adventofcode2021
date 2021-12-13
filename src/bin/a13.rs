use adventofcode2021::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Split {
    X(i64),
    Y(i64),
}

pub fn main() -> Result<()> {
    let lines = include_str!("../../data/a13_input.txt").lines().collect::<Vec<_>>();
    let mut iter = lines.split(|line| line.is_empty());
    let lines = iter.next().ok_or(Error::EmptyIterator)?;
    let splits = iter.next().ok_or(Error::EmptyIterator)?;

    let mut points = lines
        .iter()
        .map(|line| {
            let mut split = line.split(",");
            let x = split.next().ok_or(Error::EmptyIterator)?.parse::<i64>()?;
            let y = split.next().ok_or(Error::EmptyIterator)?.parse::<i64>()?;

            Ok((x, y))
        })
        .collect::<Result<Vec<(i64, i64)>>>()?;

    let splits = splits
        .iter()
        .map(|line| {
            let mut split = line.split("=");
            let at = split.next().ok_or(Error::EmptyIterator)?;
            let d = split.next().ok_or(Error::EmptyIterator)?.parse::<i64>()?;
            if at.ends_with("x") {
                Ok(Split::X(d))
            } else if at.ends_with("y") {
                Ok(Split::Y(d))
            } else {
                Err(Error::General(format!("Could not split: {}", line)))
            }
        })
        .collect::<Result<Vec<_>>>()?;

    for (i, split) in splits.iter().copied().enumerate() {
        match split {
            Split::X(d) => {
                points.iter_mut().for_each(|(x, _y)| {
                    if *x > d {
                        *x = d - (*x - d).abs()
                    }
                });
            }
            Split::Y(d) => {
                points.iter_mut().for_each(|(_x, y)| {
                    if *y > d {
                        *y = d - (*y - d).abs()
                    }
                });
            }
        }
        points.sort();
        points.dedup();
        println!("After split {}: {}", i + 1, points.len());
    }

    let min_x = points.iter().map(|p| p.0).min().ok_or(Error::EmptyIterator)?;
    let min_y = points.iter().map(|p| p.1).min().ok_or(Error::EmptyIterator)?;
    let max_x = points.iter().map(|p| p.0).max().ok_or(Error::EmptyIterator)?;
    let max_y = points.iter().map(|p| p.1).max().ok_or(Error::EmptyIterator)?;

    if [min_x, min_y, max_x, max_y].iter().any(|v| *v < 0) {
        return Err(Error::General("Negative coordinate".into()));
    }

    let mut code = vec![vec![b'.'; max_x as usize + 1]; max_y as usize + 1];

    for (x, y) in points.into_iter() {
        code[y as usize][x as usize] = b'#';
    }

    println!();
    for y in 0..max_y as usize + 1 {
        for x in 0..max_x as usize + 1 {
            print!("{}", code[y][x] as char)
        }
        println!();
    }

    Ok(())
}
