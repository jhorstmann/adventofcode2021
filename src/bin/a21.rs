use adventofcode2021::prelude::*;

pub fn main() -> Result<()> {
    let _example = [4, 8];
    let input = [4, 10];

    let mut positions = input;
    let mut scores = [0, 0];
    let mut dice = 0_usize;
    let mut rolls = 0_usize;
    'outer: loop {
        for i in 0..2 {
            // die is zero-based
            let roll = dice % 100 + (dice + 1) % 100 + (dice + 2) % 100 + 3;
            rolls += 3;

            dice += 3;
            dice %= 100;
            positions[i] += roll;
            while positions[i] > 10 {
                positions[i] -= 10;
            }

            scores[i] += positions[i];

            if scores[i] >= 1000 {
                break 'outer;
            }
        }
        // dbg!(positions, scores, rolls);
    }

    println!("Part 1: {}", scores[0].min(scores[1]) * rolls);

    Ok(())
}
