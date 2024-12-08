use std::collections::{BTreeSet, HashSet};

fn part_one(obstacles: &Vec<Vec<bool>>, start_pos: &(usize, usize)) {
    let n = obstacles.len();
    let m = obstacles[0].len();

    let mut position = (start_pos.0 as isize, start_pos.1 as isize);
    let mut direction = (-1, 0);
    let mut visited = vec![vec![false; m]; n];

    while 0 < position.0
        && position.0 < (n - 1) as isize
        && 0 < position.1
        && position.1 < (m - 1) as isize
    {
        visited[position.0 as usize][position.1 as usize] = true;

        let next_pos = (position.0 + direction.0, position.1 + direction.1);
        if obstacles[next_pos.0 as usize][next_pos.1 as usize] {
            direction = (direction.1, -direction.0);
        } else {
            position = next_pos;
        }
    }

    let visit_count = visited.into_iter().flatten().filter(|v| *v).count() + 1;
    println!("{}", visit_count);
}

fn part_two(obstacles: &Vec<Vec<bool>>, start_pos: &(usize, usize)) {
    let n = obstacles.len();
    let m = obstacles[0].len();
    let mut obstacles_in_row = vec![BTreeSet::new(); n];
    let mut obstacles_in_col = vec![BTreeSet::new(); m];

    for row in 0..n {
        for col in 0..m {
            if !obstacles[row][col] {
                continue;
            }

            obstacles_in_row[row].insert(col);
            obstacles_in_col[col].insert(row);
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Direction {
        Up,
        Right,
        Down,
        Left,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Position {
        row: usize,
        col: usize,
        dir: Direction,
    }

    let compute_next_pos = |pos: Position,
                            obstacles_in_row: &Vec<BTreeSet<usize>>,
                            obstacles_in_col: &Vec<BTreeSet<usize>>|
     -> Position {
        let Position { row, col, dir } = pos;
        match dir {
            Direction::Up => {
                let row = obstacles_in_col[col]
                    .range(..row)
                    .rev()
                    .next()
                    .map(|row| row + 1)
                    .unwrap_or(0);
                Position {
                    row,
                    col,
                    dir: Direction::Right,
                }
            }
            Direction::Right => {
                let col = obstacles_in_row[row]
                    .range(col + 1..)
                    .next()
                    .map(|col| col - 1)
                    .unwrap_or(m - 1);
                Position {
                    row,
                    col,
                    dir: Direction::Down,
                }
            }
            Direction::Down => {
                let row = obstacles_in_col[col]
                    .range(row + 1..)
                    .next()
                    .map(|row| row - 1)
                    .unwrap_or(n - 1);
                Position {
                    row,
                    col,
                    dir: Direction::Left,
                }
            }
            Direction::Left => {
                let col = obstacles_in_row[row]
                    .range(..col)
                    .rev()
                    .next()
                    .map(|col| col + 1)
                    .unwrap_or(0);
                Position {
                    row,
                    col,
                    dir: Direction::Up,
                }
            }
        }
    };

    let mut sum = 0;
    for row in 0..n {
        for col in 0..m {
            if obstacles[row][col] {
                continue;
            }

            obstacles_in_row[row].insert(col);
            obstacles_in_col[col].insert(row);

            let has_loop = 'hl: {
                let mut path = Vec::new();
                let mut visited = HashSet::new();
                let mut pos = Position {
                    row: start_pos.0,
                    col: start_pos.1,
                    dir: Direction::Up,
                };

                while 0 < pos.row && pos.row < n - 1 && 0 < pos.col && pos.col < m - 1 {
                    path.push(pos);
                    if !visited.insert(pos) {
                        break 'hl true;
                    }
                    pos = compute_next_pos(pos, &obstacles_in_row, &obstacles_in_col);
                }

                false
            };

            if has_loop {
                sum += 1;
            }

            obstacles_in_row[row].remove(&col);
            obstacles_in_col[col].remove(&row);
        }
    }

    println!("{}", sum);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 6)?;

    let mut obstacles = Vec::new();
    let mut start_pos = (0, 0);

    for line in input.lines() {
        let mut row = Vec::new();
        for byte in line.bytes() {
            if byte == b'^' {
                start_pos = (obstacles.len(), row.len());
            }

            row.push(match byte {
                b'.' | b'^' => false,
                b'#' => true,
                _ => unreachable!(),
            });
        }
        obstacles.push(row);
    }

    part_one(&obstacles, &start_pos);
    part_two(&obstacles, &start_pos);

    Ok(())
}
