use std::fmt::{Display, Formatter, Write};
use adventofcode2021::prelude::*;

enum SyntaxError {
    UnexpectedChar(u8),
    Incomplete(u8),
}

struct Chunk {
    start_char: u8,
    end_char: u8,
    children: Vec<Chunk>,
}

impl Chunk {
    fn new(start_char: u8, end_char: u8) -> Self {
        Self {
            start_char,
            end_char,
            children: vec![]
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.start_char as char)?;
        for chunk in self.children.iter() {
            chunk.fmt(f)?;
        }
        f.write_char(self.end_char as char)?;
        Ok(())
    }
}

fn closing_delimiter(ch: u8) -> Option<u8> {
    match ch {
        b'(' => Some(b')'),
        b'[' => Some(b']'),
        b'{' => Some(b'}'),
        b'<' => Some(b'>'),
        _ => None,
    }
}

fn is_closing_delimiter(ch: u8) -> bool {
    match ch {
        b')' | b']' | b'}' | b'>' => true,
        _ => false,
    }
}

fn score(ch: u8) -> u64 {
    match ch {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => unreachable!("invalid character for score: {}", ch as char)
    }
}

fn score_part2(chars: &[u8]) -> u64 {
    chars.iter().fold(0_u64, |a, ch| {
        a * 5 + match *ch {
            b')' => 1,
            b']' => 2,
            b'}' => 3,
            b'>' => 4,
            _ => unreachable!("invalid character for score: {}", *ch as char)
        }
    })
}

fn parse_chunk(start_char: u8, mut line: &[u8], recover: bool) -> std::result::Result<(&[u8], Chunk), SyntaxError> {
    let end_char = closing_delimiter(start_char).ok_or(SyntaxError::UnexpectedChar(start_char))?;
    if line.is_empty() {
        if recover {
            return Ok((line, Chunk::new(start_char, end_char)));
        } else {
            return Err(SyntaxError::Incomplete(end_char))
        }
    }
    let mut chunk = Chunk::new(start_char, end_char);
    while let Some((first, rest)) = line.split_first() {
        if *first == end_char {
            line = rest;
            break;
        } else if is_closing_delimiter(*first) {
            return Err(SyntaxError::UnexpectedChar(*first))
        } else {
            let (rest, nested_chunk) = parse_chunk(*first, rest, recover)?;
            line = rest;
            chunk.children.push(nested_chunk);
        }
    }
    Ok((line, chunk))
}

fn parse_line(line: &[u8], recover: bool) -> std::result::Result<Vec<Chunk>, SyntaxError> {
    let mut chunks = vec![];
    let mut line = line;
    while let Some((first, rest)) = line.split_first() {
        let (rest, chunk) = parse_chunk(*first, rest, recover)?;
        line = rest;
        chunks.push(chunk);
    }
    Ok(chunks)
}

pub fn main() -> Result<()> {

    let data = include_str!("../../data/a10_input.txt");

    data.lines().for_each(|line| {
        eprint!("{}: ", line);
        match parse_line(line.as_bytes(), false) {
            Ok(_chunks) => eprintln!("Ok"),
            Err(SyntaxError::Incomplete(ch)) => eprintln!("Missing {}", ch as char),
            Err(SyntaxError::UnexpectedChar(ch)) => eprintln!("Unexpected {}", ch as char),
        }
    });
    eprintln!();

    let part1 = data.lines().map(|line| {
        match parse_line(line.as_bytes(), false) {
            Ok(_chunks) => 0_u64,
            Err(SyntaxError::Incomplete(_)) => 0_u64,
            Err(SyntaxError::UnexpectedChar(ch)) => score(ch),
        }
    }).sum::<u64>();

    println!("Part1: {}", part1);

    let mut part2_scores = data.lines().filter_map(|line| {
        match parse_line(line.as_bytes(), true) {
            Ok(chunks) => {
                let formatted = chunks.iter().map(|chunk| chunk.to_string()).collect::<String>();
                if formatted.len() == line.len() {
                    eprintln!("Ok");
                    None
                } else {
                    eprintln!("Recovered {} to {}", line, &formatted);
                    Some(formatted[line.len()..].to_string())
                }
            },
            Err(SyntaxError::Incomplete(ch)) => panic!("Missing {} (SHOULDN'T HAPPEN IN RECOVERY MODE)", ch as char),
            Err(SyntaxError::UnexpectedChar(_)) => None,
        }
    }).map(|missing_chars| {
        score_part2(missing_chars.as_bytes())
    }).collect::<Vec<u64>>();
    part2_scores.sort();

    println!("Part2: {}", part2_scores[part2_scores.len()/2]);

    Ok(())

}