#[derive(Debug)]
struct Calibration {
    result: i64,
    values: Vec<i64>,
}

fn count_digits(mut num: i64) -> u32 {
    let mut count = 0;
    while num > 0 {
        count += 1;
        num /= 10;
    }
    count.max(1)
}

fn test_calibration_recursive(
    calibration: &Calibration,
    result: i64,
    value_idx: usize,
    use_concat_op: bool,
) -> bool {
    let value = calibration.values[value_idx];
    if value_idx == 0 {
        return result == value;
    }

    let mut is_possible = false;

    if result % value == 0 {
        is_possible |=
            test_calibration_recursive(calibration, result / value, value_idx - 1, use_concat_op);
    }

    // Pretty sure calibration results are never negative
    if result >= value {
        is_possible |=
            test_calibration_recursive(calibration, result - value, value_idx - 1, use_concat_op);
    }

    if use_concat_op {
        let value_digits = 10i64.pow(count_digits(value));
        if result % value_digits == value {
            is_possible |= test_calibration_recursive(
                calibration,
                result / value_digits,
                value_idx - 1,
                use_concat_op,
            );
        }
    }

    is_possible
}

fn test_calibration(calibration: &Calibration, use_concat_op: bool) -> bool {
    test_calibration_recursive(
        calibration,
        calibration.result,
        calibration.values.len() - 1,
        use_concat_op,
    )
}

fn part_one(calibrations: &Vec<Calibration>) {
    let mut sum = 0;
    for calibration in calibrations.iter() {
        if test_calibration(calibration, false) {
            sum += calibration.result;
        }
    }

    println!("{}", sum);
}

fn part_two(calibrations: &Vec<Calibration>) {
    let mut sum = 0;
    for calibration in calibrations.iter() {
        if test_calibration(calibration, true) {
            sum += calibration.result;
        }
    }

    println!("{}", sum);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 7)?;

    let mut calibrations = Vec::new();

    for line in input.lines() {
        let (result_info, values_info) = line.split_once(":").unwrap();
        let result = result_info.parse().unwrap();
        let values = values_info
            .split_whitespace()
            .map(|value| value.parse().unwrap())
            .collect();

        calibrations.push(Calibration { result, values });
    }

    part_one(&calibrations);
    part_two(&calibrations);

    Ok(())
}
