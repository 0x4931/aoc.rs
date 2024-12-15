use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    NorthSouthPipe,
    EastWestPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthWestPipe,
    SouthEastPipe,
    Ground,
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: isize,
    col: isize,
}

fn compute_adjacency_list(map: &Vec<Vec<Tile>>) -> HashMap<Coord, Vec<Coord>> {
    let mut adjacency_list = HashMap::new();
    let n = map.len();
    let m = map[0].len();

    // Create unchecked adjacency list
    for row in 0..n as isize {
        for col in 0..m as isize {
            let coord = Coord { row, col };
            let north = Coord { row: row - 1, col };
            let south = Coord { row: row + 1, col };
            let east = Coord { row, col: col + 1 };
            let west = Coord { row, col: col - 1 };

            let adjacent = adjacency_list.entry(coord).or_insert_with(|| Vec::new());

            match map[row as usize][col as usize] {
                Tile::NorthSouthPipe => {
                    adjacent.push(north);
                    adjacent.push(south);
                }
                Tile::EastWestPipe => {
                    adjacent.push(east);
                    adjacent.push(west);
                }
                Tile::NorthEastPipe => {
                    adjacent.push(north);
                    adjacent.push(east);
                }
                Tile::NorthWestPipe => {
                    adjacent.push(north);
                    adjacent.push(west);
                }
                Tile::SouthWestPipe => {
                    adjacent.push(south);
                    adjacent.push(west);
                }
                Tile::SouthEastPipe => {
                    adjacent.push(south);
                    adjacent.push(east);
                }
                Tile::Ground => (),
                Tile::Start => {
                    adjacent.push(north);
                    adjacent.push(south);
                    adjacent.push(east);
                    adjacent.push(west);
                }
            }
        }
    }

    // Cleanup adjacency list (remove singly linked connections, pipes must be doubly connected)
    let adjacency_list_readable = adjacency_list.clone(); // hacky solution to borrow twice

    for (coord, adjacent) in adjacency_list.iter_mut() {
        adjacent.retain(|next| {
            if let Some(adjacent) = adjacency_list_readable.get(next) {
                if adjacent.contains(coord) {
                    return true;
                }
            }

            false
        });
    }

    adjacency_list
}

fn find_cycle(map: &Vec<Vec<Tile>>, root: Coord) -> Vec<Coord> {
    let adjacency_list = compute_adjacency_list(map);
    let mut visited = HashSet::new();
    let mut visit_stack = Vec::new();

    find_cycle_dfs(&adjacency_list, root, &mut visited, &mut visit_stack, root);

    visit_stack
}

fn find_cycle_dfs(
    adjacency_list: &HashMap<Coord, Vec<Coord>>,
    node: Coord,
    visited: &mut HashSet<Coord>,
    visit_stack: &mut Vec<Coord>,
    parent: Coord,
) -> bool {
    visited.insert(node);
    visit_stack.push(node);

    if let Some(adjacent) = adjacency_list.get(&node) {
        for next in adjacent.iter() {
            if !visited.contains(next) {
                if find_cycle_dfs(adjacency_list, *next, visited, visit_stack, node) {
                    return true;
                }
            } else {
                if *next != parent {
                    return true;
                }
            }
        }
    }

    visit_stack.pop();
    false
}

fn reveal_start_tile(cycle: &Vec<Coord>) -> Tile {
    let start = cycle[0];
    let first = cycle[1];
    let last = cycle[cycle.len() - 1];

    let first_delta = Coord {
        row: first.row - start.row,
        col: first.col - start.col,
    };

    let last_delta = Coord {
        row: last.row - start.row,
        col: last.col - start.col,
    };

    let north = Coord { row: -1, col: 0 };
    let south = Coord { row: 1, col: 0 };
    let east = Coord { row: 0, col: 1 };
    let west = Coord { row: 0, col: -1 };

    let check = |dir1: Coord, dir2: Coord| {
        first_delta == dir1 && last_delta == dir2 || first_delta == dir2 && last_delta == dir1
    };

    if check(north, south) {
        return Tile::NorthSouthPipe;
    }
    if check(east, west) {
        return Tile::EastWestPipe;
    }
    if check(north, east) {
        return Tile::NorthEastPipe;
    }
    if check(north, west) {
        return Tile::NorthWestPipe;
    }
    if check(south, east) {
        return Tile::SouthEastPipe;
    }
    if check(south, west) {
        return Tile::SouthWestPipe;
    }

    unreachable!()
}

fn part_one(map: &Vec<Vec<Tile>>, start: Coord) {
    let cycle = find_cycle(map, start);
    let steps = cycle.len() / 2;

    println!("{}", steps);
}

fn part_two(map: &Vec<Vec<Tile>>, start: Coord) {
    let mut map = map.clone();
    let n = map.len();
    let m = map[0].len();
    let cycle = find_cycle(&map, start);

    // Fix start tile
    map[start.row as usize][start.col as usize] = reveal_start_tile(&cycle);

    // Cache edges in each row
    let mut edges_in_row = vec![vec![]; n];

    for node in cycle.iter() {
        let row = node.row as usize;
        let col = node.col as usize;
        edges_in_row[row].push(col);
    }

    for row in 0..n {
        edges_in_row[row].sort();
    }

    // "Raycasting"
    let mut count = 0;

    for row in 0..n {
        for col in 0..m {
            let Err(edges_on_left) = edges_in_row[row].binary_search(&col) else {
                continue;
            };

            use Tile::*;

            let mut is_inside = false;
            let mut curr_edge: Tile;
            let mut prev_edge: Tile = Ground;

            for edge_idx in 0..edges_on_left {
                curr_edge = map[row][edges_in_row[row][edge_idx]];

                match curr_edge {
                    NorthSouthPipe => is_inside = !is_inside,
                    NorthWestPipe if prev_edge == SouthEastPipe => is_inside = !is_inside,
                    SouthWestPipe if prev_edge == NorthEastPipe => is_inside = !is_inside,
                    _ => (),
                }

                // Don't store the sides, we want edges
                if curr_edge != EastWestPipe {
                    prev_edge = curr_edge;
                }
            }

            if is_inside {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2023, 10)?;

    let mut map: Vec<Vec<Tile>> = Default::default();

    for line in input.lines() {
        let tiles = line
            .chars()
            .map(|c| match c {
                '|' => Tile::NorthSouthPipe,
                '-' => Tile::EastWestPipe,
                'L' => Tile::NorthEastPipe,
                'J' => Tile::NorthWestPipe,
                '7' => Tile::SouthWestPipe,
                'F' => Tile::SouthEastPipe,
                '.' => Tile::Ground,
                'S' => Tile::Start,
                _ => unreachable!(),
            })
            .collect();
        map.push(tiles);
    }

    let start = 'start: {
        let n = map.len();
        let m = map[0].len();

        for row in 0..n {
            for col in 0..m {
                if map[row][col] == Tile::Start {
                    break 'start Coord {
                        row: row as isize,
                        col: col as isize,
                    };
                }
            }
        }

        unreachable!()
    };

    part_one(&map, start);
    part_two(&map, start);

    Ok(())
}
