use adventofcode2021::prelude::*;

const DIR: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

fn floodfill(map: &mut [Vec<u8>], size: &mut usize, x: usize, y: usize) {
    let h = &mut map[y as usize][x as usize];
    if *h < 9_u8 {
        *size += 1;
        *h = 255; // mark as visited

        if y > 0 {
            floodfill(map, size, x, y - 1);
        }
        if y < map.len() - 1 {
            floodfill(map, size, x, y + 1);
        }
        if x > 0 {
            floodfill(map, size, x - 1, y);
        }
        if x < map[y].len() - 1 {
            floodfill(map, size, x + 1, y);
        }
    }
}

pub fn main() -> Result<()> {
    let mut map = include_str!("../../data/a9_input.txt")
        .lines()
        .map(|l| l.bytes().map(|b| b - ('0' as u8)).collect::<Vec<u8>>())
        .collect::<Vec<_>>();

    let height = map.len();
    let width = map.iter().map(|row| row.len()).max().ok_or(Error::EmptyIterator)?;

    if map.iter().any(|row| row.len() != width) {
        return Err(Error::General("non-rectangular map".into()));
    }

    let mut risk = 0_u64;
    let mut basins = vec![];
    for y in 0..height as isize {
        for x in 0..width as isize {
            let h = map[y as usize][x as usize] as u64;
            if DIR.iter().all(|(dx, dy)| {
                match map
                    .get((y + *dy) as usize)
                    .map(|row| row.get((x + *dx) as usize))
                    .flatten()
                {
                    Some(h2) if *h2 as u64 > h => true,
                    None => true,
                    _ => false,
                }
            }) {
                risk += h + 1;
                basins.push((x as usize, y as usize));
            }
        }
    }
    println!("Part1: {}", risk);

    let mut sizes = vec![];

    for (x, y) in basins {
        let mut size = 0;
        floodfill(&mut map, &mut size, x, y);
        sizes.push(size);
    }

    sizes.sort();
    let part2 = sizes.iter().rev().take(3).fold(1_usize, |mut a, s| {
        a *= s;
        a
    });

    println!("Part2: {}", part2);

    Ok(())
}
