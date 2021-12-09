#![feature(core_intrinsics)]

use criterion::{criterion_group, criterion_main, Criterion};
use std::str::FromStr;

#[inline(never)]
fn part2(ages: &[usize], days: usize) -> usize {
    let mut histogram = [0_usize; 9];
    for age in ages.iter() {
        unsafe {
            std::intrinsics::assume(*age < histogram.len());
        }
        histogram[*age] += 1;
    }

    for _day in 0..days {
        let count = histogram[0];
        for i in 1..histogram.len() {
            histogram[i - 1] = histogram[i]
        }
        histogram[6] += count;
        histogram[8] = count;
    }

    histogram.iter().sum()
}

#[inline(never)]
fn part2_ptr_copy(ages: &[usize], days: usize) -> usize {
    let mut histogram = [0_usize; 9];
    for age in ages.iter() {
        unsafe {
            std::intrinsics::assume(*age < histogram.len());
        }
        histogram[*age] += 1;
    }

    for _day in 0..days {
        let count = histogram[0];
        unsafe {
            std::ptr::copy(histogram.as_ptr().add(1), histogram.as_mut_ptr(), 8)
        }
        histogram[6] += count;
        histogram[8] = count;
    }

    histogram.iter().sum()
}

#[inline(never)]
fn part2_rotate(ages: &[usize], days: usize) -> u64 {
    let mut histogram = [0_u64; 9];
    for age in ages {
        unsafe {
            std::intrinsics::assume(*age < histogram.len());
        }
        histogram[*age as usize] += 1;
    }
    for _ in 0..days {
        histogram.rotate_left(1);
        histogram[6] += histogram[8];
    }
    histogram.iter().sum()
}

fn bench_a6(c: &mut Criterion) {
    let ages = include_str!("../data/a6_input.txt")
        .split(",")
        .map(|n| usize::from_str(n).unwrap())
        .collect::<Vec<usize>>();

    let days = 256;

    c.bench_function("part2", |b| b.iter(|| part2(&ages, days)));
    c.bench_function("part2_ptr_copy", |b| b.iter(|| part2_ptr_copy(&ages, days)));
    c.bench_function("part2_rotate", |b| b.iter(|| part2_rotate(&ages, days)));
}

criterion_group!(benches, bench_a6);
criterion_main!(benches);
