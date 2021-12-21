use adventofcode2021::prelude::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn solve_part1(mut positions: [usize; 2]) -> usize {
    let mut scores = [0, 0];
    let mut dice = 0_usize;
    let mut rolls = 0_usize;
    loop {
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
                return scores[0].min(scores[1]) * rolls;
            }
        }
    }
}

const PROBABILITIES: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];

fn solve_part2(positions: [usize; 2], scores: [usize; 2], player: usize) -> (usize, usize) {
    if scores[player] >= 21 {
        (1, 1)
    } else if scores[1 - player] >= 21 {
        (0, 1)
    } else {
        let mut new_positions = positions;
        let mut new_scores = scores;

        let mut wins = 0;
        let mut universes = 0;

        for (i, n) in PROBABILITIES.iter().enumerate() {
            new_positions[player] = positions[player] + i + 3;
            while new_positions[player] > 10 {
                new_positions[player] -= 10;
            }
            new_scores[player] = scores[player] + new_positions[player];

            if new_scores[player] < 21 {
                let (wins1, universes1) = solve_part2(new_positions, new_scores, 1 - player);
                wins += n * (universes1 - wins1);
                universes += n * universes1;
            } else {
                wins += n;
                universes += n;
            }
        }

        (wins, universes)
    }
}

pub fn main() -> Result<()> {
    let _example = [4, 8];
    let input = [4, 10];

    println!("Part 1: {}", solve_part1(input));

    let (wins, universes) = solve_part2(input, [0, 0], 0);

    println!("Part 2: {:?}", wins.max(universes - wins));

    Ok(())
}
