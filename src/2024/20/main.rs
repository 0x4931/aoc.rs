use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coord {
    row: isize,
    col: isize,
}

fn find_start_and_end(map: &Vec<Vec<Tile>>) -> (Coord, Coord) {
    let mut start: Option<Coord> = None;
    let mut end: Option<Coord> = None;

    for (row, tiles) in map.iter().enumerate() {
        for (col, tile) in tiles.iter().enumerate() {
            let coord = Some(Coord {
                row: row as isize,
                col: col as isize,
            });

            match tile {
                Tile::Start => start = coord,
                Tile::End => end = coord,
                _ => (),
            }
        }
    }

    let start = start.expect("start tile exists");
    let end = end.expect("end tile exists");
    (start, end)
}

fn compute_reverse_dijkstras(map: &Vec<Vec<Tile>>) -> HashMap<Coord, i64> {
    // Intuition: Reverse Dijkstra's Algorithm
    // - Since there's only a single path from S>E without cheating, the paths match after cheating
    // - For each tile, mark the number of moves until E is reached
    // - When cheating, just take the delta between the two tiles as the time saved

    let n = map.len();
    let m = map[0].len();
    let (_start, end) = find_start_and_end(map);

    let mut moves_until = HashMap::new();
    moves_until.insert(end, 0);

    let mut to_process = BinaryHeap::new();
    to_process.push(Reverse((0, end)));

    while !to_process.is_empty() {
        let (moves, coord) = to_process.pop().unwrap().0;

        let dirs = &[
            Coord { row: 1, col: 0 },
            Coord { row: -1, col: 0 },
            Coord { row: 0, col: 1 },
            Coord { row: 0, col: -1 },
        ][..];
        for dir in dirs {
            let moves = moves + 1;
            let coord = Coord {
                row: coord.row + dir.row,
                col: coord.col + dir.col,
            };

            if (0..n as isize).contains(&coord.row)
                && (0..m as isize).contains(&coord.col)
                && map[coord.row as usize][coord.col as usize] != Tile::Wall
            {
                let next_moves = moves_until.entry(coord).or_insert(i64::MAX);
                if moves < *next_moves {
                    *next_moves = moves;

                    to_process.push(Reverse((moves, coord)));
                }
            }
        }
    }

    moves_until
}

fn part_one(map: &Vec<Vec<Tile>>) {
    let moves_until = compute_reverse_dijkstras(map);

    let mut count = 0;
    for (coord, moves) in moves_until.iter() {
        let dirs = &[
            Coord { row: 2, col: 0 },
            Coord { row: -2, col: 0 },
            Coord { row: 0, col: 2 },
            Coord { row: 0, col: -2 },
            Coord { row: 1, col: 1 },
            Coord { row: 1, col: -1 },
            Coord { row: -1, col: 1 },
            Coord { row: -1, col: -1 },
        ][..];
        for dir in dirs {
            let next_coord = Coord {
                row: coord.row + dir.row,
                col: coord.col + dir.col,
            };

            if let Some(next_moves) = moves_until.get(&next_coord) {
                let moves_saved = moves - (next_moves + 2);
                if moves_saved >= 100 {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}

fn part_two(map: &Vec<Vec<Tile>>) {
    const CHEAT_MAX: i64 = 20;

    let moves_until = compute_reverse_dijkstras(map);

    let mut count = 0;
    for (coord, moves) in moves_until.iter() {
        for (next_coord, next_moves) in moves_until.iter() {
            let moves_inbetween = {
                let delta_rows = (next_coord.row - coord.row).abs() as i64;
                let delta_cols = (next_coord.col - coord.col).abs() as i64;
                delta_rows + delta_cols
            };

            if moves_inbetween <= CHEAT_MAX {
                let moves_saved = moves - (next_moves + moves_inbetween);
                if moves_saved >= 100 {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 20)?;

    let map: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    'S' => Tile::Start,
                    'E' => Tile::End,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    part_one(&map);
    part_two(&map);

    Ok(())
}
