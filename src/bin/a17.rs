use adventofcode2021::prelude::*;

#[derive(Debug,Clone,Copy,Default)]
struct Point {
    x: i64,
    y: i64,
}

fn simulate(target: &(Point, Point), min_vx: i64, max_vx: i64, min_vy: i64, max_vy: i64) -> Option<i64> {
    let mut max_y: Option<i64> = None;

    for initial_vx in min_vx..max_vx+1 {
        for initial_vy in min_vy..max_vy+1 {
            let mut vx = initial_vx;
            let mut vy = initial_vy;
            let mut x = 0;
            let mut y = 0;
            let mut current_max_y = 0;

            loop {
                current_max_y = current_max_y.max(y);

                if x >= target.0.x && x <= target.1.x && y >= target.0.y && y <= target.1.y {
                    max_y = max_y.map(|m| m.max(current_max_y)).or(Some(current_max_y));
                    dbg!(initial_vx, initial_vy, current_max_y, &max_y);
                    break;
                }

                if x > target.1.x || y < target.0.y {
                    break;
                }


                x += vx;
                y += vy;
                vx = (vx - 1).max(0);
                vy -= 1;
            }

            // dbg!(initial_vx, initial_vy, current_max_y, max_y);
        }
    }


    max_y
}

pub fn main() -> Result<()> {
    let example = "target area: x=20..30, y=-10..-5";
    let example = (Point { x: 20, y: -10}, Point {x:30, y:-5});
    let input = "target area: x=195..238, y=-93..-67";
    let input = (Point { x: 195, y: -93}, Point {x:238, y:-67});

    let area = input;

    let min_vx = 1;
    let max_vx = (area.1.x)/2;
    let min_vy = (area.0.y);
    let max_vy = 50_000;

    dbg!(min_vx, max_vx, min_vy, max_vy);

    let part1 = simulate(&area, min_vx, max_vx, min_vy, max_vy);
    // let part1 = simulate(&area, 6, 6, 9, 9);

    dbg!(part1);


    Ok(())
}