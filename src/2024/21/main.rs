use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

fn key_to_num(key: char) -> usize {
    match key {
        '^' => 0,
        '<' => 1,
        'v' => 2,
        '>' => 3,
        'A' => 4,
        _ => unreachable!(),
    }
}

fn key_from_num(num: usize) -> char {
    match num {
        0 => '^',
        1 => '<',
        2 => 'v',
        3 => '>',
        4 => 'A',
        _ => unreachable!(),
    }
}

#[deprecated]
fn key_to_coord(key: char) -> (isize, isize) {
    match key {
        '^' => (0, 1),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        'A' => (0, 2),
        _ => unreachable!(),
    }
}

fn compute_keypress_costs(robots: usize) -> Vec<Vec<Vec<i64>>> {
    // Intuition: Dynamic Programming, with Dijkstra's baked into each calculation step

    // Generalization:

    // Press(N, X, Y) = Cost of moving Robot N from key X to Y and pressing Y

    // Press(0, X, Y) = 1 (Human has 0 cost moves)

    // Press(N, X, Y) = Find directional path I-J-K for previous robot minimizing:
    //     Press(N-1, A, I) + Press(N-1, I, J) + Press(N-1, J, K) + Press(N-1, K, A)

    let dirpad = [
        [' ', '^', 'A'], // anti autoformat comment
        ['<', 'v', '>'],
    ];
    let n = dirpad.len();
    let m = dirpad[0].len();

    let mut dp = vec![vec![vec![0; 5]; 5]; robots + 1];

    for key1 in 0..5 {
        for key2 in 0..5 {
            dp[0][key1][key2] = 1;
        }
    }

    for robot in 1..=robots {
        for key1 in 0..5 {
            for key2 in 0..5 {
                let (key1_coord, key2_coord) = {
                    let (key1_char, key2_char) = (key_from_num(key1), key_from_num(key2));
                    let mut key1_coord = (0, 0);
                    let mut key2_coord = (0, 0);
                    for (row, keys) in dirpad.iter().enumerate() {
                        for (col, key) in keys.iter().enumerate() {
                            let coord = (row as isize, col as isize);
                            if *key == key1_char {
                                key1_coord = coord;
                            }
                            if *key == key2_char {
                                key2_coord = coord;
                            }
                        }
                    }
                    (key1_coord, key2_coord)
                };

                let mut min_costs = HashMap::new();
                min_costs.insert((key1_coord, 'A'), 0);

                let mut dijkstra = BinaryHeap::new();
                dijkstra.push(Reverse((0, (key1_coord, 'A'))));

                while !dijkstra.is_empty() {
                    let (cost, state) = dijkstra.pop().unwrap().0;
                    let (coord, prev_key_char) = state;
                    let prev_key = key_to_num(prev_key_char);

                    for dir_key_char in ['^', '<', 'v', '>'] {
                        let dir_key = key_to_num(dir_key_char);
                        let dir = match dir_key_char {
                            '^' => (-1, 0),
                            '<' => (0, -1),
                            'v' => (1, 0),
                            '>' => (0, 1),
                            _ => unreachable!(),
                        };

                        let next_coord = (coord.0 + dir.0, coord.1 + dir.1);
                        let next_row = next_coord.0 as usize;
                        let next_col = next_coord.1 as usize;

                        if (0..n).contains(&next_row)
                            && (0..m).contains(&next_col)
                            && dirpad[next_row][next_col] != ' '
                        {
                            let next_state = if next_coord != key2_coord {
                                (next_coord, dir_key_char)
                            } else {
                                (next_coord, 'A')
                            };

                            let next_cost = {
                                let mut next_cost = cost + dp[robot - 1][prev_key][dir_key];
                                if next_coord == key2_coord {
                                    next_cost += dp[robot - 1][dir_key][key_to_num('A')]
                                }
                                next_cost
                            };

                            let min_cost = min_costs.entry(next_state).or_insert(i64::MAX);
                            if next_cost < *min_cost {
                                *min_cost = next_cost;
                                dijkstra.push(Reverse((next_cost, next_state)));
                            }
                        }
                    }
                }

                let min_cost = if key1 != key2 {
                    *min_costs.get(&(key2_coord, 'A')).expect("path exists")
                } else {
                    1
                };

                dp[robot][key1][key2] = min_cost;
            }
        }
    }

    dp
}

