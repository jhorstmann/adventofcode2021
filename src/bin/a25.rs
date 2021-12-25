use adventofcode2021::prelude::*;

pub fn main() -> Result<()> {
    let mut map = include_str!("../../data/a25_input.txt")
        .bytes()
        .filter(|b| !b.is_ascii_whitespace())
        .collect::<Vec<_>>();

    dbg!(map.len());

    let width = if map.len() == 90 {
        10
    } else if map.len() == 7 * 7 {
        7
    } else if map.len() == 139 * 137 {
        139
    } else {
        panic!("Unsupported map size {}", map.len());
    };
    assert_eq!(map.len() % width, 0);
    let width = width as usize;
    let height = map.len() / width;

    let mut new_map = vec![b'.'; map.len()];

    let debug = false;

    let mut step = 0;
    // let dirs = [b'>', ]

    loop {
        let mut moved = false;

        if debug {
            for y in 0..height {
                for x in 0..width {
                    eprint!("{}", map[y * width + x] as char);
                }
                eprintln!();
            }
            eprintln!();
        }

        // new_map.fill(b'.');

        for y in 0..height {
            for x in 0..width {
                let curr = map[y * width + x];
                let prev = map[y * width + (x + width - 1) % width];
                let next = map[y * width + (x + 1) % width];
                new_map[y * width + x] = if curr == b'.' {
                    if prev == b'>' {
                        prev
                    } else {
                        curr
                    }
                } else if curr == b'>' {
                    if next == b'.' {
                        b'.'
                    } else {
                        b'>'
                    }
                } else {
                    curr
                };
                moved |= new_map[y * width + x] != curr;
            }
        }
        std::mem::swap(&mut map, &mut new_map);


        for y in 0..height {
            for x in 0..width {
                let curr = map[y * width + x];
                let prev = map[(y + height - 1) % height * width + x];
                let next = map[(y + 1) % height * width + x];
                new_map[y * width + x] = if curr == b'.' {
                    if prev == b'v' {
                        prev
                    } else {
                        curr
                    }
                } else if curr == b'v' {
                    if next == b'.' {
                        b'.'
                    } else {
                        b'v'
                    }
                } else {
                    curr
                };
                moved |= new_map[y * width + x] != curr;
            }
        }

        step += 1;
        std::mem::swap(&mut map, &mut new_map);

        if !moved {
            break;
        }
        if step > 10_000 {
            break;
        }
    }


    println!("Part 1: {}", step);


    Ok(())
}