#![feature(box_patterns)]
use std::fmt::Write;
use std::fmt::Display;
use std::fmt::Formatter;
use std::iter::{Copied, Peekable};
use std::slice;
use adventofcode2021::prelude::*;

#[derive(Debug, PartialEq, Clone)]
enum SnailfishNumber {
    Number(u32),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

use SnailfishNumber::*;

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailfishNumber::Number(n) => f.write_fmt(format_args!("{}", n))?,
            SnailfishNumber::Pair(left, right) => {
                f.write_char('[')?;
                left.as_ref().fmt(f)?;
                f.write_char(',')?;
                right.as_ref().fmt(f)?;
                f.write_char(']')?;
            }
        }
        Ok(())
    }
}

impl SnailfishNumber {
    fn into_box(self) -> Box<Self> {
        Box::new(self)
    }

    fn add_left(&mut self, to_add: &mut u32) {
        if *to_add > 0 {
            match self {
                Number(n) => {
                    *n += *to_add;
                    *to_add = 0;
                }
                Pair(box left, box right) => {
                    left.add_left(to_add);
                    if *to_add > 0 {
                        right.add_left(to_add);
                    }
                }
            }
        }
    }

    fn add_right(&mut self, to_add: &mut u32) {
        if *to_add > 0 {
            match self {
                Number(n) => {
                    *n += *to_add;
                    *to_add = 0;
                }
                Pair(box left, box right) => {
                    right.add_right(to_add);
                    if *to_add > 0 {
                        left.add_right(to_add);
                    }
                }
            }
        }
    }

    fn explode(self: Box<Self>, level: usize) -> (Box<Self>, u32, u32, bool) {
        match (level >= 4, self) {
            (true, box Pair(box Number(nleft), box Number(nright))) => {
                (Box::new(Number(0)), nleft, nright, false)
            }
            (true, n) => {
                (n, 0, 0, false)
            }
            (false, box Pair(left,mut right)) => {
                let (mut left, nll, mut nlr , left_exploded) = left.explode(level + 1);

                right.as_mut().add_left(&mut nlr);

                let (right, mut nrl, nrr, right_exploded) = right.explode(level + 1);

                left.as_mut().add_right(&mut nrl);

                (Box::new(Pair(left, right)), nll+nrl, nrr+nlr, left_exploded || right_exploded)
            }
            (false, n @ box Number(_)) => {
                (n, 0, 0, false)
            }
        }

    }

    fn split(self: Box<Self>) -> (Box<Self>, bool) {
        match self {
            box Number(n) => {
                if n >= 10 {
                    let left = n / 2;
                    let result = Box::new(Pair(Box::new(Number(left)), Box::new(Number(n - left))));
                    (result, true)
                } else {
                    (Box::new(Number(n)), false)
                }
            },
            box Pair(left, right) => {
                let (new_left, split) = left.split();
                if split {
                    (Box::new(Pair(new_left, right)), split)
                } else {
                    let (new_right, split) = right.split();
                    (Box::new(Pair(new_left, new_right)), split)
                }
            }
        }
    }

    fn reduce(self: Box<Self>) -> Box<Self> {
        let mut current = self;
        loop {
            let (result, _, _, something_exploded) = current.explode(0);
            current = result;
            if something_exploded {
                continue;
            }
            let (result, something_splitted) = current.split();
            current = result;
            if something_splitted {
                continue;
            }

            break;
        }
        current
    }

    fn magnitude(&self) -> u32 {
        match self {
            Number(n) => *n,
            Pair(left, right) => left.magnitude()*3 + right.magnitude()*2
        }
    }

    fn add(left: Box<Self>, right: Box<Self>) -> Box<Self> {
        let tmp = Box::new(Pair(left, right));
        tmp.reduce()
    }

    fn add_list(iter: impl Iterator<Item=Box<Self>>) -> Option<Box<Self>> {
        iter.reduce(Self::add)
    }
}

type Input<'a> = Peekable<Copied<slice::Iter<'a, u8>>>;

fn expect(input: &mut Input, expected: u8) -> Result<()> {
    let next = input.next().ok_or(Error::EmptyIterator)?;
    if next != expected {
        Err(Error::General(format!("Unexpected input '{}', expected '{}'", next as char, expected as char)))
    } else {
        Ok(())
    }
}

fn consume(input: &mut Input) -> Result<()> {
    let _ = input.next().ok_or(Error::EmptyIterator)?;
    Ok(())
}

fn parse(input: &mut Input) -> Result<SnailfishNumber> {
    let current = input.peek().copied();
    match current {
        Some(b'[') => {
            expect(input, b'[')?;
            let left = parse(input)?;
            expect(input, b',')?;
            let right = parse(input)?;
            expect(input, b']')?;
            Ok(SnailfishNumber::Pair(Box::new(left), Box::new(right)))
        }
        Some(n @ b'0'..=b'9') => {
            consume(input)?;
            Ok(SnailfishNumber::Number((n-b'0') as u32))
        }
        Some(ch @ _) => {
            Err(Error::General(format!("Unexpected input '{}', expected pair or number", ch as char)))
        }
        None => {
            Err(Error::EmptyIterator)
        }
    }
}

fn parse_str(input: &str) -> Result<SnailfishNumber> {
    let mut input: Input = input.as_bytes().iter().copied().peekable();

    parse(&mut input)
}

pub fn main() -> Result<()> {

    assert_eq!(parse_str("[[[[[9,8],1],2],3],4]")?.into_box().reduce(), parse_str("[[[[0,9],2],3],4]")?.into_box());
    assert_eq!(parse_str("[7,[6,[5,[4,[3,2]]]]]")?.into_box().reduce(), parse_str("[7,[6,[5,[7,0]]]]")?.into_box());
    assert_eq!(parse_str("[[6,[5,[4,[3,2]]]],1]")?.into_box().reduce(), parse_str("[[6,[5,[7,0]]],3]")?.into_box());
    assert_eq!(parse_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")?.into_box().reduce(), parse_str("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")?.into_box());

    assert_eq!(SnailfishNumber::add(parse_str("[[[[4,3],4],4],[7,[[8,4],9]]]")?.into_box(), parse_str("[1,1]")?.into_box()), parse_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")?.into_box());


    let expected = parse_str("[[[[1,1],[2,2]],[3,3]],[4,4]]")?.into_box();
    let actual = SnailfishNumber::add_list([
        "[1,1]",
        "[2,2]",
        "[3,3]",
        "[4,4]",
    ].iter().map(|n| parse_str(n).unwrap().into_box())).unwrap();

assert_eq!(actual, expected);


    let example_number = SnailfishNumber::add_list(include_str!("../../data/a18_example.txt").lines().map(|line| parse_str(line).unwrap().into_box())).unwrap();

    assert_eq!(example_number, parse_str("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")?.into_box());
    assert_eq!(4140, example_number.magnitude());

    let numbers = include_str!("../../data/a18_input.txt").lines().map(|line| Ok(parse_str(line)?.into_box())).collect::<Result<Vec<_>>>()?;
    let number = SnailfishNumber::add_list(numbers.clone().into_iter()).unwrap();

    // println!("{}", number);
    println!("Part {}", number.magnitude());



    Ok(())
}