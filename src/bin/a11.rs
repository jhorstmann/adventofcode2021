use adventofcode2021::prelude::*;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

const DIR: [(isize, isize); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

#[allow(unused)]
fn print_map(map: &[u8]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let level = map[y * WIDTH + x];
            eprint!("{}", if level > 9 { '#' } else { (level + b'0') as char })
        }
        eprintln!();
    }
    eprintln!();
}

pub fn main() -> Result<()> {
    let mut map = include_str!("../../data/a11_input.txt")
        .trim()
        .bytes()
        .filter_map(|b| if b == b'\n' { None } else { Some(b - b'0') })
        .collect::<Vec<_>>();

    assert_eq!(map.len(), WIDTH * HEIGHT);

    let mut total_flashes = 0_usize;

    let mut step = 0;
    loop {
        map.iter_mut().for_each(|level| *level += 1);
        let mut flashed = [[false; WIDTH]; HEIGHT];
        loop {
            let mut any_flashed = false;

            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let level = map[y * WIDTH + x] as usize;

                    if level >= 10 && !flashed[y][x] {
                        flashed[y][x] = true;
                        any_flashed = true;
                        total_flashes += 1;

                        DIR.iter().copied().for_each(|(dx, dy)| {
                            if let Some(level) = get_2d_relative_mut::<_, WIDTH>(&mut map, y, x, dy, dx) {
                                *level = (*level + 1).min(10);
                            }
                        });
                    }
                }
            }

            if !any_flashed {
                break;
            }
        }

        // print_map(&map);
        map.iter_mut().for_each(|level| {
            if *level >= 10 {
                *level = 0
            }
        });

        step += 1;
        if step == 100 {
            println!("Part1: {}", total_flashes);
        }

        if flashed.iter().all(|row| row.iter().all(|flashed| *flashed)) {
            println!("Part2: {}", step);
            break;
        }
    }

    Ok(())
}