fn compute_code_cost(code: &String, dp: &Vec<Vec<Vec<i64>>>) -> i64 {
    let numpad = [
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        [' ', '0', 'A'],
    ];
    let n = numpad.len();
    let m = numpad[0].len();
    let robots = dp.len() - 1;

    let numeric: i64 = code.trim_end_matches('A').parse().unwrap();
    let mut sequence: i64 = 0;

    let code = "A".to_string() + code;
    let code: Vec<_> = code.chars().collect();

    for keys in code.windows(2) {
        let (key1, key2) = (keys[0], keys[1]);
        let (key1_coord, key2_coord) = {
            let mut key1_coord = (0, 0);
            let mut key2_coord = (0, 0);
            for (row, keys) in numpad.iter().enumerate() {
                for (col, key) in keys.iter().enumerate() {
                    let coord = (row as isize, col as isize);
                    if *key == key1 {
                        key1_coord = coord;
                    }
                    if *key == key2 {
                        key2_coord = coord;
                    }
                }
            }
            (key1_coord, key2_coord)
        };

        let mut min_costs = HashMap::new();
        min_costs.insert((key1_coord, 'A'), 0);

        let mut dijkstra = BinaryHeap::new();
        dijkstra.push(Reverse((0, (key1_coord, 'A'))));

        while !dijkstra.is_empty() {
            let (cost, state) = dijkstra.pop().unwrap().0;
            let (coord, prev_key_char) = state;
            let prev_key = key_to_num(prev_key_char);

            for dir_key_char in ['^', '<', 'v', '>'] {
                let dir_key = key_to_num(dir_key_char);
                let dir = match dir_key_char {
                    '^' => (-1, 0),
                    '<' => (0, -1),
                    'v' => (1, 0),
                    '>' => (0, 1),
                    _ => unreachable!(),
                };

                let next_coord = (coord.0 + dir.0, coord.1 + dir.1);
                let next_row = next_coord.0 as usize;
                let next_col = next_coord.1 as usize;

                if (0..n).contains(&next_row)
                    && (0..m).contains(&next_col)
                    && numpad[next_row][next_col] != ' '
                {
                    let next_state = if next_coord != key2_coord {
                        (next_coord, dir_key_char)
                    } else {
                        (next_coord, 'A')
                    };

                    let next_cost = {
                        let mut next_cost = cost + dp[robots][prev_key][dir_key];
                        if next_coord == key2_coord {
                            next_cost += dp[robots][dir_key][key_to_num('A')]
                        }
                        next_cost
                    };

                    let min_cost = min_costs.entry(next_state).or_insert(i64::MAX);
                    if next_cost < *min_cost {
                        *min_cost = next_cost;
                        dijkstra.push(Reverse((next_cost, next_state)));
                    }
                }
            }
        }

        let min_cost = if key1 != key2 {
            *min_costs.get(&(key2_coord, 'A')).expect("path exists")
        } else {
            1
        };

        sequence += min_cost;
    }

    sequence * numeric
}

fn part_one(codes: &Vec<String>) {
    let dp = compute_keypress_costs(2);
    let sum: i64 = codes.iter().map(|code| compute_code_cost(code, &dp)).sum();

    println!("{}", sum);
}

fn part_two(codes: &Vec<String>) {
    let dp = compute_keypress_costs(25);
    let sum: i64 = codes.iter().map(|code| compute_code_cost(code, &dp)).sum();

    println!("{}", sum);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 21)?;

    let codes: Vec<String> = input.lines().map(str::to_string).collect();

    part_one(&codes);
    part_two(&codes);

    Ok(())
}
