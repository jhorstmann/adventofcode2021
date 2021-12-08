use adventofcode2021::prelude::*;

struct Input {
    patterns: [u8; 10],
    output_values: [u8; 4],
}

fn pattern_to_mask(pattern: &str) -> u8 {
    pattern
        .bytes()
        .fold(0_u8, |acc, segment| acc | (1 << (segment as u8 - ('a' as u8))))
}

fn parse<const N: usize>(input: &str) -> Result<[u8; N]> {
    let mut result = [0_u8; N];
    input
        .split(" ")
        .map(pattern_to_mask)
        .enumerate()
        .take(N)
        .for_each(|(i, p)| result[i] = p);

    if result.iter().any(|mask| *mask == 0) {
        return Err(Error::General("Missing input".into()));
    }

    Ok(result)
}

pub fn main() -> Result<()> {
    let lines = include_str!("../../data/a8_input.txt")
        .lines()
        .map(|l| {
            let (patterns, output_values) = l.split_once(" | ").ok_or(Error::PatternMatch)?;

            Ok(Input {
                patterns: parse(patterns)?,
                output_values: parse(output_values)?,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let part1 = lines
        .iter()
        .map(|input| {
            input
                .output_values
                .iter()
                .filter(|v| {
                    let segments = v.count_ones();
                    segments == 2 || segments == 4 || segments == 3 || segments == 7
                })
                .count()
        })
        .sum::<usize>();

    println!("Part1: {}", part1);

    Ok(())
}
