use adventofcode2021::prelude::*;

pub fn main() -> Result<()> {
    let map = include_str!("../../data/a9_input.txt")
        .lines()
        .map(|l| l.bytes().map(|b| b-('0' as u8)).collect::<Vec<u8>>())
        .collect::<Vec<_>>();


    let height = map.len();
    let width = map.iter().map(|row| row.len()).max().ok_or(Error::EmptyIterator)?;

    if map.iter().any(|row| row.len() != width) {
        return Err(Error::General("non-rectangular map".into()));
    }

    let dir: [(isize, isize); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1,1), (0, 1), (1, 1)];

    let mut risk = 0_u64;
    for y in 0..height as isize {
        for x in 0..width as isize {
            let h = map[y as usize][x as usize] as u64;
            if dir.iter().all(|(dx, dy)| {
                match map.get((y+ *dy) as usize).map(|row| row.get((x+ *dx) as usize)).flatten() {
                    Some(h2) if *h2 as u64 > h => true,
                    None => true,
                    _ => false
                }
            }) {
                risk += h+1;
            }
        }
    }
    println!("{}", risk);


    Ok(())
}