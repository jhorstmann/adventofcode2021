#![feature(array_windows)]
#![feature(int_abs_diff)]

use adventofcode2021::prelude::*;

type Matrix = [[i32; 3]; 3];
type Vector = [i32; 3];

const IDENTITY: Matrix = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

const fn int_sin(angle: i32) -> i32 {
    match angle {
        0 => 0,
        90 => 1,
        180 => 0,
        270 => -1,
        _ => panic!("sin: unsupported angle"),
    }
}

const fn int_cos(angle: i32) -> i32 {
    match angle {
        0 => 1,
        90 => 0,
        180 => -1,
        270 => 0,
        _ => panic!("cos: unsupported angle"),
    }
}

const fn rotate_x(angle: i32) -> Matrix {
    let mut m = IDENTITY;
    let s = int_sin(angle);
    let c = int_cos(angle);
    m[1][1] = c;
    m[1][2] = -s;
    m[2][1] = s;
    m[2][2] = c;
    m
}

const fn rotate_y(angle: i32) -> Matrix {
    let mut m = IDENTITY;
    let s = int_sin(angle);
    let c = int_cos(angle);
    m[0][0] = c;
    m[0][2] = s;
    m[2][0] = -s;
    m[2][2] = c;
    m
}

const fn rotate_z(angle: i32) -> Matrix {
    let mut m = IDENTITY;
    let s = int_sin(angle);
    let c = int_cos(angle);
    m[0][0] = c;
    m[0][1] = -s;
    m[1][0] = s;
    m[1][1] = c;
    m
}

const fn matrix_mul(a: &Matrix, b: &Matrix) -> Matrix {
    let mut m = [[0; 3]; 3];
    // for loops are not supported in const fn
    let mut i = 0;
    while i < 3 {
        let mut j = 0;
        while j < 3 {
            m[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];
            j += 1;
        }
        i += 1;
    }
    m
}

#[allow(unused)]
const fn rotate_axis(v: &Vector, angle: i32) -> Matrix {
    let s = int_sin(angle);
    let c = int_cos(angle);

    let c1 = 1 - c;
    let x2 = v[0] * v[0];
    let y2 = v[1] * v[1];
    let z2 = v[2] * v[2];

    let xy = v[0] * v[1];
    let xz = v[0] * v[2];
    let yz = v[1] * v[2];

    let xs = v[0] * s;
    let ys = v[1] * s;
    let zs = v[2] * s;

    let mut m = [[0; 3]; 3];

    m[0][0] = x2 * c1 + c;
    m[0][1] = xy * c1 - zs;
    m[0][2] = xz * c1 + ys;

    m[1][0] = xy * c1 + zs;
    m[1][1] = y2 * c1 + c;
    m[1][2] = yz * c1 - xs;

    m[2][0] = xz * c1 - ys;
    m[2][1] = yz * c1 + xs;
    m[2][2] = z2 * c1 + c;

    m
}

const fn transform(v: &Vector, m: &Matrix) -> Vector {
    let mut r = [0; 3];
    r[0] = v[0] * m[0][0] + v[1] * m[0][1] + v[2] * m[0][2];
    r[1] = v[0] * m[1][0] + v[1] * m[1][1] + v[2] * m[1][2];
    r[2] = v[0] * m[2][0] + v[1] * m[2][1] + v[2] * m[2][2];
    r
}

fn normalize_coordinates(coordinates: &mut [Vector]) -> Option<Vector> {
    // coordinates.sort();
    if let Some((first, rest)) = coordinates.split_first_mut() {
        rest.iter_mut().for_each(|v| {
            v[0] -= first[0];
            v[1] -= first[1];
            v[2] -= first[2];
        });
        let res = *first;
        first[0] = 0;
        first[1] = 0;
        first[2] = 0;
        Some(res)
    } else {
        None
    }
}

fn rotate_coordinates(coordinates: &mut [Vector], m: &Matrix) {
    coordinates.iter_mut().for_each(|v| {
        *v = transform(v, m);
    });
}

