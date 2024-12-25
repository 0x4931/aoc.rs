use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
    Box,
    Robot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: isize,
    col: isize,
}

fn find_robot(map: &Vec<Vec<Tile>>) -> Coord {
    for (row, tiles) in map.iter().enumerate() {
        for (col, tile) in tiles.iter().enumerate() {
            if tile == &Tile::Robot {
                return Coord {
                    row: row as isize,
                    col: col as isize,
                };
            }
        }
    }
    unreachable!()
}

fn part_one(map: &Vec<Vec<Tile>>, moves: &Vec<Move>) {
    let mut map = map.clone();

    let mut robot = find_robot(&map);
    let mut move_robot = |dir: Coord| {
        let boxes_to_move = {
            let mut count = 0;
            let mut row = robot.row;
            let mut col = robot.col;

            loop {
                row += dir.row;
                col += dir.col;
                if map[row as usize][col as usize] != Tile::Box {
                    break;
                }
                count += 1;
            }

            count
        };

        let is_path_blocked = {
            let row = robot.row + dir.row * (boxes_to_move + 1);
            let col = robot.col + dir.col * (boxes_to_move + 1);
            map[row as usize][col as usize] == Tile::Wall
        };

        if is_path_blocked {
            return;
        }

        let mut row = robot.row;
        let mut col = robot.col;
        map[row as usize][col as usize] = Tile::Empty;

        row += dir.row;
        col += dir.col;
        map[row as usize][col as usize] = Tile::Robot;

        if boxes_to_move > 0 {
            row += dir.row * boxes_to_move;
            col += dir.col * boxes_to_move;
            map[row as usize][col as usize] = Tile::Box;
        }

        robot.row += dir.row;
        robot.col += dir.col;
    };

    for mov in moves.iter() {
        match mov {
            Move::Up => move_robot(Coord { row: -1, col: 0 }),
            Move::Down => move_robot(Coord { row: 1, col: 0 }),
            Move::Left => move_robot(Coord { row: 0, col: -1 }),
            Move::Right => move_robot(Coord { row: 0, col: 1 }),
        }
    }

    let mut sum = 0;
    for (row, tiles) in map.iter().enumerate() {
        for (col, tile) in tiles.iter().enumerate() {
            if tile == &Tile::Box {
                sum += 100 * row + col;
            }
        }
    }

    println!("{}", sum);
}

fn part_two(map: &Vec<Vec<Tile>>, moves: &Vec<Move>) {
    let mut robot = find_robot(map);
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();

    // Expand map
    robot.col *= 2;

    for (row, tiles) in map.iter().enumerate() {
        for (col, tile) in tiles.iter().enumerate() {
            let coord = Coord {
                row: row as isize,
                col: col as isize * 2,
            };
            match tile {
                Tile::Wall => {
                    walls.insert(coord);
                }
                Tile::Box => {
                    boxes.insert(coord);
                }
                _ => (),
            }
        }
    }

    // Move robot
    let mut move_robot = |dir: Coord| {
        let mut boxes_to_move = HashSet::new();
        let mut coords_to_move = Vec::new();
        coords_to_move.push(Coord {
            row: robot.row + dir.row,
            col: robot.col + dir.col,
        });

        while !coords_to_move.is_empty() {
            // Also check coordinate to the left to account for expanded tiles
            let coord = coords_to_move.pop().unwrap();
            let coord2 = Coord {
                row: coord.row,
                col: coord.col - 1,
            };

            if boxes_to_move.contains(&coord) || boxes_to_move.contains(&coord2) {
                continue;
            }

            // Check if the path is blocked
            let is_wall = walls.contains(&coord) || walls.contains(&coord2);
            if is_wall {
                return;
            }

            let is_box_right = boxes.contains(&coord);
            let is_box_left = boxes.contains(&coord2);
            let is_box = is_box_left || is_box_right;
            if is_box {
                let box_left = if is_box_left { coord2 } else { coord };
                let box_right = Coord {
                    row: box_left.row,
                    col: box_left.col + 1,
                };

                boxes_to_move.insert(box_left);

                coords_to_move.push(Coord {
                    row: box_left.row + dir.row,
                    col: box_left.col + dir.col,
                });

                coords_to_move.push(Coord {
                    row: box_right.row + dir.row,
                    col: box_right.col + dir.col,
                });
            }
        }

        // Move all boxes
        let boxes_to_move = Vec::from_iter(boxes_to_move);
        for box_to_move in boxes_to_move.iter() {
            boxes.remove(box_to_move);
        }
        for box_to_move in boxes_to_move.iter() {
            boxes.insert(Coord {
                row: box_to_move.row + dir.row,
                col: box_to_move.col + dir.col,
            });
        }

        // Move robot
        robot.row += dir.row;
        robot.col += dir.col;
    };

    for mov in moves.iter() {
        match mov {
            Move::Up => move_robot(Coord { row: -1, col: 0 }),
            Move::Down => move_robot(Coord { row: 1, col: 0 }),
            Move::Left => move_robot(Coord { row: 0, col: -1 }),
            Move::Right => move_robot(Coord { row: 0, col: 1 }),
        }
    }

    // Calculate GPS sum
    let mut sum = 0;
    for bx in boxes.iter() {
        // bro since when was box a reserved name
        sum += 100 * bx.row + bx.col;
    }

    println!("{}", sum);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 15)?;

    let (map, moves) = input.split_once("\n\n").unwrap();

    let map: Vec<Vec<Tile>> = map
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    'O' => Tile::Box,
                    '@' => Tile::Robot,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let moves: Vec<Move> = moves
        .lines()
        .map(|line| line.chars())
        .flatten()
        .map(|c| match c {
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            '>' => Move::Right,
            _ => unreachable!(),
        })
        .collect();

    part_one(&map, &moves);
    part_two(&map, &moves);

    Ok(())
}
