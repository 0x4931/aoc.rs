fn part_one(document: &Vec<String>) {
    let mut calibration_sum = 0;
    for line in document.iter() {
        let first_digit: i32 = line
            .matches(|c| char::is_digit(c, 10))
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let last_digit: i32 = line
            .rmatches(|c| char::is_digit(c, 10))
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let calibration_val = 10 * first_digit + last_digit;
        calibration_sum += calibration_val;
    }

    println!("{}", calibration_sum);
}

fn part_two(document: &Vec<String>) {
    let digits = [
        (1, "1"),
        (1, "one"),
        (2, "2"),
        (2, "two"),
        (3, "3"),
        (3, "three"),
        (4, "4"),
        (4, "four"),
        (5, "5"),
        (5, "five"),
        (6, "6"),
        (6, "six"),
        (7, "7"),
        (7, "seven"),
        (8, "8"),
        (8, "eight"),
        (9, "9"),
        (9, "nine"),
    ];

    let mut calibration_sum = 0;
    for line in document.iter() {
        let first_digit = digits
            .iter()
            .filter_map(|(value, pattern)| line.find(pattern).map(|idx| (idx, *value)))
            .min_by_key(|(idx, _)| *idx)
            .unwrap()
            .1;
        let last_digit = digits
            .iter()
            .filter_map(|(value, pattern)| line.rfind(pattern).map(|idx| (idx, *value)))
            .max_by_key(|(idx, _)| *idx)
            .unwrap()
            .1;
        let calibration_val = 10 * first_digit + last_digit;
        calibration_sum += calibration_val;
    }

    println!("{}", calibration_sum);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2023, 1)?;

    let document: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    part_one(&document);
    part_two(&document);

    Ok(())
}
