use adventofcode2021::prelude::*;

enum SyntaxError {
    UnexpectedChar(u8),
    Incomplete(u8),
}

struct Chunk {
    #[allow(unused)]
    start_char: u8,
    children: Vec<Chunk>,
}

impl Chunk {
    fn new(start_char: u8) -> Self {
        Self {
            start_char,
            children: vec![]
        }
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

fn parse_chunk(start_char: u8, mut line: &[u8]) -> std::result::Result<(&[u8], Chunk), SyntaxError> {
    let end_char = closing_delimiter(start_char).ok_or(SyntaxError::UnexpectedChar(start_char))?;
    if line.is_empty() {
        return Err(SyntaxError::Incomplete(end_char))
    }
    let mut chunk = Chunk::new(start_char);
    while let Some((first, rest)) = line.split_first() {
        if *first == end_char {
            line = rest;
            break;
        } else if is_closing_delimiter(*first) {
            return Err(SyntaxError::UnexpectedChar(*first))
        } else {
            let (rest, nested_chunk) = parse_chunk(*first, rest)?;
            line = rest;
            chunk.children.push(nested_chunk);
        }
    }
    Ok((line, chunk))
}

fn parse_line(line: &[u8]) -> std::result::Result<Vec<Chunk>, SyntaxError> {
    let mut chunks = vec![];
    let mut line = line;
    while let Some((first, rest)) = line.split_first() {
        let (rest, chunk) = parse_chunk(*first, rest)?;
        line = rest;
        chunks.push(chunk);
    }
    Ok(chunks)
}

pub fn main() -> Result<()> {

    let data = include_str!("../../data/a10_example.txt");

    data.lines().for_each(|line| {
        print!("{}: ", line);
        match parse_line(line.as_bytes()) {
            Ok(_chunks) => println!("Ok"),
            Err(SyntaxError::Incomplete(ch)) => println!("Missing {}", ch as char),
            Err(SyntaxError::UnexpectedChar(ch)) => println!("Unexpected {}", ch as char),
        }
    });

    let part1 = data.lines().map(|line| {
        match parse_line(line.as_bytes()) {
            Ok(_chunks) => 0_u64,
            Err(SyntaxError::Incomplete(_)) => 0_u64,
            Err(SyntaxError::UnexpectedChar(ch)) => score(ch),
        }
    }).sum::<u64>();

    println!("Part1: {}", part1);

    Ok(())

}