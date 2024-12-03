fn is_safe(levels: &Vec<i32>) -> bool {
    let n = levels.len();

    // Check if all increasing or decreasing
    for i in 2..n {
        let curr_diff = levels[i] - levels[i - 1];
        let prev_diff = levels[i - 1] - levels[i - 2];
        if curr_diff.signum() != prev_diff.signum() {
            return false;
        }
    }

    // Check if adjacents differ by [1, 3]
    for i in 1..n {
        let abs_diff = levels[i].abs_diff(levels[i - 1]);
        if abs_diff < 1 || abs_diff > 3 {
            return false;
        }
    }

    true
}

fn part_one(reports: &Vec<Vec<i32>>) {
    let mut safe_reports = 0;
    for levels in reports.iter() {
        if is_safe(levels) {
            safe_reports += 1;
        }
    }

    println!("{}", safe_reports);
}

fn part_two(reports: &Vec<Vec<i32>>) {
    let mut safe_reports = 0;
    for levels in reports.iter() {
        if is_safe(levels) {
            safe_reports += 1;
            continue;
        }

        let n = levels.len();

        let mut safe = false;
        let mut try_remove = |idx: usize| {
            let mut levels = levels.clone();
            levels.remove(idx);
            if is_safe(&levels) {
                safe = true;
            }
        };

        // Check if all increasing or decreasing after removing an index
        for i in 2..n {
            let curr_diff = levels[i] - levels[i - 1];
            let prev_diff = levels[i - 1] - levels[i - 2];
            if curr_diff.signum() != prev_diff.signum() {
                try_remove(i);
                try_remove(i - 1);
                try_remove(i - 2);
                break;
            }
        }

        // Check if adjacents differ by [1, 3] after removing an index
        for i in 1..n {
            let abs_diff = levels[i].abs_diff(levels[i - 1]);
            if abs_diff < 1 || abs_diff > 3 {
                try_remove(i);
                try_remove(i - 1);
                break;
            }
        }

        if safe {
            safe_reports += 1;
        }
    }

    println!("{}", safe_reports);
}

fn main() -> std::io::Result<()> {
    let input = aoc_rs::read_input(2024, 2)?;

    let mut reports = Vec::new();
    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        reports.push(levels);
    }

    part_one(&reports);
    part_two(&reports);

    Ok(())
}
