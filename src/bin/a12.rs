use std::collections::HashMap;
use adventofcode2021::prelude::*;

struct Edge {
    from_id: usize,
    from_small: bool,
    to_id: usize,
    to_small: bool,
}

fn count_paths(edges: &[(usize, usize)], dict: &HashMap<usize, &str>, small_caves: u64, current: usize, end: usize, visited: u64, len: usize, found: &mut usize) {
    eprintln!("Current {} ({})", dict.get(&current).unwrap_or(&""), *found);
    // eprintln!("{:016b} ({})", visited, len);
    if current == end {
        *found += 1
    } else {
        let visited = visited | (1 << current);
        for (from, to) in edges.iter().copied() {
            if from == current {
                let is_small = small_caves & (1<<to) != 0;
                let is_visited = visited & (1<<to) != 0;
                if !is_small || !is_visited {
                    count_paths(edges, dict, small_caves, to, end, visited, len+1, found);
                }
            }
        }
    }
}

pub fn main() -> Result<()> {
    let mut dict = HashMap::new();
    let mut edges = include_str!("../../data/a12_example3.txt").lines().map(|line| {
        let mut split = line.split("-");
        let from = split.next().ok_or(Error::EmptyIterator)?;
        let next_id = dict.len();
        let from_id = *dict.entry(from).or_insert(next_id);
        let to = split.next().ok_or(Error::EmptyIterator)?;
        let next_id = dict.len();
        let to_id = *dict.entry(to).or_insert(next_id);

        Ok((from_id, to_id))
    }).collect::<Result<Vec<(usize, usize)>>>()?;

    let mut reverse_edges = edges.iter().map(|(from, to)| (*to, *from)).collect::<Vec<_>>();
    edges.extend(&reverse_edges);

    if dict.len() > 64 {
        return Err(Error::General("too many edges".into()));
    }

    let dict_values = dict.iter().map(|(key, value)| (*value, *key)).collect::<HashMap<_, _>>();

    let start_id = *dict.get("start").ok_or(Error::General("Could not find start id".into()))?;
    let end_id = *dict.get("end").ok_or(Error::General("Could not find end id".into()))?;

    dbg!(&edges, start_id, end_id);

    let small_mask = dict.iter().filter_map(|(key, value)| {
        match key.chars().next() {
         Some(ch) if ch.is_ascii_lowercase() => Some(*value),
            _ => None

        }
    }).fold(0_u64, |a, id| a | (1<<id));

    let mut part1= 0_usize;
        count_paths(&edges, &dict_values, small_mask, start_id, end_id, 0_u64, 0, &mut part1);

    println!("Part1: {}", part1);

    Ok(())
}