use adventofcode2021::prelude::*;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

const DIR: [(isize, isize); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

fn index_relative<T>(map: &[T], y: usize, x: usize, dy: isize, dx: isize) -> Option<&T> {
    let y = (y as isize) + dy;
    let x = (x as isize) + dx;
    index(map, y, x)
}

fn index<T>(map: &[T], y: isize, x: isize) -> Option<&T> {
    if y >= 0 && (y as usize) < HEIGHT && x >= 0 && (x as usize) < WIDTH {
        Some(&map[(y as usize) * WIDTH + (x as usize)])
    } else {
        None
    }
}

fn index_relative_mut<T>(map: &mut [T], y: usize, x: usize, dy: isize, dx: isize) -> Option<&mut T> {
    let y = (y as isize) + dy;
    let x = (x as isize) + dx;
    index_mut(map, y, x)
}

fn index_mut<T>(map: &mut [T], y: isize, x: isize) -> Option<&mut T> {
    if y >= 0 && (y as usize) < HEIGHT && x >= 0 && (x as usize) < WIDTH {
        Some(&mut map[(y as usize) * WIDTH + (x as usize)])
    } else {
        None
    }
}

fn print_map(map: &[usize]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let level = map[y * WIDTH + x];
            eprint!("{}", if level > 9 { '#' } else { ((level as u8) + b'0') as char })
        }
        eprintln!();
    }
    eprintln!();
}

pub fn main() -> Result<()> {
    let mut map = include_str!("../../data/a11_input.txt").trim().bytes().filter_map(|b| if b == b'\n' { None } else { Some((b - b'0') as usize) }).collect::<Vec<_>>();
    assert_eq!(map.len(), WIDTH * HEIGHT);

    print_map(&map);

    let mut new_map = vec![0; WIDTH * HEIGHT];

    let mut total_flashes = 0_usize;

    for step in 1..=100 {
        // new_map.fill(0);
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
                            if let Some(level) = index_relative_mut(&mut map, y, x, dy, dx) {
                                *level += 1;
                            }
                        });
                    }
                }
            }

            // std::mem::swap(&mut map, &mut new_map);

            if !any_flashed {
                break;
            }
        }
        // eprintln!("After Step {}", step);
        // print_map(&map);
        map.iter_mut().for_each(|level| if *level >= 10 { *level = 0 });
    }

    println!("Part1: {}", total_flashes);


    Ok(())
}