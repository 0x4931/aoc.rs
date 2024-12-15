use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Galaxy,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    row: usize,
    col: usize,
}

fn solve(map: &Vec<Vec<Tile>>, expansion_ratio: i64) -> i64 {
    let n = map.len();
    let m = map[0].len();
    let mut empty_rows: BTreeSet<_> = (0..n).collect();
    let mut empty_cols: BTreeSet<_> = (0..m).collect();
    let mut galaxies = Vec::new();

    for row in 0..n {
        for col in 0..m {
            if map[row][col] == Tile::Galaxy {
                empty_rows.remove(&row);
                empty_cols.remove(&col);
                galaxies.push(Coord { row, col });
            }
        }
    }

    let empty_rows = Vec::from_iter(empty_rows);
    let empty_cols = Vec::from_iter(empty_cols);
    let k = galaxies.len();
    let mut sum = 0;

    for i in 0..k {
        let galaxy_i = galaxies[i];

        for j in 0..i {
            let galaxy_j = galaxies[j];

            let row1 = galaxy_i.row.min(galaxy_j.row);
            let row2 = galaxy_i.row.max(galaxy_j.row);
            let col1 = galaxy_i.col.min(galaxy_j.col);
            let col2 = galaxy_i.col.max(galaxy_j.col);

            let empty_rows_within = empty_rows.binary_search(&row2).unwrap_err()
                - empty_rows.binary_search(&row1).unwrap_err();
            let empty_cols_within = empty_cols.binary_search(&col2).unwrap_err()
                - empty_cols.binary_search(&col1).unwrap_err();

            let rows = (row2 - row1) as i64;
            let cols = (col2 - col1) as i64;
            let expanded_rows = empty_rows_within as i64 * (expansion_ratio - 1);
            let expanded_cols = empty_cols_within as i64 * (expansion_ratio - 1);

            sum += rows + expanded_rows + cols + expanded_cols;
        }
    }

    sum
}

fn part_one(map: &Vec<Vec<Tile>>) {
    println!("{}", solve(&map, 2));
}

fn part_two(map: &Vec<Vec<Tile>>) {
    println!("{}", solve(&map, 1000000));
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2023, 11)?;

    let mut map: Vec<Vec<Tile>> = Default::default();
    for line in input.lines() {
        let tiles = line
            .chars()
            .map(|c| match c {
                '#' => Tile::Galaxy,
                '.' => Tile::Empty,
                _ => unreachable!(),
            })
            .collect();
        map.push(tiles);
    }

    part_one(&map);
    part_two(&map);

    Ok(())
}
