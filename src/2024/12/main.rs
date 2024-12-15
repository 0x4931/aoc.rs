fn part_one(map: &Vec<Vec<char>>) {
    let n = map.len();
    let m = map[0].len();
    let mut scanned = vec![vec![false; m]; n];
    let mut sum = 0;

    for row in 0..n {
        for col in 0..m {
            if scanned[row][col] {
                continue;
            }

            let mut to_scan = Vec::new();
            to_scan.push((row, col));
            scanned[row][col] = true;

            let mut area = 0;
            let mut perimeter = 0;

            while !to_scan.is_empty() {
                let (row, col) = to_scan.pop().unwrap();

                let is_valid_idx = |r: isize, c: isize| {
                    (0..n as isize).contains(&r)
                        && (0..m as isize).contains(&c)
                        && map[r as usize][c as usize] == map[row][col]
                };

                let row = row as isize;
                let col = col as isize;

                area += 1;

                let mut try_scan = |r: isize, c: isize| {
                    if is_valid_idx(r, c) {
                        let r = r as usize;
                        let c = c as usize;

                        if scanned[r][c] {
                            return;
                        }

                        to_scan.push((r, c));
                        scanned[r][c] = true;
                    } else {
                        perimeter += 1;
                    }
                };

                try_scan(row - 1, col);
                try_scan(row + 1, col);
                try_scan(row, col - 1);
                try_scan(row, col + 1);
            }

            sum += area * perimeter;
        }
    }

    println!("{}", sum);
}

fn part_two(map: &Vec<Vec<char>>) {
    let n = map.len();
    let m = map[0].len();
    let mut scanned = vec![vec![false; m]; n];
    let mut sum = 0;

    for row in 0..n {
        for col in 0..m {
            if scanned[row][col] {
                continue;
            }

            let mut to_scan = Vec::new();
            to_scan.push((row, col));
            scanned[row][col] = true;

            let mut area = 0;
            let mut perimeter = 0;

            while !to_scan.is_empty() {
                let (row, col) = to_scan.pop().unwrap();

                let is_valid_idx = |r: isize, c: isize| {
                    (0..n as isize).contains(&r)
                        && (0..m as isize).contains(&c)
                        && map[r as usize][c as usize] == map[row][col]
                };

                let row = row as isize;
                let col = col as isize;

                area += 1;

                // Number of sides = number of vertices (corners)

                if !is_valid_idx(row - 1, col) && !is_valid_idx(row, col - 1) {
                    perimeter += 1; // top left corner
                }

                if !is_valid_idx(row - 1, col) && !is_valid_idx(row, col + 1) {
                    perimeter += 1; // top right corner
                }

                if !is_valid_idx(row + 1, col) && !is_valid_idx(row, col - 1) {
                    perimeter += 1; // bottom left corner
                }

                if !is_valid_idx(row + 1, col) && !is_valid_idx(row, col + 1) {
                    perimeter += 1; // bottom right corner
                }

                if is_valid_idx(row - 1, col)
                    && is_valid_idx(row, col - 1)
                    && !is_valid_idx(row - 1, col - 1)
                {
                    perimeter += 1; // top left corner (concave)
                }

                if is_valid_idx(row - 1, col)
                    && is_valid_idx(row, col + 1)
                    && !is_valid_idx(row - 1, col + 1)
                {
                    perimeter += 1; // top right corner (concave)
                }

                if is_valid_idx(row + 1, col)
                    && is_valid_idx(row, col - 1)
                    && !is_valid_idx(row + 1, col - 1)
                {
                    perimeter += 1; // bottom left corner (concave)
                }

                if is_valid_idx(row + 1, col)
                    && is_valid_idx(row, col + 1)
                    && !is_valid_idx(row + 1, col + 1)
                {
                    perimeter += 1; // bottom right corner (concave)
                }

                let mut try_scan = |r: isize, c: isize| {
                    if is_valid_idx(r, c) {
                        let r = r as usize;
                        let c = c as usize;

                        if scanned[r][c] {
                            return;
                        }

                        to_scan.push((r, c));
                        scanned[r][c] = true;
                    }
                };

                try_scan(row - 1, col);
                try_scan(row + 1, col);
                try_scan(row, col - 1);
                try_scan(row, col + 1);
            }

            sum += area * perimeter;
        }
    }

    println!("{}", sum);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 12)?;

    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    part_one(&map);
    part_two(&map);

    Ok(())
}
