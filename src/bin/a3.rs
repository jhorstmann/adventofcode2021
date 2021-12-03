use adventofcode2021::prelude::*;


pub fn main() -> Result<()> {
    let data = include_str!("../../data/a3_input.txt");

    let numbers = data.lines().map(|l| Ok(u64::from_str_radix(l, 2)?)).collect::<Result<Vec<u64>>>()?;


    let mut histogram = vec![0_usize; 64];

    for n in numbers.iter() {
        for b in 0..64 {
            if n & (1<<b) != 0 {
                histogram[b] += 1;
            }
        }
    }

    let digits = data.lines().map(|l| l.len()).max().unwrap();

    dbg!(digits);

    let gamma = histogram.iter().take(digits).enumerate().map(| (i, hist)| {
        if *hist >= numbers.len()/2 {
            1 << i
        } else {
            0
        }
    }).sum::<u64>();

    let epsilon = histogram.iter().take(digits).enumerate().map(| (i, hist)| {
        if *hist < numbers.len()/2 {
            1 << i
        } else {
            0
        }
    }).sum::<u64>();

    let epsilon = ((!gamma) & ((1 << digits)-1));


    println!("{:012b} * {:012b} = {}", gamma, epsilon, gamma * epsilon);
    println!("{} * {} = {}", gamma, epsilon, gamma * epsilon);


    Ok(())


}