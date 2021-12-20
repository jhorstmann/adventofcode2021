use adventofcode2021::prelude::*;

const DIR: [(isize, isize); 9] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (0, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

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
    DIR.iter().enumerate().fold(0_usize, |a, (i, (dx, dy))| {
        (a<<1) | (((get_bit(image, width, height, y, x, *dy, *dx)) as usize))
    })
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

    assert_eq!(get_index(&[b'.', b'.', b'.', b'#', b'.', b'.', b'.', b'#', b'.'], 3, 3, 1, 1), 0b000100010);
    assert_eq!(get_index(&[b'#', b'.', b'.', b'#', b'.', b'.', b'.', b'#', b'.'], 3, 3, 1, 1), 0b100100010);
    assert_eq!(get_index(&[b'#', b'.', b'#', b'#', b'.', b'.', b'.', b'#', b'.'], 3, 3, 1, 1), 0b101100010);
    assert_eq!(get_index(&[b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.'], 3, 3, 1, 1), 0b000000000);
    assert_eq!(get_index(&[b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.'], 3, 3, 0, 0), 0b000000000);
    assert_eq!(get_index(&[b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.'], 3, 3, 2, 0), 0b000000000);
    assert_eq!(get_index(&[b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.'], 3, 3, 2, 2), 0b000000000);

    let height = width;

    let original = image.clone();

    {
        let original_width = width;
        let border = 102;
        let height = height+ border*2;
        let width = width+ border*2;
        let mut image = vec![b'.'; (width)*(height)];

        for y in 0..original_width {
            let start = (y+border) * width + border;
            image[start..start+original_width].copy_from_slice(&original[y*original_width..y*original_width+original_width]);
        }


        let mut next_image = vec![0_u8; image.len()];

        for i in 0..50 {
            for y in 0..height {
                for x in 0..width {
                    let mut idx = get_index(&image, width, height, y, x);
                    let replacement = lookup[idx];
                    next_image[y*width+x] = replacement;
                }
            }

            // eprintln!();
            let mut count = 0_usize;
            for y in i..height-i {
                for x in i..width-i {
                    let b = next_image[y * width + x];
                    eprint!("{}", b as char);
                    if b == b'#' {
                        count +=1;
                    }
                }
                eprintln!();
            }
            eprintln!();

            println!("{}", count);

            std::mem::swap(&mut image, &mut next_image);
        }
    }

    // part1: 5812 too high
    // part2: 22617 too high
    // part2: 20808 too high


    Ok(())
}