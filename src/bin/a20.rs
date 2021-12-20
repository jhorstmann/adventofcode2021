use adventofcode2021::prelude::*;

const DIR: [(isize, isize); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[inline]
fn get_bit(image: &[u8], width: usize, height: usize, y: usize, x: usize, dy: isize, dx: isize) -> bool {
    let y = (y as isize) + dy;
    let x = (x as isize) + dx;
    if y >= 0 && (y as usize) < height && x >= 0 && (x as usize) < width {
        image[(y as usize) * width + (x as usize)] == b'#'
    } else {
        false
    }
}

fn get_index(image: &[u8], width: usize, height: usize, y: usize, x: usize) -> usize {
    DIR.iter().fold(0_usize, |a, (dx, dy)| {
        (a << 1) | ((get_bit(image, width, height, y, x, *dy, *dx)) as usize)
    })
}

fn iterate(
    original_image: &[u8],
    original_width: usize,
    original_height: usize,
    lookup: &[u8],
    iterations: usize,
    debug: bool,
) -> usize {
    let border = iterations * 2 + 2;
    let height = original_height + border * 2;
    let width = original_width + border * 2;
    let mut image = vec![b'.'; width * height];

    for y in 0..original_width {
        let start = (y + border) * width + border;
        image[start..start + original_width]
            .copy_from_slice(&original_image[y * original_width..y * original_width + original_width]);
    }

    let mut next_image = vec![0_u8; image.len()];

    let mut count = 0_usize;

    for i in 0..iterations {
        count = 0;
        for y in 0..height {
            for x in 0..width {
                let idx = get_index(&image, width, height, y, x);
                next_image[y * width + x] = lookup[idx];
            }
        }

        for y in i..height - i {
            for x in i..width - i {
                let b = next_image[y * width + x];
                if b == b'#' {
                    count += 1;
                }
            }
        }

        if debug {
            for y in i..height - i {
                for x in i..width - i {
                    let b = next_image[y * width + x];
                    eprint!("{}", b as char);
                }
                eprintln!();
            }
            eprintln!("{}", count);
            eprintln!();
        }

        std::mem::swap(&mut image, &mut next_image);
    }

    count
}

pub fn main() -> Result<()> {
    let mut lines = include_str!("../../data/a20_input.txt").lines();
    let lookup = lines.next().expect("lookup").trim().as_bytes();
    let empty = lines.next().expect("empty line");
    assert!(empty.is_empty());
    let mut width = None;
    let mut image = vec![];
    for line in lines {
        let line = line.trim();
        if let Some(width) = width {
            if width != line.len() {
                panic!("Lines have different lengths {} <> {}", width, line.len());
            }
        }
        width = Some(line.len());
        image.extend_from_slice(line.as_bytes());
    }

    let width = width.expect("width");

    assert_eq!(image.len() % width, 0);
    assert_eq!(image.len() / width, width);

    assert_eq!(get_index(b"...#...#.", 3, 3, 1, 1), 0b000100010);
    assert_eq!(get_index(b"#..#...#.", 3, 3, 1, 1), 0b100100010);
    assert_eq!(get_index(b"#.##...#.", 3, 3, 1, 1), 0b101100010);
    assert_eq!(get_index(b".........", 3, 3, 1, 1), 0b000000000);
    assert_eq!(get_index(b".........", 3, 3, 0, 0), 0b000000000);
    assert_eq!(get_index(b".........", 3, 3, 2, 0), 0b000000000);
    assert_eq!(get_index(b".........", 3, 3, 2, 2), 0b000000000);

    let part1 = iterate(&image, width, width, lookup, 2, true);
    println!("Part 1: {}", part1);

    let part2 = iterate(&image, width, width, lookup, 50, false);
    println!("Part 2: {}", part2);

    Ok(())
}
