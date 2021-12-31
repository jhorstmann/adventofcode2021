use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use adventofcode2021::prelude::*;


/*


7+9+60+60+500+600+8000+9000
#############
#...........#
###C#C#A#B###
  #D#D#B#A#
  #########

#############
#...B......A#
###C#C#.#B###
  #D#D#.#A#
  #########
+5+50

#############
#AB.........#
###C#C#.#B###
  #D#D#.#A#
  #########
+70

#############
#AB.........#
###.#.#C#B###
  #D#D#C#A#
  #########
+500+600

#############
#AB.B.....A.#
###.#.#C#.###
  #D#D#C#.#
  #########
+60+3

#############
#AB.B.....A.#
###.#.#C#.###
  #D#.#C#D#
  #########
+8000

#############
#AB.......A.#
###.#.#C#.###
  #D#B#C#D#
  #########
+30

#############
#AB.......A.#
###.#.#C#D###
  #.#B#C#D#
  #########
+9000

#############
#A........A.#
###.#B#C#D###
  #.#B#C#D#
  #########
+40

#############
#...........#
###.#B#C#D###
  #.#B#C#D#
  #########
+4+8




18322 not right
18326 not right
18327 not right
18318 not right
17395 too low
18342 too high
18382 too high
18386 too high



part2

#############
#...........#
###C#C#A#B###
  #D#C#B#A#
  #D#B#A#C#
  #D#D#B#A#
  #########

7+60+8+70+700+700+700+40+5000+50+80+80+60+4+600+5+9000+11000+11000+11000+5+5+9+9


500+500+40+10000+50+80+4+60+4+70+900+800+800



50183 too high
50192 too high
*/

