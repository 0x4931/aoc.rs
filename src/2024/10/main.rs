fn part_one(map: &Vec<Vec<i32>>) {
    let n = map.len();
    let m = map[0].len();

    let mut sum = 0;

    for row in 0..n {
        for col in 0..m {
            if map[row][col] != 0 {
                continue;
            }

            let mut score = 0;
            let mut visited = vec![vec![false; m]; n];
            let mut visiting = vec![];
            visiting.push((row, col));

            while !visiting.is_empty() {
                let (row, col) = visiting.pop().unwrap();
                visited[row][col] = true;

                let height = map[row][col];
                let row = row as isize;
                let col = col as isize;

                if height == 9 {
                    score += 1;
                    continue;
                }

                let is_valid_idx = |row: isize, col: isize| {
                    (0..n as isize).contains(&row)
                        && (0..m as isize).contains(&col)
                        && !visited[row as usize][col as usize]
                        && map[row as usize][col as usize] == height + 1
                };

                if is_valid_idx(row + 1, col) {
                    visiting.push((row as usize + 1, col as usize));
                }
                if is_valid_idx(row - 1, col) {
                    visiting.push((row as usize - 1, col as usize));
                }
                if is_valid_idx(row, col + 1) {
                    visiting.push((row as usize, col as usize + 1));
                }
                if is_valid_idx(row, col - 1) {
                    visiting.push((row as usize, col as usize - 1));
                }
            }

            sum += score;
        }
    }

    println!("{}", sum);
}

fn part_two(map: &Vec<Vec<i32>>) {
    let n = map.len();
    let m = map[0].len();

    let mut sum = 0;

    for row in 0..n {
        for col in 0..m {
            if map[row][col] != 0 {
                continue;
            }

            // literally the exact same code but without the visited flag... lmao

            let mut score = 0;
            let mut visiting = vec![];
            visiting.push((row, col));

            while !visiting.is_empty() {
                let (row, col) = visiting.pop().unwrap();

                let height = map[row][col];
                let row = row as isize;
                let col = col as isize;

                if height == 9 {
                    score += 1;
                    continue;
                }

                let is_valid_idx = |row: isize, col: isize| {
                    (0..n as isize).contains(&row)
                        && (0..m as isize).contains(&col)
                        && map[row as usize][col as usize] == height + 1
                };

                if is_valid_idx(row + 1, col) {
                    visiting.push((row as usize + 1, col as usize));
                }
                if is_valid_idx(row - 1, col) {
                    visiting.push((row as usize - 1, col as usize));
                }
                if is_valid_idx(row, col + 1) {
                    visiting.push((row as usize, col as usize + 1));
                }
                if is_valid_idx(row, col - 1) {
                    visiting.push((row as usize, col as usize - 1));
                }
            }

            sum += score;
        }
    }

    println!("{}", sum);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 10)?;

    let mut map = Vec::new();
    for line in input.lines() {
        let row = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        map.push(row);
    }

    part_one(&map);
    part_two(&map);

    Ok(())
}
