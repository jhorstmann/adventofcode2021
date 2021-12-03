use adventofcode2021::prelude::*;

pub fn main() -> Result<()> {
    let data = include_str!("../../data/a3_input.txt");

    let numbers = data.lines().map(|l| Ok(u64::from_str_radix(l, 2)?)).collect::<Result<Vec<u64>>>()?;

    let mut histogram = vec![0_usize; 64];

    for n in numbers.iter() {
        for b in 0..64 {
            if n & (1 << b) != 0 {
                histogram[b] += 1;
            }
        }
    }

    let digits = histogram.len() - histogram.iter().rev().take_while(|hist| **hist == 0).count();

    let gamma = histogram.iter().take(digits).enumerate().map(|(i, hist)| {
        if *hist >= numbers.len() / 2 {
            1 << i
        } else {
            0
        }
    }).sum::<u64>();

    let epsilon = (!gamma) & ((1 << digits) - 1);

    println!("{:012b} * {:012b} = {}", gamma, epsilon, gamma * epsilon);
    println!("{} * {} = {}", gamma, epsilon, gamma * epsilon);

    let mut ox_rating = numbers.clone();
    for bit in (0..digits).rev() {
        let len = ox_rating.len();
        let count_ones = ox_rating.iter().filter(|n| **n & (1 << bit) != 0).count();

        let search_bit = count_ones * 2 >= len;

        ox_rating.retain(|n| (n & (1 << bit) != 0) == search_bit);

        if ox_rating.len() <= 1 {
            break;
        }
    }

    let ox_rating = ox_rating[0];

    let mut co2_rating = numbers.clone();
    for bit in (0..digits).rev() {
        let len = co2_rating.len();
        let count_ones = co2_rating.iter().filter(|n| **n & (1 << bit) != 0).count();

        let search_bit = !(count_ones * 2 >= len);

        co2_rating.retain(|n| (n & (1 << bit) != 0) == search_bit);

        if co2_rating.len() <= 1 {
            break;
        }
    }

    let co2_rating = co2_rating[0];

    println!("{} * {} = {}", ox_rating, co2_rating, ox_rating * co2_rating);

    Ok(())
}