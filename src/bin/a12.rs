use std::collections::HashMap;
use adventofcode2021::prelude::*;

fn count_paths(edges: &[(usize, usize)], small_caves: Bitmap64, current: usize, end: usize, visited: Bitmap64, found: &mut usize) {
    if current == end {
        *found += 1
    } else {
        let visited = visited.set(current);
        for (from, to) in edges.iter().copied() {
            if from == current {
                let is_small = small_caves.is_set(to);
                let is_visited = visited.is_set(to);
                if !is_small || !is_visited {
                    count_paths(edges, small_caves, to, end, visited, found);
                }
            }
        }
    }
}

fn count_paths_part2(edges: &[(usize, usize)], small_caves: Bitmap64, current: usize, start: usize, end: usize, visited: Bitmap64, visited_small_twice: bool, found: &mut usize) {
    if current == end {
        *found += 1
    } else {
        let (visited, visited_small_twice) = if visited.is_set(current) {
            if small_caves.is_set(current) {
                (visited, true)
            } else {
                (visited, visited_small_twice)
            }
        } else {
            (visited.set(current), visited_small_twice)
        };
        for (from, to) in edges.iter().copied() {
            if from == current {
                let is_small = small_caves.is_set(to);
                let is_visited = visited.is_set(to);
                if !is_visited || !is_small || !(visited_small_twice || to == start) {
                    count_paths_part2(edges, small_caves, to, start, end, visited, visited_small_twice, found);
                }
            }
        }
    }
}

pub fn main() -> Result<()> {
    let mut dict = HashMap::new();
    let edges = include_str!("../../data/a12_input.txt").lines().map(|line| {
        let mut split = line.split("-");
        let from = split.next().ok_or(Error::EmptyIterator)?;
        let next_id = dict.len();
        let from_id = *dict.entry(from).or_insert(next_id);
        let to = split.next().ok_or(Error::EmptyIterator)?;
        let next_id = dict.len();
        let to_id = *dict.entry(to).or_insert(next_id);

        Ok((from_id, to_id))
    }).collect::<Result<Vec<(usize, usize)>>>()?;

    let edges = edges.iter()
        .flat_map(|(from, to)| [(*from, *to), (*to, *from)].into_iter())
        .collect::<Vec<_>>();

    if dict.len() > 64 {
        return Err(Error::General("too many edges".into()));
    }

    let _dict_values = dict.iter().map(|(key, value)| (*value, *key)).collect::<HashMap<_, _>>();

    let start_id = *dict.get("start").ok_or(Error::General("Could not find start id".into()))?;
    let end_id = *dict.get("end").ok_or(Error::General("Could not find end id".into()))?;

    let small_mask = dict.iter().filter_map(|(key, value)| {
        match key.chars().next() {
            Some(ch) if ch.is_ascii_lowercase() => Some(*value),
            _ => None
        }
    }).fold(Bitmap64::default(), |a, id| a.set(id));

    let mut part1 = 0_usize;
    count_paths(&edges, small_mask, start_id, end_id, Bitmap64::default(), &mut part1);

    println!("Part1: {}", part1);

    let mut part2 = 0_usize;
    count_paths_part2(&edges, small_mask, start_id, start_id, end_id, Bitmap64::default(), false, &mut part2);

    println!("Part2: {}", part2);

    Ok(())
}