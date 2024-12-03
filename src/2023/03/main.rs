use std::collections::{HashMap, HashSet};

fn part_one(schematic: &Vec<Vec<u8>>) {
    let n = schematic.len();
    let m = schematic[0].len();

    let mut symbols = HashSet::new();
    let mut sum = 0;

    for row in 0..n {
        for col in 0..m {
            let c = schematic[row][col];
            if c != b'.' && !c.is_ascii_digit() {
                symbols.insert((row, col));
            }
        }
    }

    for row in 0..n {
        for col in 0..m {
            let c = schematic[row][col];
            let c_prev = if col > 0 {
                schematic[row][col - 1]
            } else {
                b'.'
            };

            if c.is_ascii_digit() && !c_prev.is_ascii_digit() {
                let mut col_end = col;
                while col_end < m && schematic[row][col_end].is_ascii_digit() {
                    col_end += 1;
                }

                let num_str = String::from_utf8(schematic[row][col..col_end].to_owned()).unwrap();
                let num: i32 = num_str.parse().unwrap();

                let adjacent_symbol = 'adj: {
                    let row = row as isize;
                    let col = col as isize;
                    let col_end = col_end as isize;
                    for i in row - 1..=row + 1 {
                        for j in col - 1..=col_end {
                            if i >= 0 && j >= 0 && symbols.contains(&(i as usize, j as usize)) {
                                break 'adj true;
                            }
                        }
                    }
                    false
                };

                if adjacent_symbol {
                    sum += num;
                }
            }
        }
    }

    println!("{}", sum);
}

fn part_two(schematic: &Vec<Vec<u8>>) {
    let n = schematic.len();
    let m = schematic[0].len();

    type Coord = (usize, usize);
    type Parts = (usize, i32); // number of adjacent parts numbers, gear ratio

    let mut gears: HashMap<Coord, Parts> = HashMap::new();
    let mut sum = 0;

    for row in 0..n {
        for col in 0..m {
            let c = schematic[row][col];
            if c == b'*' {
                gears.insert((row, col), (0, 1));
            }
        }
    }

    for row in 0..n {
        for col in 0..m {
            let c = schematic[row][col];
            let c_prev = if col > 0 {
                schematic[row][col - 1]
            } else {
                b'.'
            };

            if c.is_ascii_digit() && !c_prev.is_ascii_digit() {
                let mut col_end = col;
                while col_end < m && schematic[row][col_end].is_ascii_digit() {
                    col_end += 1;
                }

                let num_str = String::from_utf8(schematic[row][col..col_end].to_owned()).unwrap();
                let num: i32 = num_str.parse().unwrap();

                let row = row as isize;
                let col = col as isize;
                let col_end = col_end as isize;
                for i in row - 1..=row + 1 {
                    for j in col - 1..=col_end {
                        if i < 0 || j < 0 {
                            continue;
                        }
                        if let Some((count, ratio)) = gears.get_mut(&(i as usize, j as usize)) {
                            *count += 1;
                            *ratio *= num;
                        }
                    }
                }
            }
        }
    }

    for (_, (count, ratio)) in gears.iter() {
        if *count == 2 {
            sum += *ratio;
        }
    }

    println!("{}", sum);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2023, 3)?;

    let schematic: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

    part_one(&schematic);
    part_two(&schematic);

    Ok(())
}