pub fn main() -> Result<()> {

    #[rustfmt::skip]
    let _nodes = [
        00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10,
                11,     12,     13,     14,
                15,     16,     17,     18,
                19,     20,     21,     22,
                23,     24,     25,     26,
    ];

    let room1 = vec![23, 19, 15, 11];
    let room2 = vec![24, 20, 16, 12];
    let room3 = vec![25, 21, 17, 13];
    let room4 = vec![26, 22, 18, 14];

    let edges: Vec<(usize, usize, Bitmap64)> = {
        let mut edges = vec![];

        let mut edge = |room: &[usize], path: Vec<usize>| {
            for i in 0..4 {
                let from = room[i];
                let to = *path.last().expect("non-empty path");
                let path = room[i..].iter().chain(path.iter()).copied().collect::<Bitmap64>();
                edges.push((from, to, path.unset(from)));
                edges.push((to, from, path.unset(to)));
            }
        };

        edge(&room1, vec![2, 1, 0]);
        edge(&room1, vec![2, 1]);
        edge(&room1, vec![2, 3]);
        edge(&room1, vec![2, 3, 4, 5]);
        edge(&room1, vec![2, 3, 4, 5, 6, 7]);
        edge(&room1, vec![2, 3, 4, 5, 6, 7, 8, 9]);
        edge(&room1, vec![2, 3, 4, 5, 6, 7, 8, 9, 10]);

        edge(&room2, vec![4, 3, 2, 1, 0]);
        edge(&room2, vec![4, 3, 2, 1]);
        edge(&room2, vec![4, 3]);
        edge(&room2, vec![4, 5]);
        edge(&room2, vec![4, 5, 6, 7]);
        edge(&room2, vec![4, 5, 6, 7, 8, 9]);
        edge(&room2, vec![4, 5, 6, 7, 8, 9, 10]);

        edge(&room3, vec![6, 5, 4, 3, 2, 1, 0]);
        edge(&room3, vec![6, 5, 4, 3, 2, 1]);
        edge(&room3, vec![6, 5, 4, 3]);
        edge(&room3, vec![6, 5]);
        edge(&room3, vec![6, 7]);
        edge(&room3, vec![6, 7, 8, 9]);
        edge(&room3, vec![6, 7, 8, 9, 10]);

        edge(&room4, vec![8, 7, 6, 5, 4, 3, 2, 1, 0]);
        edge(&room4, vec![8, 7, 6, 5, 4, 3, 2, 1]);
        edge(&room4, vec![8, 7, 6, 5, 4, 3]);
        edge(&room4, vec![8, 7, 6, 5]);
        edge(&room4, vec![8, 7]);
        edge(&room4, vec![8, 9]);
        edge(&room4, vec![8, 9, 10]);

        edges
    };

    // let rooms_mask = rooms.iter().flatten().copied().collect::<Bitmap64>();
    let rooms_mask = [room1, room2, room3, room4].iter().map(|r| r.iter().copied().collect::<Bitmap64>()).collect::<Vec<_>>();
    let hallway_mask = [0_usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter().collect::<Bitmap64>();
    let entry_mask = [2_usize,4,6,8].into_iter().collect::<Bitmap64>();

    const A: u8 = 0;
    const B: u8 = 1;
    const C: u8 = 2;
    const D: u8 = 3;

    const COST: [usize; 4] = [1, 10, 100, 1000];

    #[rustfmt::skip]
    let _goal: Vec<(u8, u8)> = vec![
        (23, A), (19, A), (15, A), (11, A),
        (24, B), (20, B), (16, B), (12, B),
        (25, C), (21, C), (17, C), (13, C),
        (26, D), (22, D), (18, D), (14, D),
    ];

    let goal = [
        Bitmap64::default().set(23).set(19).set(15).set(11).as_u64(),
        Bitmap64::default().set(24).set(20).set(16).set(12).as_u64(),
        Bitmap64::default().set(25).set(21).set(17).set(13).as_u64(),
        Bitmap64::default().set(26).set(22).set(18).set(14).as_u64(),
    ];

    #[rustfmt::skip]
    let mut _example: Vec<(u8, u8)> = vec![
        (23, A), (19, D), (15, D), (11, B),
        (24, D), (20, B), (16, C), (12, C),
        (25, C), (21, A), (17, B), (13, B),
        (26, A), (22, C), (18, A), (14, D),
    ];

    #[rustfmt::skip]
    let input: Vec<(u8, u8)> = vec![
        (23, D), (19, D), (15, D), (11, C),
        (24, D), (20, B), (16, C), (12, C),
        (25, B), (21, A), (17, B), (13, A),
        (26, A), (22, C), (18, A), (14, B),
    ];

    fn board_to_bitmap(board: &[(u8, u8)], color: u8) -> Bitmap64 {
        board.iter().filter(|(_, c)| *c == color).map(|(p, _)| *p as usize).collect::<Bitmap64>()
    }

    let input = [
        board_to_bitmap(&input, A).as_u64(),
        board_to_bitmap(&input, B).as_u64(),
        board_to_bitmap(&input, C).as_u64(),
        board_to_bitmap(&input, D).as_u64(),
    ];

    let mut queue = BinaryHeap::new();

    queue.push((Reverse(0_usize), input, 0_u64));

    let mut i = 0_usize;

    let mut visited = HashSet::new();

    while let Some((Reverse(cost), board, room_was_dest)) = queue.pop() {
        if i % 20_000 == 0 {
            eprintln!("queue len {}, min cost {}", queue.len(), cost);
        }
        i += 1;

        if board == goal {
            println!("Cost: {}", cost);
            break;
        }

        if !visited.insert(board) {
            continue;
        }

        let occupied = board.iter().fold(Bitmap64::default(), |a, x| a.or(&Bitmap64::from(*x)));

        for (color, mask) in board.iter().enumerate() {
            for pos in Bitmap64::from(*mask).iter() {
                let is_hallway = hallway_mask.is_set(pos);
                let possible_destinations = if is_hallway {
                    rooms_mask[color].and_not(&Bitmap64::from(room_was_dest))
                } else if Bitmap64::from(room_was_dest).is_set(pos) {
                    continue;
                } else {
                    hallway_mask.and_not(&entry_mask)
                };
                let possible_destinations = possible_destinations.and_not(&occupied);

                for dest in possible_destinations.iter() {
                    for (edge_from, edge_to, edge_path) in edges.iter() {
                        // eprintln!("Move {} to {} ({} -> {})", color, if is_hallway { "room" } else { "hall" }, pos, dest);
                        if *edge_from == pos && *edge_to == dest {
                            let is_free = edge_path.and(&occupied).is_empty();
                            if is_free {
                                let path_cost = COST[color] * edge_path.len();

                                let new_cost = cost + path_cost;

                                // eprintln!("Move {} to {} ({} -> {}), cost {}, queued {}", color, if is_hallway { "room" } else { "hall" }, pos, dest, new_cost, queue.len());

                                let new_room_was_dest = if is_hallway {
                                    Bitmap64::from(room_was_dest).set(dest).as_u64()
                                } else {
                                    room_was_dest
                                };

                                let mut new_board = board.clone();
                                new_board[color] = Bitmap64::from(new_board[color]).unset(pos).set(dest).as_u64();

                                queue.push((Reverse(new_cost), new_board, new_room_was_dest));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}