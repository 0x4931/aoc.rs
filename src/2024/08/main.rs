use std::collections::{HashMap, HashSet};

fn get_antennas(map: &Vec<Vec<u8>>) -> HashMap<u8, Vec<(usize, usize)>> {
    let n = map.len();
    let m = map[0].len();
    let mut antennas = HashMap::new();

    for row in 0..n {
        for col in 0..m {
            if map[row][col] == b'.' {
                continue;
            }

            antennas
                .entry(map[row][col])
                .or_insert_with(|| Vec::new())
                .push((row, col));
        }
    }

    antennas
}

fn part_one(map: &Vec<Vec<u8>>) {
    let n = map.len();
    let m = map[0].len();
    let antennas = get_antennas(map);

    let mut antinodes = HashSet::new();
    for positions in antennas.values() {
        let k = positions.len();
        for i in 0..k {
            let row1 = positions[i].0 as isize;
            let col1 = positions[i].1 as isize;

            for j in i + 1..k {
                let row2 = positions[j].0 as isize;
                let col2 = positions[j].1 as isize;

                let row_diff = row2 - row1;
                let col_diff = col2 - col1;

                let antinode1 = (row2 + row_diff, col2 + col_diff);
                let antinode2 = (row1 - row_diff, col1 - col_diff);

                let is_valid_pos = |pos: (isize, isize)| {
                    (0..n as isize).contains(&pos.0) && (0..m as isize).contains(&pos.1)
                };

                if is_valid_pos(antinode1) {
                    antinodes.insert(antinode1);
                }
                if is_valid_pos(antinode2) {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    let unique_antinodes = antinodes.len();
    println!("{}", unique_antinodes);
}

fn part_two(map: &Vec<Vec<u8>>) {
    let n = map.len();
    let m = map[0].len();
    let antennas = get_antennas(map);

    let mut antinodes = HashSet::new();
    for positions in antennas.values() {
        let k = positions.len();
        for i in 0..k {
            let row1 = positions[i].0 as isize;
            let col1 = positions[i].1 as isize;

            for j in i + 1..k {
                let row2 = positions[j].0 as isize;
                let col2 = positions[j].1 as isize;

                let row_diff = row2 - row1;
                let col_diff = col2 - col1;

                let is_valid_pos = |pos: (isize, isize)| {
                    (0..n as isize).contains(&pos.0) && (0..m as isize).contains(&pos.1)
                };

                let mut antinode1 = (row2, col2);
                let mut antinode2 = (row1, col1);

                while is_valid_pos(antinode1) {
                    antinodes.insert(antinode1);
                    antinode1.0 += row_diff;
                    antinode1.1 += col_diff;
                }

                while is_valid_pos(antinode2) {
                    antinodes.insert(antinode2);
                    antinode2.0 -= row_diff;
                    antinode2.1 -= col_diff;
                }
            }
        }
    }

    let unique_antinodes = antinodes.len();
    println!("{}", unique_antinodes);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 8)?;

    let mut map = Vec::new();
    for line in input.lines() {
        let row: Vec<_> = line.bytes().collect();
        map.push(row);
    }

    part_one(&map);
    part_two(&map);

    Ok(())
}
