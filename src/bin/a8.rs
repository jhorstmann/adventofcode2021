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

fn part1(lines: &[Input]) -> usize {
    lines
        .iter()
        .flat_map(|input| input.output_values.iter())
        .filter(|v| {
            let segments = v.count_ones();
            segments == 2 || segments == 4 || segments == 3 || segments == 7
        })
        .count()
}

fn part2_line(input: &Input) -> Result<u32> {
    let patterns = input.patterns;
    let one = *patterns
        .iter()
        .find(|m| m.count_ones() == 2)
        .ok_or(Error::General("one".into()))?;
    let seven = *patterns
        .iter()
        .find(|m| m.count_ones() == 3)
        .ok_or(Error::General("seven".into()))?;
    let eight = *patterns
        .iter()
        .find(|m| m.count_ones() == 7)
        .ok_or(Error::General("eight".into()))?;
    let four = *patterns
        .iter()
        .find(|m| m.count_ones() == 4)
        .ok_or(Error::General("four".into()))?;
    let zero = *patterns
        .iter()
        .find(|m| m.count_ones() == 6 && (**m & !seven).count_ones() == 3 && (**m & !four).count_ones() == 3)
        .ok_or(Error::General("zero".into()))?;
    let three = *patterns
        .iter()
        .find(|m| m.count_ones() == 5 && (**m & !seven).count_ones() == 2)
        .ok_or(Error::General("three".into()))?;
    let nine = *patterns
        .iter()
        .find(|m| m.count_ones() == 6 && (**m & seven).count_ones() == 3 && (**m & !four).count_ones() == 2)
        .ok_or(Error::General("nine".into()))?;
    let six = *patterns
        .iter()
        .find(|m| m.count_ones() == 6 && (**m & one).count_ones() == 1)
        .ok_or(Error::General("six".into()))?;
    let five = *patterns
        .iter()
        .find(|m| m.count_ones() == 5 && (**m & one).count_ones() == 1 && (**m & four).count_ones() == 3)
        .ok_or(Error::General("five".into()))?;
    let two = *patterns
        .iter()
        .find(|m| m.count_ones() == 5 && (**m & one).count_ones() == 1 && (**m & four).count_ones() == 2)
        .ok_or(Error::General("two".into()))?;

    let numbers = [zero, one, two, three, four, five, six, seven, eight, nine];

    let result = input.output_values.iter().try_fold(0_u32, |a, m| -> Result<u32> {
        let digit_pos = numbers.iter().position(|n| *n == *m).ok_or(Error::EmptyIterator)?;
        Ok(a * 10 + digit_pos as u32)
    })?;

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

    println!("Part1: {}", part1(&lines));

    let part2 = lines.iter().try_fold(0_u32, |sum, l| -> Result<u32> {
        let num = part2_line(l)?;
        Ok(sum + num)
    })?;

    println!("Part2: {}", part2);

    Ok(())
}
