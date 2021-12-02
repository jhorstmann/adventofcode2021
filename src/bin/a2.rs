use adventofcode2021::prelude::*;


pub fn main() -> Result<()> {
    let data = include_str!("../../data/a2_input.txt");

    let (x, depth) = data.lines().try_fold((0, 0), |(mut x, mut depth), line| -> Result<_> {
        if let Some(forward) = line.strip_prefix("forward ") {
            x += forward.parse::<i32>()?;
        } else if let Some(down) =  line.strip_prefix("down ") {
            depth += down.parse::<i32>()?;
        } else if let Some(up) =  line.strip_prefix("up ") {
            depth -= up.parse::<i32>()?;
        }
        Ok((x, depth))
    })?;

    let part1 = x*depth;
    println!("{}", part1);


    let (_, x, depth) = data.lines().try_fold((0, 0, 0), |(mut aim, mut x, mut depth), line| -> Result<_> {
        if let Some(forward) = line.strip_prefix("forward ") {
            let n = forward.parse::<i32>()?;
            x += n;
            depth += aim*n;
        } else if let Some(down) =  line.strip_prefix("down ") {
            aim += down.parse::<i32>()?;
        } else if let Some(up) =  line.strip_prefix("up ") {
            aim -= up.parse::<i32>()?;
        }
        Ok((aim, x, depth))
    })?;

    let part2 = x*depth;
    println!("{}", part2);

    Ok(())
}
