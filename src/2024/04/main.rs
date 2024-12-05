fn part_one(puzzle: &Vec<Vec<u8>>) {
    let n = puzzle.len();
    let m = puzzle[0].len();

    let mut appearances = 0;

    let is_valid_index = |row: isize, col: isize| -> bool {
        (0..n as isize).contains(&row) && (0..m as isize).contains(&col)
    };

    let is_xmas_in_direction = |row: isize, col: isize, step_row: isize, step_col: isize| -> bool {
        let x = (row, col);
        let m = (row + step_row, col + step_col);
        let a = (row + step_row * 2, col + step_col * 2);
        let s = (row + step_row * 3, col + step_col * 3);

        if !is_valid_index(x.0, x.1)
            || !is_valid_index(m.0, m.1)
            || !is_valid_index(a.0, a.1)
            || !is_valid_index(s.0, s.1)
        {
            return false;
        }

        if puzzle[x.0 as usize][x.1 as usize] != b'X'
            || puzzle[m.0 as usize][m.1 as usize] != b'M'
            || puzzle[a.0 as usize][a.1 as usize] != b'A'
            || puzzle[s.0 as usize][s.1 as usize] != b'S'
        {
            return false;
        }

        true
    };

    for row in 0..n {
        for col in 0..m {
            let row = row as isize;
            let col = col as isize;

            // lol
            if is_xmas_in_direction(row, col, 1, 0) {
                appearances += 1;
            }
            if is_xmas_in_direction(row, col, -1, 0) {
                appearances += 1;
            }
            if is_xmas_in_direction(row, col, 0, 1) {
                appearances += 1;
            }
            if is_xmas_in_direction(row, col, 0, -1) {
                appearances += 1;
            }
            if is_xmas_in_direction(row, col, 1, 1) {
                appearances += 1;
            }
            if is_xmas_in_direction(row, col, -1, 1) {
                appearances += 1;
            }
            if is_xmas_in_direction(row, col, 1, -1) {
                appearances += 1;
            }
            if is_xmas_in_direction(row, col, -1, -1) {
                appearances += 1;
            }
        }
    }

    println!("{}", appearances);
}

fn part_two(puzzle: &Vec<Vec<u8>>) {
    let n = puzzle.len();
    let m = puzzle[0].len();

    let mut appearances = 0;

    let is_xmas_center = |row: usize, col: usize| -> bool {
        if puzzle[row][col] != b'A' {
            return false;
        }

        let diag1 = (puzzle[row - 1][col - 1] == b'M' && puzzle[row + 1][col + 1] == b'S')
            || (puzzle[row + 1][col + 1] == b'M' && puzzle[row - 1][col - 1] == b'S');
        let diag2 = (puzzle[row - 1][col + 1] == b'M' && puzzle[row + 1][col - 1] == b'S')
            || (puzzle[row + 1][col - 1] == b'M' && puzzle[row - 1][col + 1] == b'S');
        diag1 && diag2
    };

    for row in 1..n - 1 {
        for col in 1..m - 1 {
            if is_xmas_center(row, col) {
                appearances += 1;
            }
        }
    }

    println!("{}", appearances);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 4)?;

    let puzzle: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

    part_one(&puzzle);
    part_two(&puzzle);

    Ok(())
}
