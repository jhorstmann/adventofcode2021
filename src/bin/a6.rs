use adventofcode2021::prelude::*;

pub fn main() -> Result<()> {
    let data = include_str!("../../data/a6_example.txt");

    let mut ages = data.split(",").map(|n| Ok(u32::from_str(n)?)).collect::<Result<Vec<u32>>>()?;
    let mut new_ages = Vec::with_capacity(4096);

    for _day in 0..80 {
        new_ages.clear();
        for age in ages.iter() {
            if let Some( age) = age.checked_sub(1) {
                new_ages.push(age);
            } else {
                new_ages.push(6);
                new_ages.push(8);
            }
        }

        std::mem::swap(&mut ages, &mut new_ages)
    }

    println!("{}", ages.len());

    Ok(())
}