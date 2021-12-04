use adventofcode2021::prelude::*;

fn read_board<'a>(lines: impl Iterator<Item=&'a str>) -> [[i32; 5]; 5] {
    let mut result = [[0; 5]; 5];
    result.iter_mut().zip(lines).for_each(|(row, line)| {
        row.iter_mut().zip(line.split_ascii_whitespace()).for_each(|(r, n)| *r = i32::from_str(n).unwrap());
    });
    result
}

pub fn main() -> Result<()> {
    let data = include_str!("../../data/a4_input.txt");
    let mut lines = data.lines().peekable();
    let numbers = lines.next().ok_or(Error::EmptyIterator)?.split(",").map(|n| Ok(i32::from_str(n)?)).collect::<Result<Vec<i32>>>()?;
    let _empty_line = lines.next().ok_or(Error::EmptyIterator)?;

    let mut boards = vec![];
    while lines.peek().is_some() {
        boards.push(read_board(&mut lines));
        if let Some(line) = lines.peek() {
            if line.is_empty() {
                lines.next();
            } else {
                return Err(Error::General(format!("Unexpected non-empty line: {}", line)));
            }
        }
    }

    let mut markers = vec![[[false; 5]; 5]; boards.len()];

    for n in numbers {
        for (board, marker) in boards.iter_mut().zip(markers.iter_mut()) {
            let mut marked = false;
            for row in 0..5 {
                for col in 0..5 {
                    if board[row][col] == n {
                        marker[row][col] = true;
                        marked = true;
                    }
                }
            }
            if marked {
                let horz_win = (0..5).any(|row| marker[row].iter().all(|col| *col));
                let vert_win = (0..5).any(|col| marker.iter().all(|m| m[col as usize]));
                if horz_win || vert_win {
                    let sum_unmarked: i32 = board.iter().zip(marker.iter()).flat_map(|(row, m)| {
                        row.iter().zip(m.iter()).filter(|(_, m)| !**m).map(|(x, _)| *x)
                    }).sum();
                    let score = n * sum_unmarked;

                    // answer for part1 is first printed score, part2 is the last printed score
                    println!("Score = {}", score);

                    // invalidate board, removing the board would be nicer
                    board.fill([-1; 5]);
                }
            }
        }
    }

    Ok(())
}