#![feature(core_intrinsics)]
use adventofcode2021::prelude::*;

fn part1(ages: &[u32], days: i32) -> usize {
    let mut ages = ages.to_vec();
    let mut new_ages = Vec::with_capacity(4096);

    for _day in 0..days {
        new_ages.clear();
        for age in ages.iter() {
            if let Some(age) = age.checked_sub(1) {
                new_ages.push(age);
            } else {
                new_ages.push(6);
                new_ages.push(8);
            }
        }

        std::mem::swap(&mut ages, &mut new_ages)
    }

    ages.len()
}

fn part2(ages: &[u32], days: usize) -> usize {
    let mut histogram = [0_usize; 9];
    for age in ages.iter() {
        unsafe { std::intrinsics::assume((*age as usize) < histogram.len()); }
        histogram[*age as usize] += 1;
    }

    for _day in 0..days {
        let mut count = histogram[0];
        unsafe {
            std::ptr::copy(histogram.as_ptr().add(1), histogram.as_mut_ptr(), 8)
        }
        // for i in 1..histogram.len() {
        //     histogram[i - 1] = histogram[i]
        // }
        histogram[6] += count;
        histogram[8] = count;
    }

    histogram.iter().sum()
}

pub fn main() -> Result<()> {
    let data = include_str!("../../data/a6_input.txt");

    let ages = data
        .trim()
        .split(",")
        .map(|n| Ok(u32::from_str(n)?))
        .collect::<Result<Vec<u32>>>()?;

    println!("After  18 Days: {}", part1(&ages, 18));
    println!("After  80 Days: {}", part1(&ages, 80));
    println!();

    println!("Part 2 optimized version");
    for days in [18, 80, 256] {
        println!("After {:3} Days: {}", days, part2(&ages, days));
    }

    Ok(())
}