fn translate_coordinates(coordinates: &mut [Vector], v: &Vector) {
    coordinates.iter_mut().for_each(|[x, y, z]| {
        *x += v[0];
        *y += v[1];
        *z += v[2];
    });
}

pub fn main() -> Result<()> {
    assert_eq!(IDENTITY, matrix_mul(&IDENTITY, &IDENTITY));

    const ANGLES: [i32; 4] = [0, 90, 180, 270];
    let mut rotations = ANGLES
        .iter()
        .flat_map(|x| {
            ANGLES.iter().flat_map(|y| {
                ANGLES.iter().map(|z| {
                    let mx = rotate_x(*x);
                    let my = rotate_y(*y);
                    let mz = rotate_z(*z);
                    let tmp = matrix_mul(&mx, &my);
                    matrix_mul(&tmp, &mz)
                })
            })
        })
        .collect::<Vec<_>>();
    rotations.sort();
    rotations.dedup();

    assert_eq!(rotations.len(), 24);

    let mut scanners = include_str!("../../data/a19_input.txt")
        .split("\n\n")
        .map(|scanner| {
            let mut lines = scanner.lines();
            let header = lines.next().expect("header line");
            assert!(header.starts_with("--- scanner "));
            lines
                .map(|line| {
                    let mut vector = [0_i32; 3];
                    line.split(",")
                        .zip(vector.iter_mut())
                        .for_each(|(s, out)| *out = s.parse::<i32>().expect("number"));
                    vector
                })
                .collect::<Vec<Vector>>()
        })
        .collect::<Vec<_>>();

    let scanner_len = scanners.len();

    let mut first_coords = scanners.remove(0);
    normalize_coordinates(&mut first_coords);

    let mut aligned: Vec<(Vec<Vector>, Vector)> = Vec::with_capacity(scanner_len);
    aligned.push((first_coords, [0, 0, 0]));

    while !scanners.is_empty() {
        let found = scanners.iter().enumerate().find_map(|(i, coords)| {
            rotations.iter().find_map(|rotate_matrix| {
                let mut coords = coords.clone();
                rotate_coordinates(&mut coords, rotate_matrix);

                aligned.iter().find_map(|(aligned_coords, _)| {
                    aligned_coords.iter().find_map(|ac| {
                        coords.iter().find_map(|rc| {
                            let count_intersect = aligned_coords.iter().filter(|a| coords.iter().any(|r| {
                                a[0]-ac[0] == r[0]-rc[0] && a[1]-ac[1] == r[1]-rc[1] && a[2]-ac[2] == r[2]-rc[2]
                            })).count();
                            if count_intersect >= 12 {
                                let offset = [ac[0]-rc[0], ac[1]-rc[1], ac[2]-rc[2]];
                                Some(offset)
                            } else {
                                None
                            }
                        })
                    })

                }).map(|offset| {
                    translate_coordinates(&mut coords, &offset);
                    (i, coords, offset)
                })
            })
        });

        if let Some((index, coords, offset)) = found {
            aligned.push((coords, offset));
            scanners.remove(index);
        } else {
            panic!("Found no matching scanners");
        }
    }

    let scanner_positions = aligned.iter().map(|(_, offset)| *offset).collect::<Vec<_>>();
    let mut deduplicated = aligned
        .into_iter()
        .map(|(coords, _)| coords)
        .flatten()
        .collect::<Vec<_>>();
    deduplicated.sort();
    deduplicated.dedup();

    println!("Part1: {}", deduplicated.len());

    let max_manhattan = scanner_positions
        .iter()
        .map(|v1| {
            scanner_positions
                .iter()
                .map(|v2| v1[0].abs_diff(v2[0]) + v1[1].abs_diff(v2[1]) + v1[2].abs_diff(v2[2]))
        })
        .flatten()
        .max()
        .expect("max");

    println!("Part2: {}", max_manhattan);

    Ok(())
}
